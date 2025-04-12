use rustfft::{FftPlanner, num_complex::Complex};
use wasm_bindgen::prelude::*;

fn autocorrelation_function_inner(signal: &[f32], lag: usize, window_size: usize) -> f32 {
  let signal_window = &signal[..window_size];
  let (shifted_start, shifted_end) = (lag, lag + window_size);
  let shifted_signal_window = &signal[shifted_start..shifted_end];
  signal_window.into_iter()
               .zip(shifted_signal_window.into_iter())
               .map(|(i, j)| i * j)
               .collect::<Vec<f32>>()
               .into_iter()
               .sum()
}

#[wasm_bindgen]
pub fn autocorrelation_function(signal: &[f32], window_size: usize) -> Vec<f32> {
  let lag_iter = 0..window_size;
  lag_iter.map(|lag| autocorrelation_function_inner(signal, lag, window_size)).collect() 
}

#[wasm_bindgen]
pub fn normalize(signal: &[f32]) -> Vec<f32> {
  let mut max: f32 = 0.0;
  signal.into_iter().for_each(|&sample| {
    if sample > max {
      max = sample;
    }
  });
  if max == 0.0 {
    return vec![0.0; signal.len()]
  }
  signal.into_iter().map(|sample| *sample / max).collect()
}

#[wasm_bindgen]
pub fn join(signal_a: &[f32], signal_b: &[f32]) -> Vec<f32> {
  let signal_length = signal_a.len().max(signal_b.len());
  let a_iter = signal_a.into_iter().chain(std::iter::once(&0.0).cycle());
  let b_iter = signal_b.into_iter().chain(std::iter::once(&0.0).cycle());
  a_iter.zip(b_iter).take(signal_length).map(|(a, b)| a + b).collect()
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
