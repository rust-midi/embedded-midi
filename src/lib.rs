//! *Midi driver on top of embedded hal serial communications*
//!
#![no_std]
#[warn(missing_debug_implementations, missing_docs)]
use embedded_hal::serial;
use nb::block;

mod midi;
mod parser;

use core::fmt::Debug;
pub use midi::{Channel, Control, MidiEvent, Note, Program};
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

    pub fn read(&mut self) -> nb::Result<MidiEvent, E> {
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

    pub fn write(&mut self, event: MidiEvent) -> Result<(), E> {
        match event {
            MidiEvent::NoteOn {
                channel,
                note,
                velocity,
            } => {
                let channelnum: u8 = channel.into();
                block!(self.tx.write(0x90u8 + channelnum))?;
                block!(self.tx.write(note.into()))?;
                block!(self.tx.write(velocity.into()))?;
            }
            MidiEvent::NoteOff {
                channel,
                note,
                velocity,
            } => {
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
