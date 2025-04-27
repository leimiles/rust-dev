//! Hide console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use std::fs;
use std::path::PathBuf;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "File Browser",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    files: Vec<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        let entries = read_current_dir();
        Self { files: entries }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Current Directory Files:");

            if ui.button("Refresh").clicked() {
                self.files = read_current_dir();
            }

            ui.separator();

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for file in &self.files {
                        ui.label(file);
                    }
                });
        });
    }
}

fn read_current_dir() -> Vec<String> {
    let mut entries = Vec::new();
    if let Ok(dir) = fs::read_dir(".") {
        for entry in dir.flatten() {
            let path: PathBuf = entry.path();
            if let Some(name) = path.file_name() {
                let mut display_name = name.to_string_lossy().to_string();
                if path.is_dir() {
                    display_name = format!("[DIR] {}", display_name);
                }
                entries.push(display_name);
            }
        }
    }
    entries
}
