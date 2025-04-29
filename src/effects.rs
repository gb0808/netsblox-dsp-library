use crate::buffer::AudioBuffer;
use std::iter::once;

pub struct AudioEffect {}

impl AudioEffect {
  pub fn delay(buffer: &mut AudioBuffer, delay_time: f32) {
    let sample_rate = buffer.sample_rate;
    let zero_pad_size: usize = (sample_rate as f32 * delay_time).ceil().abs() as usize;
    let mut signal = buffer.src.clone();
    let mut zero_pad = once(0.0).cycle().take(zero_pad_size).collect::<Vec<f32>>();
    zero_pad.append(&mut signal);
    buffer.src = zero_pad;
  }
}

#[cfg(test)]
mod tests {
  use crate::buffer::AudioBuffer;
  use crate::DEFAULT_SAMPLE_RATE;
  use crate::effects::AudioEffect;

  #[test]
  fn test_delay() {
    let signal = (0..DEFAULT_SAMPLE_RATE).map(|i| (i as f32).sin()).collect::<Vec<f32>>();

    let mut buffer_1 = AudioBuffer::from_raw(&signal, None);
    AudioEffect::delay(&mut buffer_1, 1.0);
    assert_eq!(buffer_1.src.len(), DEFAULT_SAMPLE_RATE as usize * 2);

    let mut buffer_2 = AudioBuffer::from_raw(&signal, None);
    AudioEffect::delay(&mut buffer_2, 0.5);
    assert_eq!(buffer_2.src.len(), (DEFAULT_SAMPLE_RATE as f32 * 1.5).ceil() as usize);

    let mut buffer_3 = AudioBuffer::from_raw(&signal, None);
    AudioEffect::delay(&mut buffer_3, 0.0);
    assert_eq!(buffer_3.src.len(), DEFAULT_SAMPLE_RATE as usize);
  }
}
