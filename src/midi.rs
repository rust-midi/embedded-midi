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
        control: Control,
        value: u8,
    },
    ProgramChange {
        channel: Channel,
        program: u8,
    },
    PitchBend {
        channel: Channel,
        value: Value14,
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
