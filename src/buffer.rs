use crate::oscillators::Oscillator;

pub struct AudioBuffer {
  sample_rate: u32,
  src: Vec<f32>
}

impl AudioBuffer {
  pub fn from_oscillator(oscillator: Oscillator, duration: usize) -> Self {
    let sample_rate = oscillator.sample_rate;
    Self {
      sample_rate,
      src: oscillator.take(duration * sample_rate as usize).collect::<Vec<f32>>()
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::buffer::AudioBuffer;
  use crate::oscillators::{OscillatorType, Oscillator};

  #[test]
  fn test_from_oscillator() {
    let oscillator = Oscillator::new(OscillatorType::SINE, 440.0, None);
    let duration = 2;
    let buffer = AudioBuffer::from_oscillator(oscillator, duration);
    assert_eq!(buffer.sample_rate, 44100);
    assert_eq!(buffer.src.len(), 44100 * 2);
  }
}
