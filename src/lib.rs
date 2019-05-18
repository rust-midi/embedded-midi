//! *Midi driver on top of embedded hal serial communications*
//!
#![no_std]
use embedded_hal::serial;

mod error;
mod midi;

pub struct MidiIn<RX> {
    rx: RX,
}

impl<RX> MidiIn<RX> where RX: serial::Read<u8> {}

pub struct MidiOut<TX> {
    tx: TX,
}

impl<TX> MidiOut<TX> where TX: serial::Write<u8> {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
