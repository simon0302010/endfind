use std::{collections::BTreeSet, fmt};

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

#[derive(Default, Debug)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
    yaw: f32,
    pitch: f32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "X: {}, Y: {}, Z: {}\nYaw: {}, Pitch: {}",
            self.x, self.y, self.z, self.yaw, self.pitch
        )
    }
}

struct FindEnd {
    clipboard: Clipboard,
    clipboard_text: String,
    points: BTreeSet<Point>,
}

impl Default for FindEnd {
    fn default() -> Self {
        Self {
            clipboard: Clipboard::new().expect("failed to initialize clipboard"),
            clipboard_text: String::new(),
            points: BTreeSet::new(),
        }
    }
}

impl eframe::App for FindEnd {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                if ui.button("Get clipboard").clicked() {
                    let clipboard = self.clipboard.get_text().unwrap_or_default();
                    let filtered: Vec<f32> = clipboard
                        .chars()
                        .filter(|c| c.is_numeric() || [' ', '.'].contains(c))
                        .collect::<String>()
                        .trim()
                        .split(' ')
                        .filter(|part| part.parse::<f32>().is_ok())
                        .map(|part| part.parse::<f32>().unwrap_or_default())
                        .collect();

                    if filtered.len() == 5 && clipboard.trim().starts_with('/') {
                        self.clipboard_text = format!("{:?}", filtered);
                    } else {
                        self.clipboard_text = "Failed to parse command.".to_string();
                    }
                }
                ui.add_space(10.0);
                ui.add(Label::new(format!("{}", self.clipboard_text)));
            })
        });
    }
}
