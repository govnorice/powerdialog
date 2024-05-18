use gtk::{CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION, StyleContext};
use gtk::prelude::{CssProviderExt};

pub struct CssManager {
    provider: CssProvider,
    provider2: CssProvider,
}

impl CssManager {
    pub fn new() -> CssManager {
        CssManager {
            provider: CssProvider::new(),
            provider2: CssProvider::new(),
        }
    }
    pub fn load_from_file(&self, path: &str) {
        if let Err(err) = self.provider.load_from_path(path) {
            eprintln!("Error loading CSS from {}: {}", path, err);
        } else {
            StyleContext::add_provider_for_screen(&gdk::Screen::default().unwrap(), &self.provider, STYLE_PROVIDER_PRIORITY_APPLICATION);
            println!("CSS loaded successfully")
        }
    }
    pub fn load_from_data(&self, style: &[u8]) {
        if let Err(err) = self.provider2.load_from_data(style) {
            eprintln!("Error loading style from data: {}", err);
        } else {
            StyleContext::add_provider_for_screen(&gdk::Screen::default().unwrap(), &self.provider2, STYLE_PROVIDER_PRIORITY_APPLICATION);
            println!("Style from data loaded successfully")
        }
    }
}