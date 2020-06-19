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
    ControlChange {
        channel: Channel,
        controller: u8,
        value: u8,
    },
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
