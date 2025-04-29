const CHANNEL_MAX: u8 = 0b00001111;

#[derive(Debug)]
pub enum MidiError {
  InvalidChannel
}

pub struct MTrkEvent {
  pub delta_time: usize,
  pub event: Event
}

pub enum Event {
  MidiEvent(MidiMessage)
}

pub struct MidiMessage {
  pub status: u8,
  data_byte: (u8, u8)
}

impl MidiMessage {
  pub fn note_off(note_number: u8, velocity: u8, channel: Option<u8>) -> Result<Self, MidiError> {
    let m_channel = channel.unwrap_or(1);
    if m_channel <= CHANNEL_MAX {
      Ok(Self { status: 0b10000000 | m_channel, data_byte: (note_number, velocity) })
    } else {
      Err(MidiError::InvalidChannel)
    }
  }

  pub fn note_on(note_number: u8, velocity: u8, channel: Option<u8>) -> Result<Self, MidiError> {
    let m_channel = channel.unwrap_or(1);
    if m_channel <= CHANNEL_MAX {
      Ok(Self { status: 0b10010000 | m_channel, data_byte: (note_number, velocity) })
    } else {
      Err(MidiError::InvalidChannel)
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::midi::MidiMessage;

  #[test]
  fn test_note_off() {
    let message_1 = MidiMessage::note_off(60, 255, None).unwrap();
    assert_eq!(message_1.status, 0b10000001);
    let message_2 = MidiMessage::note_off(60, 255, Some(3)).unwrap();
    assert_eq!(message_2.status, 0b10000011);
    assert!(MidiMessage::note_off(60, 255, Some(16)).is_err());
  }

  #[test]
  fn test_note_on() {
    let message_1 = MidiMessage::note_on(60, 255, None).unwrap();
    assert_eq!(message_1.status, 0b10010001);
    let message_2 = MidiMessage::note_on(60, 255, Some(3)).unwrap();
    assert_eq!(message_2.status, 0b10010011);
    assert!(MidiMessage::note_on(60, 255, Some(16)).is_err());
  }
}
