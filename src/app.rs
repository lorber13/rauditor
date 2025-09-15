use std::{f64::INFINITY, time::Instant};

use eframe::{
    egui::{CentralPanel, Color32, Stroke, Vec2, Vec2b},
    glow::BLUE,
};
use egui_plot::{Bar, BarChart, Line, Plot, PlotBounds, PlotPoints, VLine};

use crate::app::{audio::WaveForm, decoder::Decoder};

mod audio;
mod decoder;

pub struct App {
    decoder: Decoder,
    start_time: Option<Instant>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let decoder = Decoder::new("audio/audio.flac");
        App {
            decoder,
            start_time: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            if ui.button("Decode").clicked() {
                self.decoder.decode();
                self.start_time = Some(Instant::now());
            }
            let samples = self.decoder.get_samples();
            if samples.len() > 0 {
                // let plot_points = PlotPoints::from_ys_f64(&samples);
                // let waveform = Line::new("left", plot_points).color(Color32::BLUE);
                let waveform = WaveForm::new("left", samples).color(Color32::BLUE);
                Plot::new("waveform_left")
                    .show_y(false)
                    .allow_axis_zoom_drag(Vec2b::new(true, false))
                    .allow_boxed_zoom(false)
                    .set_margin_fraction(Vec2::new(0.0, 0.1))
                    .allow_zoom(Vec2b::new(true, false))
                    .allow_scroll(Vec2b::new(true, false))
                    .allow_drag(Vec2b::new(true, false))
                    .height(ui.available_height() / 2 as f32)
                    .show(ui, |plot_ui| {
                        // plot_ui.line(waveform);
                        // plot_ui.vline(
                        //     VLine::new(
                        //         "cursor",
                        //         self.start_time.unwrap().elapsed().as_secs_f32() * 44100.0,
                        //     )
                        //     .color(Color32::RED),
                        // );
                        plot_ui.add(waveform);
                    });
                ctx.request_repaint();
            }
        });
    }
}
