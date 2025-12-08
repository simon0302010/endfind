use std::{collections::HashSet, fmt};
use std::time::Duration;

use arboard::Clipboard;
use eframe::{NativeOptions, egui};
use egui::widgets::Label;
use egui_notify::Toasts;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Endfind",
        NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(FindEnd::default()))),
    )
}

#[derive(Default, Debug, PartialEq, Clone)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
    yaw: f32,
    pitch: f32,
}

impl Eq for Point {}

impl std::hash::Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
        self.z.to_bits().hash(state);
        self.yaw.to_bits().hash(state);
        self.pitch.to_bits().hash(state);
    }
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
    points: HashSet<Point>,
    toasts: Toasts
}

impl Default for FindEnd {
    fn default() -> Self {
        Self {
            clipboard: Clipboard::new().expect("failed to initialize clipboard"),
            clipboard_text: String::new(),
            points: HashSet::new(),
            toasts: Toasts::default()
        }
    }
}

impl eframe::App for FindEnd {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                if ui.button("New measurement").clicked() {
                    let clipboard = self.clipboard.get_text().unwrap_or_default();
                    let filtered: Vec<f32> = clipboard
                        .chars()
                        .filter(|c| c.is_numeric() || [' ', '.', '-'].contains(c))
                        .collect::<String>()
                        .trim()
                        .split(' ')
                        .filter(|part| part.parse::<f32>().is_ok())
                        .map(|part| part.parse::<f32>().unwrap_or_default())
                        .collect();

                    if filtered.len() == 5 && clipboard.trim().starts_with('/') {
                        let point = Point {
                            x: filtered.get(0).copied().unwrap_or_default(),
                            y: filtered.get(1).copied().unwrap_or_default(),
                            z: filtered.get(2).copied().unwrap_or_default(),
                            yaw: {
                                let raw = filtered.get(3).copied().unwrap_or_default();
                                let normalized = raw % 360.0;
                                if normalized > 180.0 {
                                    normalized - 360.0
                                } else if normalized < -180.0 {
                                    normalized + 360.0
                                } else {
                                    normalized
                                }
                            },
                            pitch: filtered.get(4).copied().unwrap_or_default(),
                        };

                        self.points.insert(point);

                        self.clipboard_text = format!("{:?}", self.points);

                        self.toasts.info("Stored new point").duration(Duration::from_secs(2));
                    } else {
                        self.clipboard_text = "Failed to parse command.".to_string();
                    }
                }
                if ui.button("Clear measurements").clicked() {
                    self.points.clear();
                    self.clipboard_text = String::new();
                    self.toasts.info("Cleared measurements").duration(Duration::from_secs(2));
                }
                ui.add_space(10.0);
                ui.add(Label::new(format!("{}", self.clipboard_text)));
            })
        });

        self.toasts.show(ctx);
    }
}
