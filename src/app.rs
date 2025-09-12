use eframe::egui::CentralPanel;
use egui_plot::{Line, Plot, PlotPoints};

use crate::app::decoder::Decoder;

mod decoder;

pub struct App {
    decoder: Decoder,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let decoder = Decoder::new("audio.flac");
        App { decoder }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            if ui.button("Decode").clicked() {
                self.decoder.decode();
            }
            let samples = self.decoder.get_samples();
            if samples.len() > 0 {
                let plot_points = PlotPoints::from_ys_f32(&samples[0..1000]);
                let line = Line::new("waveform", plot_points);
                Plot::new("waveform").show(ui, |plot_ui| {
                    plot_ui.line(line);
                });
            }
        });
    }
}
