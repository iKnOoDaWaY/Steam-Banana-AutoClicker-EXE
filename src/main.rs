use image::ImageBuffer;
use eframe::egui;
use enigo::*;
use std::thread;
use std::time::Duration;

struct BananaClickerApp {
    clicking: bool,
    cps: f32, // Clicks per second
    enigo: Enigo,
    status: String,
}

impl Default for BananaClickerApp {
    fn default() -> Self {
        let settings = Settings::default();
        let enigo = Enigo::new(&settings).unwrap();

        Self {
            clicking: false,
            cps: 50.0,
            enigo,
            status: "Ready. Press START or F1 to begin!".to_owned(),
        }
    }
}

impl eframe::App for BananaClickerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Keep the UI responsive and allow fast clicking
        ctx.request_repaint();

        // F1 hotkey to toggle
        if ctx.input(|i| i.key_pressed(egui::Key::F1)) {
            self.clicking = !self.clicking;
            self.update_status();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("🍌 Banana AutoClicker 🍌");
                ui.add_space(20.0);

                // Big Start/Stop button
                let button_text = if self.clicking { "🛑 STOP" } else { "START" };
                let button = egui::Button::new(button_text)
                    .min_size(egui::vec2(200.0, 60.0))
                    .fill(if self.clicking {
                        egui::Color32::from_rgb(255, 100, 100)
                    } else {
                        egui::Color32::from_rgb(100, 255, 100)
                    });

                if ui.add(button).clicked() {
                    self.clicking = !self.clicking;
                    self.update_status();
                }

                ui.add_space(20.0);

                // Speed slider
                ui.horizontal(|ui| {
                    ui.label("Click Speed:");
                    ui.add(
                        egui::Slider::new(&mut self.cps, 1.0..=1000.0)
                            .text("CPS")
                            .logarithmic(true),
                    );
                    ui.label(format!("{:.1} clicks/sec", self.cps));
                });

                ui.add_space(20.0);

                // Status text
                ui.label(
                    egui::RichText::new(&self.status)
                        .size(18.0)
                        .color(if self.clicking {
                            egui::Color32::LIGHT_GREEN
                        } else {
                            egui::Color32::GRAY
                        }),
                );

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    ui.label("Hotkey:");
                    ui.label(egui::RichText::new("F1").strong())
                        .on_hover_text("Toggle clicking");
                });

                ui.add_space(10.0);
                ui.label("By Scratchy");
            });
        });

        // Auto-clicking logic
        if self.clicking {
            let interval_ms = if self.cps > 0.0 { 1000.0 / self.cps } else { 1.0 };
            let interval = Duration::from_secs_f32(interval_ms / 1000.0);

            self.enigo.button(Button::Left, Direction::Press);
            self.enigo.button(Button::Left, Direction::Release);

            thread::sleep(interval);
        } else {
            thread::sleep(Duration::from_millis(10));
        }
    }
}

impl BananaClickerApp {
    fn update_status(&mut self) {
        self.status = if self.clicking {
            format!("🔥 Clicking at {:.1} CPS! 🔥", self.cps)
        } else {
            "Paused. Press START or F1 to resume.".to_owned()
        };
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_resizable(false)
            .with_icon(load_icon()), // <-- Banana icon here!
        ..Default::default()
    };

    eframe::run_native(
        "Banana AutoClicker",
        options,
        Box::new(|_cc| Ok(Box::new(BananaClickerApp::default()))),
    )
}

fn load_icon() -> egui::IconData {
    // Load the .ico file you placed in the project root
    let icon_bytes = include_bytes!("banana.ico");

    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to load banana.ico - make sure it's a valid .ico file in the project root")
        .to_rgba8();

    let (width, height) = image.dimensions();

    egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}