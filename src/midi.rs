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
    ControllerChange {
        channel: Channel,
        controller: u8,
        value: u8,
    },
}

impl MidiEvent {
    pub fn note_on(channel: Channel, note: Note, velocity: Velocity) -> Self {
        MidiEvent::NoteOn {
            channel,
            note,
            velocity,
        }
    }

    pub fn note_off(channel: Channel, note: Note, velocity: Velocity) -> Self {
        MidiEvent::NoteOff {
            channel,
            note,
            velocity,
        }
    }

    pub fn controller_change(channel: Channel, controller: u8, value: u8) -> Self {
        MidiEvent::ControllerChange {
            channel,
            controller,
            value,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
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

#[derive(Debug, PartialEq, Copy, Clone)]
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

#[derive(Debug, PartialEq, Copy, Clone)]
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
    fn should_encode_note_on() {
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

    #[test]
    fn should_encode_note_off() {
        let note_off = MidiEvent::note_off(0.into(), 0x30.into(), 15.into());

        if let MidiEvent::NoteOff {
            channel,
            note,
            velocity,
        } = note_off
        {
            assert_eq!(channel, Channel(0));
            assert_eq!(note, Note(0x30));
            assert_eq!(velocity, Velocity(15));
        } else {
            assert!(false);
        }
    }
}
