//! This module contains data types to represent the different messages that can be sent over MIDI.

/// An enum with variants for all possible Midi messages.
#[derive(Debug, PartialEq)]
pub enum MidiMessage {
    // Channel voice messages
    /// Note Off message
    NoteOff(Channel, Note, Value7),

    /// Note on message
    NoteOn(Channel, Note, Value7),

    /// KeyPressure message for polyphonic aftertouch
    KeyPressure(Channel, Note, Value7),

    /// Control change message
    ControlChange(Channel, Control, Value7),

    /// Program change message
    ProgramChange(Channel, Program),

    /// Channel pressure message for channel aftertouch
    ChannelPressure(Channel, Value7),

    /// Pitch bend message
    PitchBendChange(Channel, Value14),

    // System common messages
    /// System exclusive message starts
    // SystemExclusive {
    //     /// The system exclusive manufacturer id, this is either a 1 byte or 3 byte number
    //     manufacturer_id: u32,
    // },

    /// System exclusive data is received
    // SystemExclusiveData (Value7),

    /// Signals the end of the system exclusive block
    // EndOfExclusive,

    /// Midi time code quarter frame
    QuarterFrame(QuarterFrame),

    /// Set the song position pointer
    SongPositionPointer(Value14),

    /// Specifies which sequence or song is to be played
    SongSelect(Value7),

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
#[derive(Debug, PartialEq, Copy, Clone)]
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
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum QuarterFrameType {
    /// Frame number low nibble
    FramesLS,

    /// Frame count high nibble
    FramesMS,

    /// Seconds low nibble
    SecondsLS,

    /// Seconds high nibble
    SecondsMS,

    /// Minutes low nibble
    MinutesLS,

    /// Minutes high nibble
    MinutesMS,

    /// Hours low nibble
    HoursLS,

    /// Combined hours high nibble and smpte type (frames per second)
    HoursMS,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct QuarterFrame(u8);

impl QuarterFrame {
    pub fn frame_type(&self) -> QuarterFrameType {
        unimplemented!()
    }

    pub fn value(&self) -> u8 {
        unimplemented!()
    }

    pub fn smpte_type(&self) -> SmpteType {
        unimplemented!()
    }
}

impl From<u8> for QuarterFrame {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl Into<u8> for QuarterFrame {
    fn into(self) -> u8 {
        self.0
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
