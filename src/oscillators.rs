use crate::DEFAULT_SAMPLE_RATE;
use num_traits::sign::signum;
use std::f32::consts::{PI, FRAC_PI_2};

pub enum OscillatorType { 
  Sine, 
  Square, 
  Triangle, 
  Sawtooth 
}

pub struct Oscillator {
  pub oscillator_type: OscillatorType,
  pub frequency: f32,
  pub sample_rate: u32, 
  curr: usize
}

impl Oscillator {
  pub fn new(oscillator_type: OscillatorType, frequency: f32, sample_rate: Option<u32>) -> Self {
    Self {
      oscillator_type,
      frequency,
      sample_rate: sample_rate.unwrap_or(DEFAULT_SAMPLE_RATE),
      curr: 0
    }
  }
}

impl Iterator for Oscillator {
  type Item = f32;
  fn next(&mut self) -> Option<Self::Item> {
    let x: f32 = self.frequency * self.curr as f32 / self.sample_rate as f32;
    let value = match self.oscillator_type {
      OscillatorType::Sine => (2.0 * PI * x).sin(),
      OscillatorType::Square => signum((2.0 * PI * x).sin()),
      OscillatorType::Triangle => FRAC_PI_2 * (2.0 * PI * x).sin().asin(),
      OscillatorType::Sawtooth => 2.0 * (x - (0.5 + x).floor())
    };
    self.curr += 1;
    Some(value)
  } 
}


