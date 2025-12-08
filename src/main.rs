use arboard::Clipboard;
use eframe::{NativeOptions, egui};
use egui::widgets::Label;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Endfind",
        NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(FindEnd::default()))),
    )
}

struct FindEnd {
    clipboard: Clipboard,
    clipboard_text: String,
}

impl Default for FindEnd {
    fn default() -> Self {
        Self {
            clipboard: Clipboard::new().expect("failed to initialize clipboard"),
            clipboard_text: String::new(),
        }
    }
}

impl eframe::App for FindEnd {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                if ui.button("Get clipboard").clicked() {
                    self.clipboard_text = self.clipboard.get_text().unwrap_or_default();
                }
                ui.add_space(10.0);
                ui.add(Label::new(format!("{}", self.clipboard_text)));
            })
        });
    }
}
