use rustfft::{FftPlanner, num_complex::Complex};
use wasm_bindgen::prelude::*;

fn normalize(signal: &mut [f32]) {
    let mut max: f32 = 0.0;
    signal.into_iter().for_each(|sample| {
        if *sample > max {
            max = *sample;
        }
    });
    signal.into_iter().for_each(|sample| *sample = *sample / max);
}

#[wasm_bindgen]
pub fn fourier_transform(signal: &[f32]) -> Vec<f32> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(signal.len());

    let mut buffer: Vec<Complex<f32>> = signal.into_iter()
                                              .map(|sample| Complex::new(*sample, 0.0))
                                              .collect();
    fft.process(&mut buffer);

    buffer.into_iter().map(|bin| bin.norm()).collect()
}