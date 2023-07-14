use crate::app::App;
use std::path::PathBuf;

mod cb;
mod syn;

#[derive(clap::Parser)]
#[clap(name = "four-bar", version, author, about)]
pub(crate) struct Entry {
    /// Default to startup GUI then open file paths
    files: Vec<PathBuf>,
    #[clap(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(clap::Subcommand)]
enum Cmd {
    /// Startup GUI
    Ui {
        /// Open file path
        files: Vec<PathBuf>,
    },
    /// Synthesis function without GUI
    Syn(syn::Syn),
    /// Generate codebook
    #[clap(alias = "cb")]
    Codebook(cb::CbCfg),
}

impl Entry {
    pub(super) fn main() {
        let entry = <Self as clap::Parser>::parse_from(wild::args());
        match entry.cmd {
            None => native(entry.files),
            Some(Cmd::Ui { files }) => native(files),
            Some(Cmd::Syn(syn)) => syn::syn(syn),
            Some(Cmd::Codebook(cb)) => cb::codebook(cb),
        }
    }
}

fn native(files: Vec<PathBuf>) {
    let opt = {
        use image::ImageFormat::Png;
        const ICON: &[u8] = include_bytes!("../assets/favicon.png");
        let icon = image::load_from_memory_with_format(ICON, Png).unwrap();
        eframe::NativeOptions {
            icon_data: Some(eframe::IconData {
                width: icon.width(),
                height: icon.height(),
                rgba: icon.into_bytes(),
            }),
            ..Default::default()
        }
    };
    #[cfg(all(windows, not(debug_assertions)))]
    unsafe {
        winapi::um::wincon::FreeConsole();
    }
    eframe::run_native("Four-bar", opt, Box::new(|ctx| App::new(ctx, files))).unwrap();
}
