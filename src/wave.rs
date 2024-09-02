use std::f64::consts::PI;

#[derive(Default)]
pub struct Waveform {
    x: Vec<f64>,
    y: Vec<f64>,
}

impl Waveform {
    pub fn example() -> Self {
        let x: Vec<_> = (0..=1000).map(|i| i as f64 / 1000.0).collect();
        let y: Vec<_> = x.iter().map(|x| (10.0 * 2.0 * PI * x).sin()).collect();
        Self { x, y }
    }
    pub fn range_x(&self) -> (f64, f64) {
        (
            *self.x.first().unwrap_or(&0.0),
            *self.x.last().unwrap_or(&1.0),
        )
    }
    pub fn range_y(&self) -> (f64, f64) {
        (
            *self.y.first().unwrap_or(&0.0),
            *self.y.last().unwrap_or(&1.0),
        )
    }
}

pub struct PlotWaveform {
    wfm: Waveform,
    name: String,
    visible: bool,
}

impl PlotWaveform {
    pub fn new(wfm: Waveform, name: &str) -> Self {
        PlotWaveform {
            wfm,
            name: name.to_owned(),
            visible: true,
        }
    }
}
