use num_traits::sign::signum;
use std::f32::consts::{PI, FRAC_PI_2};

pub enum OscillatorType { 
  SINE, 
  SQUARE, 
  TRIANGLE, 
  SAWTOOTH 
}

pub struct Oscillator {
  pub frequency: f32,
  pub sample_rate: u32, 
  pub oscillator_type: OscillatorType,
  curr: usize
}

impl Iterator for Oscillator {
  type Item = f32;
  fn next(&mut self) -> Option<Self::Item> {
    let x: f32 = self.frequency * self.curr as f32 / self.sample_rate as f32;
    let value = match self.oscillator_type {
      OscillatorType::SINE => (2.0 * PI * x).sin(),
      OscillatorType::SQUARE => signum((2.0 * PI * x).sin()),
      OscillatorType::TRIANGLE => FRAC_PI_2 * (2.0 * PI * x).sin().asin(),
      OscillatorType::SAWTOOTH => 2.0 * (x - (0.5 + x).floor())
    };
    self.curr += 1;
    Some(value)
  } 
}

pub fn oscillator(frequency: f32, sample_rate: u32, oscillator_type: OscillatorType) -> Oscillator {
  Oscillator { frequency, sample_rate, oscillator_type, curr: 0 }
}

#[cfg(test)]
mod tests {
  use assert_approx_eq::assert_approx_eq;
  use num_traits::sign::signum;
  use std::f32::consts::{PI, FRAC_PI_2};
  use super::{OscillatorType, oscillator};

  fn sine(x: f32) -> f32 {
    (2.0 * PI * x).sin()
  }

  fn square(x: f32) -> f32 {
    signum((2.0 * PI * x).sin())
  }

  fn triangle(x: f32) -> f32 {
    FRAC_PI_2 * (2.0 * PI * x).sin().asin()
  }

  fn sawtooth(x: f32) -> f32 {
    2.0 * (x - (0.5 + x).floor())
  }

  #[test]
  fn sine_1() {
    let freq = 1.0;
    let sample_rate = 2000;
    let buffer = oscillator(freq, sample_rate, OscillatorType::SINE).take(100).collect::<Vec<f32>>();
    let control = (0..100).map(|i| sine(freq * i as f32 / sample_rate as f32)).collect::<Vec<f32>>();
    buffer.into_iter().zip(control.into_iter()).for_each(|(b, c)| assert_approx_eq!(b, c)); 
  }


  #[test]
  fn sine_440() {
    let freq = 440.0;
    let sample_rate = 100;
    let buffer = oscillator(freq, sample_rate, OscillatorType::SINE).take(100).collect::<Vec<f32>>();
    let control = (0..100).map(|i| sine(freq * i as f32 / sample_rate as f32)).collect::<Vec<f32>>();
    buffer.into_iter().zip(control.into_iter()).for_each(|(b, c)| assert_approx_eq!(b, c)); 
  }

  #[test]
  fn sine_660() {
    let freq = 660.0;
    let sample_rate = 1000;
    let buffer = oscillator(freq, sample_rate, OscillatorType::SINE).take(100).collect::<Vec<f32>>();
    let control = (0..100).map(|i| sine(freq * i as f32 / sample_rate as f32)).collect::<Vec<f32>>();
    buffer.into_iter().zip(control.into_iter()).for_each(|(b, c)| assert_approx_eq!(b, c)); 
  }

  #[test]
  fn square_1() {
    let freq = 1.0;
    let sample_rate = 2000;
    let buffer = oscillator(freq, sample_rate, OscillatorType::SQUARE).take(100).collect::<Vec<f32>>();
    let control = (0..100).map(|i| square(freq * i as f32 / sample_rate as f32)).collect::<Vec<f32>>();
    buffer.into_iter().zip(control.into_iter()).for_each(|(b, c)| assert_approx_eq!(b, c)); 
  }


