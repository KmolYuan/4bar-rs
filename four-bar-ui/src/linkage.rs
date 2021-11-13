use crate::as_values::as_values;
#[cfg(not(target_arch = "wasm32"))]
use crate::synthesis::Synthesis;
use eframe::egui::{plot::*, *};
use four_bar::{FourBar, Mechanism};
use ron::{from_str, to_string};
use serde::{Deserialize, Serialize};
use std::{
    f64::consts::{PI, TAU},
    sync::{Arc, Mutex},
};

macro_rules! unit {
    ($label:literal, $attr:expr, $inter:expr, $ui:ident) => {
        DragValue::new(&mut $attr)
            .prefix($label)
            .speed($inter)
            .ui($ui);
    };
}

macro_rules! link {
    ($label:literal, $attr:expr, $inter:expr, $ui:ident) => {
        DragValue::new(&mut $attr)
            .prefix($label)
            .clamp_range(0.0001..=9999.)
            .speed($inter)
            .ui($ui);
    };
}

macro_rules! angle {
    ($label:literal, $attr:expr, $ui:ident, $t:literal) => {
        $ui.horizontal(|ui| {
            let mut deg = $attr / PI * 180.;
            if DragValue::new(&mut deg)
                .prefix($label)
                .suffix(concat![" deg", $t])
                .clamp_range(0..=360)
                .speed(1.)
                .ui(ui)
                .changed()
            {
                $attr = deg / 180. * PI;
            }
            DragValue::new(&mut $attr)
                .suffix(concat![" rad", $t])
                .min_decimals(2)
                .clamp_range((0.)..=TAU)
                .speed(0.01)
                .ui(ui);
        });
    };
    ($label:literal, $attr:expr, $ui:ident) => {
        if TAU - $attr < 1e-20 {
            $attr = 0.;
        }
        angle!($label, $attr, $ui, "");
    };
}

macro_rules! draw_link {
    ($a:expr, $b:expr) => {
        Line::new(as_values(&[$a, $b]))
            .width(3.)
            .color(Color32::from_rgb(165, 151, 132))
    };
    ($a:expr, $b:expr $(, $c:expr)+) => {
        Polygon::new(as_values(&[$a, $b $(, $c)+]))
            .width(3.)
            .fill_alpha(0.6)
            .color(Color32::from_rgb(165, 151, 132))
    };
}

macro_rules! draw_path {
    ($name:literal, $path:expr) => {
        Line::new(as_values(&$path)).name($name).width(3.)
    };
}

/// Linkage data.
#[derive(Deserialize, Serialize)]
#[serde(default)]
pub(crate) struct Linkage {
    config: Config,
    driver: Driver,
    four_bar: Arc<Mutex<FourBar>>,
    #[cfg(not(target_arch = "wasm32"))]
    synthesis: Synthesis,
    #[cfg(target_arch = "wasm32")]
    #[serde(skip)]
    save_fn: js_sys::Function,
    #[cfg(target_arch = "wasm32")]
    #[serde(skip)]
    load_fn: js_sys::Function,
    #[cfg(target_arch = "wasm32")]
    #[serde(skip)]
    load_str: js_sys::Array,
}

impl Default for Linkage {
    fn default() -> Self {
        Self {
            config: Default::default(),
            driver: Default::default(),
            four_bar: Default::default(),
            #[cfg(not(target_arch = "wasm32"))]
            synthesis: Default::default(),
            #[cfg(target_arch = "wasm32")]
            save_fn: js_sys::Function::new_no_args(""),
            #[cfg(target_arch = "wasm32")]
            load_fn: js_sys::Function::new_no_args(""),
            #[cfg(target_arch = "wasm32")]
            load_str: js_sys::Array::new(),
        }
    }
}

impl PartialEq for Linkage {
    fn eq(&self, other: &Self) -> bool {
        self.driver == other.driver
            && *self.four_bar.lock().unwrap() == *other.four_bar.lock().unwrap()
    }
}

#[derive(Deserialize, Serialize, PartialEq)]
#[serde(default)]
struct Config {
    interval: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self { interval: 1. }
    }
}

#[derive(Deserialize, Serialize, Default, PartialEq)]
#[serde(default)]
struct Driver {
    drive: f64,
    speed: f64,
}

impl Linkage {
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn with_hook(save_fn: js_sys::Function, load_fn: js_sys::Function) -> Self {
        Self {
            save_fn,
            load_fn,
            ..Self::default()
        }
    }

