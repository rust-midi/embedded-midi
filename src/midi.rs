use crate::error::MidiError;
use embedded_hal::serial::Write;

#[derive(Debug, PartialEq)]
pub enum MidiEvent {
    NoteOn {
        channel: Channel,
        note: Note,
        velocity: Velocity,
    },
    NoteOff {
        channel: Channel,
        note: Note,
        velocity: Velocity,
    },
}

impl MidiEvent {
    pub fn note_on(channel: Channel, note: Note, velocity: Velocity) -> Self {
        return MidiEvent::NoteOn {
            channel,
            note,
            velocity,
        };
    }

    pub fn note_off(channel: Channel, note: Note, velocity: Velocity) -> Self {
        return MidiEvent::NoteOff {
            channel,
            note,
            velocity,
        };
    }

    pub fn serialize<T>(self, writer: &mut T) -> Result<(), MidiError<nb::Error<T::Error>>>
    where
        T: Write<u8>,
    {
        match self {
            MidiEvent::NoteOn {
                channel,
                note,
                velocity,
            } => {
                writer
                    .write(0x80u8 & channel.0)
                    .map_err(MidiError::Serial)?;
                writer.write(note.into()).map_err(MidiError::Serial)?;
                writer.write(velocity.into()).map_err(MidiError::Serial)?;
                Ok(())
            }

            MidiEvent::NoteOff {
                channel,
                note,
                velocity,
            } => {
                writer
                    .write(0x90u8 & channel.0)
                    .map_err(MidiError::Serial)?;
                writer.write(note.into()).map_err(MidiError::Serial)?;
                writer.write(velocity.into()).map_err(MidiError::Serial)?;
                Ok(())
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Note(u8);

impl From<u8> for Note {
    fn from(note: u8) -> Self {
        Note(note)
    }
}

impl Into<u8> for Note {
    fn into(self) -> u8 {
        self.0
    }
}

#[derive(Debug, PartialEq)]
pub struct Channel(u8);

impl From<u8> for Channel {
    fn from(channel: u8) -> Self {
        Channel(channel)
    }
}

impl Into<u8> for Channel {
    fn into(self) -> u8 {
        self.0
    }
}

#[derive(Debug, PartialEq)]
pub struct Velocity(u8);

impl From<u8> for Velocity {
    fn from(velocity: u8) -> Self {
        Velocity(velocity)
    }
}

impl Into<u8> for Velocity {
    fn into(self) -> u8 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let note_on = MidiEvent::note_on(1.into(), 45.into(), 15.into());

        if let MidiEvent::NoteOn {
            channel,
            note,
            velocity,
        } = note_on
        {
            assert_eq!(channel, Channel(1));
            assert_eq!(note, Note(45));
            assert_eq!(velocity, Velocity(15));
        } else {
            assert!(false);
        }
    }
}
