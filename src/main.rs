mod calculator;
mod structs;

use structs::*;

use std::collections::HashSet;
use std::time::Duration;

use arboard::Clipboard;
use eframe::{NativeOptions, egui};
use egui::ViewportBuilder;
use egui::widgets::Label;
use egui_notify::Toasts;

fn main() -> eframe::Result<()> {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_always_on_top()
            .with_inner_size([300.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Endfind",
        options,
        Box::new(|_cc| Ok(Box::new(FindEnd::default()))),
    )
}

struct FindEnd {
    clipboard: Clipboard,
    clipboard_text: String,
    points: HashSet<Point>,
    toasts: Toasts,
    last_clipboard: String,
    running: bool,
}

impl Default for FindEnd {
    fn default() -> Self {
        Self {
            clipboard: Clipboard::new().expect("failed to initialize clipboard"),
            clipboard_text: String::new(),
            points: HashSet::new(),
            toasts: Toasts::default(),
            last_clipboard: String::new(),
            running: false,
        }
    }
}

impl eframe::App for FindEnd {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        // poll clipboard for changes
        if let Ok(current) = self.clipboard.get_text() {
            if current != self.last_clipboard && current.starts_with('/') {
                self.last_clipboard = current.clone();
                if self.running {
                    self.process_clipboard(&current);
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let btn_text = if self.running {
                    "Stop measurement"
                } else {
                    "Start measurement"
                };
                if ui.button(btn_text).clicked() {
                    self.running = !self.running;
                }

                if ui.button("Clear measurements").clicked() {
                    self.points.clear();
                    self.clipboard_text = String::new();
                    self.toasts
                        .info("Cleared measurements")
                        .duration(Duration::from_secs(2));
                }
                ui.add_space(10.0);

                // update display every frame
                self.clipboard_text = if self.points.is_empty() {
                    "No points recorded".to_string()
                } else if self.points.len() < 1 {
                    format!("{} points recorded", self.points.len())
                } else {
                    if let Some(res) =
                        calculator::triangulate(self.points.iter().cloned().collect::<Vec<Point>>())
                    {
                        format!("{}", res)
                    } else {
                        "Calculation error".to_string()
                    }
                };

                ui.add(Label::new(format!("{}", self.clipboard_text)));
            })
        });

        self.toasts.show(ctx);
    }
}

impl FindEnd {
    fn process_clipboard(&mut self, clipboard: &str) {
        let filtered: Vec<f32> = clipboard
            .chars()
            .filter(|c| c.is_numeric() || [' ', '.', '-'].contains(c))
            .collect::<String>()
            .trim()
            .split(' ')
            .filter(|part| part.parse::<f32>().is_ok())
            .map(|part| part.parse::<f32>().unwrap_or_default())
            .collect();

        if filtered.len() == 5 {
            let point = Point {
                x: filtered.get(0).copied().unwrap_or_default(),
                y: filtered.get(1).copied().unwrap_or_default(),
                z: filtered.get(2).copied().unwrap_or_default(),
                yaw: {
                    let raw = filtered.get(3).copied().unwrap_or_default();
                    let normalized = ((raw % 360.0) + 360.0) % 360.0;
                    normalized
                },
                pitch: filtered.get(4).copied().unwrap_or_default(),
            };

            self.points.insert(point);
            self.clipboard_text = format!("{:?}", self.points);
            self.toasts
                .info("Stored new point")
                .duration(Duration::from_secs(2));
        }
    }
}
