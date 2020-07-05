//! *Midi driver on top of embedded hal serial communications*
//!
#![no_std]
#[warn(missing_debug_implementations, missing_docs)]
mod midi;
mod parser;

use core::fmt::Debug;
use embedded_hal::serial;
pub use midi::{Channel, Control, MidiMessage, Note, Program};
use nb::block;
pub use parser::MidiParser;

pub struct MidiIn<RX> {
    rx: RX,
    parser: MidiParser,
}

impl<RX, E> MidiIn<RX>
where
    RX: serial::Read<u8, Error = E>,
    E: Debug,
{
    pub fn new(rx: RX) -> Self {
        MidiIn {
            rx,
            parser: MidiParser::new(),
        }
    }

    pub fn read(&mut self) -> nb::Result<MidiMessage, E> {
        let byte = self.rx.read()?;

        match self.parser.parse_byte(byte) {
            Some(event) => Ok(event),
            None => Err(nb::Error::WouldBlock),
        }
    }
}

pub struct MidiOut<TX> {
    tx: TX,
}

impl<TX, E> MidiOut<TX>
where
    TX: serial::Write<u8, Error = E>,
    E: Debug,
{
    pub fn new(tx: TX) -> Self {
        MidiOut { tx }
    }

    pub fn write(&mut self, event: MidiMessage) -> Result<(), E> {
        match event {
            MidiMessage::NoteOn(channel, note, velocity) => {
                let channelnum: u8 = channel.into();
                block!(self.tx.write(0x90u8 + channelnum))?;
                block!(self.tx.write(note.into()))?;
                block!(self.tx.write(velocity.into()))?;
            }
            MidiMessage::NoteOff(channel, note, velocity) => {
                let channelnum: u8 = channel.into();
                block!(self.tx.write(0x80u8 + channelnum))?;
                block!(self.tx.write(note.into()))?;
                block!(self.tx.write(velocity.into()))?;
            }
            _ => (),
        }

        Ok(())
    }
}
