use self::{linkages::*, syn::*, widgets::*};
use eframe::egui::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

mod as_values;
mod atomic;
mod csv;
mod io;
mod linkages;
mod proj;
mod syn;
mod widgets;

const RELEASE_URL: &str = concat![env!("CARGO_PKG_REPOSITORY"), "/releases/latest"];
const FONT: &[(&str, &[u8])] = &[
    ("Noto", include_bytes!("../../assets/GoNotoCurrent.ttf")),
    ("emoji", include_bytes!("../../assets/emoji-icon-font.ttf")),
];

#[derive(Default, Deserialize, Serialize, PartialEq)]
enum Panel {
    #[default]
    Linkages,
    Synthesis,
    Options,
    Off,
}

/// Main app state.
#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct App {
    welcome_off: bool,
    started: bool,
    linkage: Linkages,
    syn: Synthesis,
    #[serde(skip)]
    panel: Panel,
}

impl App {
    pub fn new(ctx: &eframe::CreationContext, files: Vec<String>) -> Box<Self> {
        let mut font_data = BTreeMap::new();
        let mut families = Vec::new();
        for &(name, font) in FONT {
            font_data.insert(name.to_string(), FontData::from_static(font));
            families.push(name.to_string());
        }
        let families = BTreeMap::from_iter([
            (FontFamily::Proportional, families.clone()),
            (FontFamily::Monospace, families),
        ]);
        ctx.egui_ctx
            .set_fonts(FontDefinitions { font_data, families });
        let mut style = (*ctx.egui_ctx.style()).clone();
        for (text_style, size) in [
            (TextStyle::Small, 18.),
            (TextStyle::Body, 24.),
            (TextStyle::Monospace, 24.),
            (TextStyle::Button, 30.),
            (TextStyle::Heading, 40.),
        ] {
            let id = FontId::proportional(size);
            style.text_styles.insert(text_style, id);
        }
        ctx.egui_ctx.set_style(style);
        let mut app = ctx
            .storage
            .and_then(|s| eframe::get_value::<Self>(s, eframe::APP_KEY))
            .unwrap_or_default();
        app.linkage.open_proj(files);
        Box::new(app)
    }

    fn welcome(&mut self, ctx: &Context) {
        let mut welcome = !self.welcome_off;
        Window::new("Welcome to Four🍀bar!")
            .open(&mut welcome)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.label(concat!["Version: v", env!("CARGO_PKG_VERSION")]);
                ui.label(env!("CARGO_PKG_DESCRIPTION"));
                ui.separator();
                ui.heading("Author");
                ui.label(env!("CARGO_PKG_AUTHORS"));
                ui.separator();
                ui.heading("License");
                ui.label("This software is under AGPL v3 license.");
                ui.label("The commercial usages under server or client side are not allowed.");
                ui.allocate_space(ui.available_size());
            });
        self.welcome_off = !welcome;
    }

    fn menu(&mut self, ui: &mut Ui) {
        ui.selectable_value(&mut self.panel, Panel::Linkages, "🍀")
            .on_hover_text("Linkages");
        ui.selectable_value(&mut self.panel, Panel::Synthesis, "💡")
            .on_hover_text("Synthesis");
        ui.selectable_value(&mut self.panel, Panel::Options, "🛠")
            .on_hover_text("Options");
        ui.selectable_value(&mut self.panel, Panel::Off, "⛶")
            .on_hover_text("Close Panel");
        ui.with_layout(Layout::right_to_left(), |ui| {
            let style = ui.style().clone();
            if let Some(v) = style.visuals.light_dark_small_toggle_button(ui) {
                ui.ctx().set_visuals(v);
            }
            url_btn(ui, "⮋", "Release", RELEASE_URL);
            url_btn(ui, "", "Repository", env!("CARGO_PKG_REPOSITORY"));
            if ui.small_button("💁").on_hover_text("Welcome").clicked() {
                self.welcome_off = !self.welcome_off;
            }
            ui.hyperlink_to("Powered by egui", "https://github.com/emilk/egui/");
        });
    }

    fn side_panel(ctx: &Context, f: impl FnOnce(&mut Ui)) {
        SidePanel::left("side panel")
            .resizable(false)
            .show(ctx, |ui| ScrollArea::vertical().show(ui, f));
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.welcome(ctx);
        TopBottomPanel::top("menu").show(ctx, |ui| ui.horizontal(|ui| self.menu(ui)));
        match self.panel {
            Panel::Linkages => Self::side_panel(ctx, |ui| self.linkage.show(ui)),
            Panel::Synthesis => Self::side_panel(ctx, |ui| self.syn.show(ui, &mut self.linkage)),
            Panel::Options => Self::side_panel(ctx, |ui| self.linkage.option(ui)),
            Panel::Off => (),
        }
        CentralPanel::default().show(ctx, |ui| {
            plot::Plot::new("canvas")
                .data_aspect(1.)
                .legend(Default::default())
                .coordinates_formatter(plot::Corner::LeftBottom, Default::default())
                .show(ui, |ui| {
                    self.linkage.plot(ui);
                    self.syn.plot(ui);
                });
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn persist_egui_memory(&self) -> bool {
        false
    }
}
