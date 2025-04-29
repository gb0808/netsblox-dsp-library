use crate::DEFAULT_SAMPLE_RATE;
use crate::oscillators::Oscillator;

pub struct AudioBuffer {
  pub sample_rate: u32,
  pub src: Vec<f32>
}

impl AudioBuffer {
  pub fn from_oscillator(oscillator: Oscillator, duration: usize) -> Self {
    let sample_rate = oscillator.sample_rate;
    Self {
      sample_rate,
      src: oscillator.take(duration * sample_rate as usize).collect::<Vec<f32>>()
    }
  }

  pub fn from_raw(samples: &[f32], sample_rate: Option<u32>) -> Self {
    Self {
      sample_rate: sample_rate.unwrap_or(DEFAULT_SAMPLE_RATE),
      src: samples.to_vec()
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::DEFAULT_SAMPLE_RATE;
  use crate::buffer::AudioBuffer;
  use crate::oscillators::{OscillatorType, Oscillator};

  #[test]
  fn test_from_oscillator() {
    let oscillator = Oscillator::new(OscillatorType::Sine, 440.0, None);
    let duration = 2;
    let buffer = AudioBuffer::from_oscillator(oscillator, duration);
    assert_eq!(buffer.sample_rate, DEFAULT_SAMPLE_RATE);
    assert_eq!(buffer.src.len(), DEFAULT_SAMPLE_RATE as usize * 2);
  }

  #[test]
  fn test_from_raw() {
    let samples = [0.0; 1000];
    let buffer = AudioBuffer::from_raw(&samples, None);
    assert_eq!(buffer.sample_rate, DEFAULT_SAMPLE_RATE);
    assert_eq!(buffer.src.len(), 1000);
  }
}
