use rustfft::{FftPlanner, num_complex::Complex};
use wasm_bindgen::prelude::*;

fn zero_pad(signal: &[f32], pad_length: usize) -> Vec<f32> {
    let pad = vec![0.0; pad_length];
    [signal, &pad].concat()
}

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
pub fn join(signal_a: &[f32], signal_b: &[f32]) -> Vec<f32> {
    let (a, b): (&[f32], &[f32]) = if signal_a.len() > signal_b.len() {
        let dif = signal_a.len() - signal_b.len();
        (signal_a, &zero_pad(signal_b, dif))
    } else if signal_a.len() < signal_b.len() {
        let dif = signal_b.len() - signal_a.len();
        (&zero_pad(signal_a, dif), signal_b)
    } else {
        (signal_a, signal_b)
    };

    let mut joined_signal: Vec<f32> = a.into_iter()
                                       .zip(b.into_iter())
                                       .map(|(x, y)| *x + *y)
                                       .collect();
    normalize(&mut joined_signal);

    joined_signal
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