#[cfg(test)]
mod tests {
  use assert_approx_eq::assert_approx_eq;
  use num_traits::sign::signum;
  use std::f32::consts::{PI, FRAC_PI_2};
  use super::{OscillatorType, Oscillator};

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
    let oscillator = Oscillator::new(OscillatorType::Sine, freq, Some(sample_rate));
    let buffer = oscillator.take(100).collect::<Vec<f32>>();
    let control = (0..100).map(|i| sine(freq * i as f32 / sample_rate as f32))
                          .collect::<Vec<f32>>();
    buffer.into_iter().zip(control.into_iter()).for_each(|(b, c)| assert_approx_eq!(b, c)); 
  }


  #[test]
  fn sine_440() {
    let freq = 440.0;
    let sample_rate = 100;
    let oscillator = Oscillator::new(OscillatorType::Sine, freq, Some(sample_rate));
    let buffer = oscillator.take(100).collect::<Vec<f32>>();
    let control = (0..100).map(|i| sine(freq * i as f32 / sample_rate as f32))
                          .collect::<Vec<f32>>();
    buffer.into_iter().zip(control.into_iter()).for_each(|(b, c)| assert_approx_eq!(b, c)); 
  }

  #[test]
  fn sine_660() {
    let freq = 660.0;
    let sample_rate = 1000;
    let oscillator = Oscillator::new(OscillatorType::Sine, freq, Some(sample_rate));
    let buffer = oscillator.take(100).collect::<Vec<f32>>();
    let control = (0..100).map(|i| sine(freq * i as f32 / sample_rate as f32))
                          .collect::<Vec<f32>>();
    buffer.into_iter().zip(control.into_iter()).for_each(|(b, c)| assert_approx_eq!(b, c)); 
  }

  #[test]
  fn square_1() {
    let freq = 1.0;
    let sample_rate = 2000;
    let oscillator = Oscillator::new(OscillatorType::Square, freq, Some(sample_rate));
    let buffer = oscillator.take(100).collect::<Vec<f32>>();
    let control = (0..100).map(|i| square(freq * i as f32 / sample_rate as f32))
                          .collect::<Vec<f32>>();
    buffer.into_iter().zip(control.into_iter()).for_each(|(b, c)| assert_approx_eq!(b, c)); 
  }


  #[test]
  fn square_440() {
    let freq = 440.0;
    let sample_rate = 100;
    let oscillator = Oscillator::new(OscillatorType::Square, freq, Some(sample_rate));
    let buffer = oscillator.take(100).collect::<Vec<f32>>();
    let control = (0..100).map(|i| square(freq * i as f32 / sample_rate as f32))
                          .collect::<Vec<f32>>();
    buffer.into_iter().zip(control.into_iter()).for_each(|(b, c)| assert_approx_eq!(b, c)); 
  }

  #[test]
  fn square_660() {
    let freq = 660.0;
    let sample_rate = 1000;
    let oscillator = Oscillator::new(OscillatorType::Square, freq, Some(sample_rate));
    let buffer = oscillator.take(100).collect::<Vec<f32>>();
    let control = (0..100).map(|i| square(freq * i as f32 / sample_rate as f32))
                          .collect::<Vec<f32>>();
    buffer.into_iter().zip(control.into_iter()).for_each(|(b, c)| assert_approx_eq!(b, c)); 
  }

  #[test]
  fn triangle_1() {
    let freq = 1.0;
    let sample_rate = 2000;
    let oscillator = Oscillator::new(OscillatorType::Triangle, freq, Some(sample_rate));
    let buffer = oscillator.take(100).collect::<Vec<f32>>();
    let control = (0..100).map(|i| triangle(freq * i as f32 / sample_rate as f32))
                          .collect::<Vec<f32>>();
    buffer.into_iter().zip(control.into_iter()).for_each(|(b, c)| assert_approx_eq!(b, c)); 
  }


  #[test]
  fn triangle_440() {
    let freq = 440.0;
    let sample_rate = 100;
    let oscillator = Oscillator::new(OscillatorType::Triangle, freq, Some(sample_rate));
    let buffer = oscillator.take(100).collect::<Vec<f32>>();
    let control = (0..100).map(|i| triangle(freq * i as f32 / sample_rate as f32))
                          .collect::<Vec<f32>>();
    buffer.into_iter().zip(control.into_iter()).for_each(|(b, c)| assert_approx_eq!(b, c)); 
  }

  #[test]
  fn triangle_660() {
    let freq = 660.0;
    let sample_rate = 1000;
    let oscillator = Oscillator::new(OscillatorType::Triangle, freq, Some(sample_rate));
    let buffer = oscillator.take(100).collect::<Vec<f32>>();
    let control = (0..100).map(|i| triangle(freq * i as f32 / sample_rate as f32))
                          .collect::<Vec<f32>>();
    buffer.into_iter().zip(control.into_iter()).for_each(|(b, c)| assert_approx_eq!(b, c)); 
  }

  #[test]
  fn sawtooth_1() {
    let freq = 1.0;
    let sample_rate = 2000;
    let oscillator = Oscillator::new(OscillatorType::Sawtooth, freq, Some(sample_rate));
    let buffer = oscillator.take(100).collect::<Vec<f32>>();
    let control = (0..100).map(|i| sawtooth(freq * i as f32 / sample_rate as f32))
                          .collect::<Vec<f32>>();
    buffer.into_iter().zip(control.into_iter()).for_each(|(b, c)| assert_approx_eq!(b, c)); 
  }


  #[test]
  fn sawtooth_440() {
    let freq = 440.0;
    let sample_rate = 100;
    let oscillator = Oscillator::new(OscillatorType::Sawtooth, freq, Some(sample_rate));
    let buffer = oscillator.take(100).collect::<Vec<f32>>();
    let control = (0..100).map(|i| sawtooth(freq * i as f32 / sample_rate as f32))
                          .collect::<Vec<f32>>();
    buffer.into_iter().zip(control.into_iter()).for_each(|(b, c)| assert_approx_eq!(b, c)); 
  }

  #[test]
  fn sawtooth_660() {
    let freq = 660.0;
    let sample_rate = 1000;
    let oscillator = Oscillator::new(OscillatorType::Sawtooth, freq, Some(sample_rate));
    let buffer = oscillator.take(100).collect::<Vec<f32>>();
    let control = (0..100).map(|i| sawtooth(freq * i as f32 / sample_rate as f32))
                          .collect::<Vec<f32>>();
    buffer.into_iter().zip(control.into_iter()).for_each(|(b, c)| assert_approx_eq!(b, c)); 
  }
}