    pub(crate) fn panel(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.heading("Save and Load");
            ui.horizontal(|ui| {
                if ui.button("💾 Save").clicked() {
                    let s = to_string(&*self.four_bar.lock().unwrap()).unwrap();
                    #[cfg(target_arch = "wasm32")]
                    {
                        use js_sys::JsString;
                        let this = wasm_bindgen::JsValue::NULL;
                        self.save_fn
                            .call2(&this, &JsString::from(s), &JsString::from("four_bar.ron"))
                            .unwrap();
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    if let Some(file_name) = rfd::FileDialog::new()
                        .set_file_name("four_bar.ron")
                        .add_filter("Rusty Object Notation", &["ron"])
                        .save_file()
                    {
                        std::fs::write(file_name, s).unwrap_or_default();
                    }
                }
                if ui.button("🖴 Load").clicked() {
                    #[cfg(target_arch = "wasm32")]
                    {
                        let this = wasm_bindgen::JsValue::NULL;
                        self.load_fn.call1(&this, &self.load_str).unwrap();
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let s = if let Some(file_name) = rfd::FileDialog::new()
                            .add_filter("Rusty Object Notation", &["ron"])
                            .pick_file()
                        {
                            std::fs::read_to_string(file_name).unwrap_or_default()
                        } else {
                            String::new()
                        };
                        if let Ok(four_bar) = from_str::<FourBar>(s.as_str()) {
                            *self.four_bar.lock().unwrap() = four_bar;
                        }
                    }
                }
            });
            ui.collapsing("Options", |ui| {
                reset_button(ui, &mut self.config);
                link!("Value interval: ", self.config.interval, 0.01, ui);
            });
        });
        #[cfg(target_arch = "wasm32")]
        if self.load_str.length() > 0 {
            use js_sys::{Array, JsString};
            let s = String::from(JsString::from(self.load_str.get(0)));
            if let Ok(four_bar) = from_str::<FourBar>(s.as_str()) {
                *self.four_bar.lock().unwrap() = four_bar;
            }
            self.load_str = Array::new();
        }
        ui.group(|ui| {
            ui.heading("Dimension");
            reset_button(ui, self);
            self.parameter(ui);
        });
        ui.group(|ui| {
            ui.heading("Driver");
            reset_button(ui, &mut self.driver);
            angle!("Speed: ", self.driver.speed, ui, "/s");
            angle!("Angle: ", self.driver.drive, ui);
        });
        #[cfg(not(target_arch = "wasm32"))]
        ui.group(|ui| {
            self.synthesis.update(ui, self.four_bar.clone());
        });
    }

    fn parameter(&mut self, ui: &mut Ui) {
        let interval = self.config.interval;
        let mut four_bar = self.four_bar.lock().unwrap();
        if ui.button("Normalize").clicked() {
            four_bar.reset();
            let l1 = four_bar.l1;
            *four_bar /= l1;
        }
        ui.group(|ui| {
            ui.heading("Offset");
            let not_default = (four_bar.p0.0, four_bar.p0.1, four_bar.a) != (0., 0., 0.);
            if ui.add_enabled(not_default, Button::new("Reset")).clicked() {
                four_bar.p0.0 = 0.;
                four_bar.p0.1 = 0.;
                four_bar.a = 0.;
            }
            unit!("X Offset: ", four_bar.p0.0, interval, ui);
            unit!("Y Offset: ", four_bar.p0.1, interval, ui);
            angle!("Rotation: ", four_bar.a, ui);
        });
        ui.group(|ui| {
            ui.heading("Parameters");
            link!("Ground: ", four_bar.l0, interval, ui);
            link!("Crank: ", four_bar.l1, interval, ui);
            link!("Coupler: ", four_bar.l2, interval, ui);
            link!("Follower: ", four_bar.l3, interval, ui);
            ui.checkbox(&mut four_bar.inv, "Invert follower and coupler");
        });
        ui.group(|ui| {
            ui.heading("Coupler");
            link!("Extended: ", four_bar.l4, interval, ui);
            angle!("Angle: ", four_bar.g, ui);
        });
    }

    pub(crate) fn plot(&mut self, ctx: &CtxRef) {
        CentralPanel::default().show(ctx, |ui| {
            let m = Mechanism::four_bar(self.four_bar.lock().unwrap().clone());
            let mut joints = [[0., 0.]; 5];
            m.apply(self.driver.drive, [0, 1, 2, 3, 4], &mut joints);
            let [path1, path2, path3] = m.four_bar_loop_all(0., 360);
            #[cfg_attr(target_arch = "wasm32", allow(unused_mut))]
            let mut plot = Plot::new("canvas")
                .line(draw_link![joints[0], joints[2]])
                .line(draw_link![joints[1], joints[3]])
                .polygon(draw_link![joints[2], joints[3], joints[4]])
                .points(
                    Points::new(as_values(&[joints[0], joints[1]]))
                        .radius(7.)
                        .color(Color32::from_rgb(93, 69, 56)),
                )
                .points(
                    Points::new(as_values(&[joints[2], joints[3], joints[4]]))
                        .radius(5.)
                        .color(Color32::from_rgb(128, 96, 77)),
                )
                .line(draw_path!("Crank pivot", path1))
                .line(draw_path!("Follower pivot", path2))
                .line(draw_path!("Coupler pivot", path3));
            #[cfg(not(target_arch = "wasm32"))]
            if !self.synthesis.curve.is_empty() {
                plot = plot.line(draw_path!("Synthesis target", self.synthesis.curve));
            }
            plot.data_aspect(1.).legend(Legend::default()).ui(ui);
            if self.driver.speed != 0. {
                self.driver.drive += self.driver.speed / 60.;
                ui.ctx().request_repaint();
            }
        });
    }
}
