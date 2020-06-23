//! This module contains data types to represent the different messages that can be sent over MIDI.

/// An enum with variants for all possible Midi messages.
#[derive(Debug, PartialEq)]
pub enum MidiEvent {
    // Channel voice messages
    NoteOff {
        channel: Channel,
        note: Note,
        velocity: Value7,
    },
    NoteOn {
        channel: Channel,
        note: Note,
        velocity: Value7,
    },
    KeyPressure {
        channel: Channel,
        note: Note,
        value: Value7,
    },
    ControlChange {
        channel: Channel,
        control: Control,
        value: Value7,
    },
    ProgramChange {
        channel: Channel,
        program: Program,
    },
    ChannelPressure {
        channel: Channel,
        value: Value7,
    },
    PitchBendChange {
        channel: Channel,
        value: Value14,
    },

    // System common messages

    // System real time messages
    TimingClock,
    Start,
    Continue,
    Stop,
    ActiveSensing,
    Reset,
}

/// Represents a midi note
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

/// Represents a Midi channel, Midi channels can range from 0 to 15, but are represented as 1 based
/// values Channel 1 to 16
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

/// A Midi controller number
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Control(u8);

impl From<u8> for Control {
    fn from(control: u8) -> Self {
        Control(control)
    }
}

impl Into<u8> for Control {
    fn into(self) -> u8 {
        self.0
    }
}

/// A Midi program number, these usually correspond to presets on Midi devices
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Program(u8);

impl From<u8> for Program {
    fn from(value: u8) -> Self {
        Program(value)
    }
}

impl Into<u8> for Program {
    fn into(self) -> u8 {
        self.0
    }
}

/// A 7 bit Midi data value stored in an unsigned 8 bit integer, the msb is always 0
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Value7(u8);

impl From<u8> for Value7 {
    fn from(value: u8) -> Self {
        Value7(value)
    }
}

impl Into<u8> for Value7 {
    fn into(self) -> u8 {
        self.0
    }
}

/// A 14 bit Midi value stored as two 7 bit Midi data values, where the msb is always 0 to signify
/// that this is a data value.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Value14(u8, u8);

impl From<(u8, u8)> for Value14 {
    fn from(value: (u8, u8)) -> Self {
        Value14(value.0, value.1)
    }
}

impl Into<(u8, u8)> for Value14 {
    fn into(self) -> (u8, u8) {
        (self.0, self.1)
    }
}

impl From<u16> for Value14 {
    fn from(value: u16) -> Self {
        Value14(((value & 0x3f80) >> 7) as u8, (value & 0x007f) as u8)
    }
}

impl Into<u16> for Value14 {
    fn into(self) -> u16 {
        (self.0 as u16) * 128 + self.1 as u16
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_combine_7_bit_vals_into_14() {
        let val: Value14 = (0b01010101u8, 0b01010101u8).into();
        assert_eq!(0b0010101011010101u16, val.into());
    }

    #[test]
    fn should_split_14_bit_val_into_7() {
        let val: Value14 = 0b0011001100110011u16.into();
        assert_eq!((0b01100110u8, 0b00110011u8), val.into())
    }
}
