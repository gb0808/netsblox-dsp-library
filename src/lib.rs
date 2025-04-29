mod buffer;
mod effects;
mod midi;
mod oscillators;

use wasm_bindgen::prelude::*;
use crate::buffer::AudioBuffer;
use crate::effects::AudioEffect;
use crate::oscillators::{Oscillator, OscillatorType};

const DEFAULT_SAMPLE_RATE: u32 = 44100;

#[wasm_bindgen]
pub fn netsblox_oscillator(oscillator_type_str: &str, frequency: f32, duration: f32) -> Vec<f32> {
  let oscillator_type = match oscillator_type_str {
    "sine" => OscillatorType::SINE,
    "square" => OscillatorType::SQUARE,
    "sawtooth" => OscillatorType::SAWTOOTH,
    "triangle" => OscillatorType::TRIANGLE,
    _ => panic!("wave type not supported")
  };
  let size = (DEFAULT_SAMPLE_RATE as f32 * duration).floor() as usize;
  Oscillator::new(oscillator_type, frequency, None).take(size).collect::<Vec<f32>>() 
}

#[wasm_bindgen]
pub fn netsblox_delay(signal: &[f32], delay_time: f32) -> Vec<f32> {
  let mut buffer = AudioBuffer::from_raw(signal, None);
  AudioEffect::delay(&mut buffer, delay_time);
  buffer.src.clone()
}