  #[test]
  fn square_440() {
    let freq = 440.0;
    let sample_rate = 100;
    let buffer = oscillator(freq, sample_rate, OscillatorType::SQUARE).take(100).collect::<Vec<f32>>();
    let control = (0..100).map(|i| square(freq * i as f32 / sample_rate as f32)).collect::<Vec<f32>>();
    buffer.into_iter().zip(control.into_iter()).for_each(|(b, c)| assert_approx_eq!(b, c)); 
  }

  #[test]
  fn square_660() {
    let freq = 660.0;
    let sample_rate = 1000;
    let buffer = oscillator(freq, sample_rate, OscillatorType::SQUARE).take(100).collect::<Vec<f32>>();
    let control = (0..100).map(|i| square(freq * i as f32 / sample_rate as f32)).collect::<Vec<f32>>();
    buffer.into_iter().zip(control.into_iter()).for_each(|(b, c)| assert_approx_eq!(b, c)); 
  }

  #[test]
  fn triangle_1() {
    let freq = 1.0;
    let sample_rate = 2000;
    let buffer = oscillator(freq, sample_rate, OscillatorType::TRIANGLE).take(100).collect::<Vec<f32>>();
    let control = (0..100).map(|i| triangle(freq * i as f32 / sample_rate as f32)).collect::<Vec<f32>>();
    buffer.into_iter().zip(control.into_iter()).for_each(|(b, c)| assert_approx_eq!(b, c)); 
  }


  #[test]
  fn triangle_440() {
    let freq = 440.0;
    let sample_rate = 100;
    let buffer = oscillator(freq, sample_rate, OscillatorType::TRIANGLE).take(100).collect::<Vec<f32>>();
    let control = (0..100).map(|i| triangle(freq * i as f32 / sample_rate as f32)).collect::<Vec<f32>>();
    buffer.into_iter().zip(control.into_iter()).for_each(|(b, c)| assert_approx_eq!(b, c)); 
  }

  #[test]
  fn triangle_660() {
    let freq = 660.0;
    let sample_rate = 1000;
    let buffer = oscillator(freq, sample_rate, OscillatorType::TRIANGLE).take(100).collect::<Vec<f32>>();
    let control = (0..100).map(|i| triangle(freq * i as f32 / sample_rate as f32)).collect::<Vec<f32>>();
    buffer.into_iter().zip(control.into_iter()).for_each(|(b, c)| assert_approx_eq!(b, c)); 
  }

  #[test]
  fn sawtooth_1() {
    let freq = 1.0;
    let sample_rate = 2000;
    let buffer = oscillator(freq, sample_rate, OscillatorType::SAWTOOTH).take(100).collect::<Vec<f32>>();
    let control = (0..100).map(|i| sawtooth(freq * i as f32 / sample_rate as f32)).collect::<Vec<f32>>();
    buffer.into_iter().zip(control.into_iter()).for_each(|(b, c)| assert_approx_eq!(b, c)); 
  }


  #[test]
  fn sawtooth_440() {
    let freq = 440.0;
    let sample_rate = 100;
    let buffer = oscillator(freq, sample_rate, OscillatorType::SAWTOOTH).take(100).collect::<Vec<f32>>();
    let control = (0..100).map(|i| sawtooth(freq * i as f32 / sample_rate as f32)).collect::<Vec<f32>>();
    buffer.into_iter().zip(control.into_iter()).for_each(|(b, c)| assert_approx_eq!(b, c)); 
  }

  #[test]
  fn sawtooth_660() {
    let freq = 660.0;
    let sample_rate = 1000;
    let buffer = oscillator(freq, sample_rate, OscillatorType::SAWTOOTH).take(100).collect::<Vec<f32>>();
    let control = (0..100).map(|i| sawtooth(freq * i as f32 / sample_rate as f32)).collect::<Vec<f32>>();
    buffer.into_iter().zip(control.into_iter()).for_each(|(b, c)| assert_approx_eq!(b, c)); 
  }
}
