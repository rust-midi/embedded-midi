//! *Midi driver on top of embedded hal serial communications*
//!
#![no_std]
#[warn(missing_debug_implementations, missing_docs)]
mod parser;

use core::fmt::Debug;
use embedded_hal::serial;
pub use midi_types::{Channel, Control, MidiMessage, Note, Program};
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
                block!(self.tx.write(0x90 + channelnum))?;
                block!(self.tx.write(note.into()))?;
                block!(self.tx.write(velocity.into()))?;
            }
            MidiMessage::NoteOff(channel, note, velocity) => {
                let channelnum: u8 = channel.into();
                block!(self.tx.write(0x80 + channelnum))?;
                block!(self.tx.write(note.into()))?;
                block!(self.tx.write(velocity.into()))?;
            }
            MidiMessage::KeyPressure(channel, note, value) => {
                let channelnum: u8 = channel.into();
                block!(self.tx.write(0xA0 + channelnum))?;
                block!(self.tx.write(note.into()))?;
                block!(self.tx.write(value.into()))?;
            }
            MidiMessage::ControlChange(channel, control, value) => {
                let channelnum: u8 = channel.into();
                block!(self.tx.write(0xB0 + channelnum))?;
                block!(self.tx.write(control.into()))?;
                block!(self.tx.write(value.into()))?;
            }
            MidiMessage::ProgramChange(channel, program) => {
                let channelnum: u8 = channel.into();
                block!(self.tx.write(0xC0 + channelnum))?;
                block!(self.tx.write(program.into()))?;
            }
            MidiMessage::ChannelPressure(channel, value) => {
                let channelnum: u8 = channel.into();
                block!(self.tx.write(0xD0 + channelnum))?;
                block!(self.tx.write(value.into()))?;
            }
            MidiMessage::PitchBendChange(channel, value) => {
                let channelnum: u8 = channel.into();
                let (first_byte, second_byte) = value.into();
                block!(self.tx.write(0xE0 + channelnum))?;
                block!(self.tx.write(first_byte))?;
                block!(self.tx.write(second_byte))?;
            }
            MidiMessage::QuarterFrame(value) => {
                block!(self.tx.write(0xF1))?;
                block!(self.tx.write(value.into()))?;
            }
            MidiMessage::SongPositionPointer(value) => {
                let (first_byte, second_byte) = value.into();
                block!(self.tx.write(0xF2))?;
                block!(self.tx.write(first_byte))?;
                block!(self.tx.write(second_byte))?;
            }
            MidiMessage::SongSelect(value) => {
                block!(self.tx.write(0xF3))?;
                block!(self.tx.write(value.into()))?;
            }
            MidiMessage::TuneRequest => {
                block!(self.tx.write(0xF6))?;
            }
            MidiMessage::TimingClock => {
                block!(self.tx.write(0xF8))?;
            }
            MidiMessage::Start => {
                block!(self.tx.write(0xFA))?;
            }
            MidiMessage::Continue => {
                block!(self.tx.write(0xFB))?;
            }
            MidiMessage::Stop => {
                block!(self.tx.write(0xFC))?;
            }
            MidiMessage::ActiveSensing => {
                block!(self.tx.write(0xFE))?;
            }
            MidiMessage::Reset => {
                block!(self.tx.write(0xFF))?;
            }
        }

        Ok(())
    }
}
