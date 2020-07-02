//! This module contains data types to represent the different messages that can be sent over MIDI.

/// An enum with variants for all possible Midi messages.
#[derive(Debug, PartialEq)]
pub enum MidiMessage {
    // Channel voice messages
    /// Note Off message
    NoteOff {
        /// Channel can be 0 to 15 for Midi channels 1 to 16
        channel: Channel,

        /// The note number
        note: Note,

        /// Note off velocity
        velocity: Value7,
    },

    /// Note on message
    NoteOn {
        /// Channel can be 0 to 15 for Midi channels 1 to 16
        channel: Channel,

        /// The note number
        note: Note,

        /// Note on velocity
        velocity: Value7,
    },

    /// KeyPressure message for polyphonic aftertouch
    KeyPressure {
        /// Channel can be 0 to 15 for Midi channels 1 to 16
        channel: Channel,

        /// The note number
        note: Note,

        /// The keypressure value
        value: Value7,
    },

    /// Control change message
    ControlChange {
        /// Channel can be 0 to 15 for Midi channels 1 to 16
        channel: Channel,

        /// The control number
        control: Control,

        /// The control value
        value: Value7,
    },

    /// Program change message
    ProgramChange {
        /// Channel can be 0 to 15 for Midi channels 1 to 16
        channel: Channel,

        /// The program number
        program: Program,
    },

    /// Channel pressure message for channel aftertouch
    ChannelPressure {
        /// Channel can be 0 to 15 for Midi channels 1 to 16
        channel: Channel,

        /// The pressure value
        value: Value7,
    },

    /// Pitch bend message
    PitchBendChange {
        /// Channel can be 0 to 15 for Midi channels 1 to 16
        channel: Channel,

        /// The pitchbend value
        value: Value14,
    },

    // System common messages
    /// System exclusive message starts
    SystemExclusive {
        /// The system exclusive manufacturer id, this is either a 1 byte or 3 byte number
        manufacturer_id: u32,
    },

    /// System exclusive data is received
    SystemExclusiveData {
        /// The received data value
        value: Value7,
    },

    /// Signals the end of the system exclusive block
    EndOfExclusive,

    /// Midi time code quarter frame
    QuarterFrame {
        /// TODO: implement parsing this message payload
        values: Value7,
    },

    /// Set the song position pointer
    SongPositionPointer {
        /// A 14 bit value representing the number of beats since the start of the song
        pointer: Value14,
    },

    /// Specifies which sequence or song is to be played
    SongSelect {
        /// The song or sequence
        value: Value7,
    },

    /// Tune all oscillators
    TuneRequest,

    // System real time messages
    /// Timing tick message
    TimingClock,

    /// Start message
    Start,

    /// Continue message
    Continue,

    /// Stop message
    Stop,

    /// Active sensing message
    ActiveSensing,

    /// Reset message
    Reset,
}

/// Represents a midi note number where 0 corresponds to C-2 and 127 corresponds to G8,
/// C4 is 72
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

/// The SMPTE type used. This indicates the number of frames per second
pub enum SmpteType {
    /// 24 frames per second
    Frames24,

    /// 25 frames per second
    Frames25,

    /// 29.97 frames per second
    DropFrame30,

    /// 30 frames per second
    Frames30,
}

/// The value of the quarter frame message, this message contains a message type and a value. Each
/// of these eight messages encodes a 4 bit part of the midi time code. As one of these is sent
/// every quarter frames, the complete midi time code is sent every two frames.
pub enum QuarterFrame {
    /// Frame number low nibble
    FramesLS { value: u8 },
    /// Frame count high nibble
    FramesMS { value: u8 },
    /// Seconds low nibble
    SecondsLS { value: u8 },
    /// Seconds high nibble
    SecondsMS { value: u8 },
    /// Minutes low nibble
    MinutesLS { value: u8 },
    /// Minutes high nibble
    MinutesMS { value: u8 },
    /// Hours low nibble
    HoursLS { value: u8 },
    /// Combined hours high nibble and smpte type (frames per second)
    /// TODO, SMPTE type
    HoursMS { value: u8 },
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
