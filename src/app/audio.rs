use std::ops::RangeInclusive;

use eframe::egui::{Color32, Pos2, Shape, Stroke, Ui};
use egui_plot::{PlotBounds, PlotGeometry, PlotItem, PlotItemBase, PlotPoint, PlotTransform};

// A sequence of vertical one-pixel-wide bars
#[derive(Clone, Debug, PartialEq)]
pub struct WaveForm<'l> {
    base: PlotItemBase,
    samples: &'l [f32],
    color: Color32,
}

impl<'l> WaveForm<'l> {
    pub fn new(name: impl Into<String>, samples: impl Into<&'l [f32]>) -> Self {
        Self {
            base: PlotItemBase::new(name.into()),
            samples: samples.into(),
            color: Color32::TRANSPARENT,
        }
    }

    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.color = color.into();
        self
    }
}

impl PlotItem for WaveForm<'_> {
    fn shapes(&self, _ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        let mut x = 0.0;

        for sample in self.samples {
            let points = [
                transform.position_from_point(&PlotPoint { x, y: 0.0 }),
                transform.position_from_point(&PlotPoint {
                    x,
                    y: *sample as f64,
                }),
            ];
            shapes.push(Shape::LineSegment {
                points,
                stroke: Stroke {
                    width: 1.0,
                    color: self.color,
                },
            });
            x += 1.0;
        }
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {}

    fn color(&self) -> Color32 {
        self.color
    }

    fn base(&self) -> &PlotItemBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut PlotItemBase {
        &mut self.base
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::None
    }

    fn bounds(&self) -> PlotBounds {
        let mut max = -1.0;
        let mut min = 1.0;
        for sample in self.samples {
            if *sample > max {
                max = *sample;
            }
            if *sample < min {
                min = *sample;
            }
        }
        PlotBounds::from_min_max([0.0, min as f64], [self.samples.len() as f64, max as f64])
    }
}
