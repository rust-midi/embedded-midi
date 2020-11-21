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
    last_status: Option<u8>,
}

impl<TX, E> MidiOut<TX>
where
    TX: serial::Write<u8, Error = E>,
    E: Debug,
{
    pub fn new(tx: TX) -> Self {
        MidiOut {
            tx,
            last_status: None,
        }
    }

    pub fn write(&mut self, message: &MidiMessage) -> Result<(), E> {
        match message {
            &MidiMessage::NoteOn(channel, note, velocity) => {
                self.write_channel_message(0x90, channel.into(), &[note.into(), velocity.into()])?;
            }
            &MidiMessage::NoteOff(channel, note, velocity) => {
                self.write_channel_message(0x80, channel.into(), &[note.into(), velocity.into()])?;
            }
            &MidiMessage::KeyPressure(channel, note, value) => {
                self.write_channel_message(0xA0, channel.into(), &[note.into(), value.into()])?;
            }
            &MidiMessage::ControlChange(channel, control, value) => {
                self.write_channel_message(0xB0, channel.into(), &[control.into(), value.into()])?;
            }
            &MidiMessage::ProgramChange(channel, program) => {
                self.write_channel_message(0xC0, channel.into(), &[program.into()])?;
            }
            &MidiMessage::ChannelPressure(channel, value) => {
                self.write_channel_message(0xD0, channel.into(), &[value.into()])?;
            }
            &MidiMessage::PitchBendChange(channel, value) => {
                let (value_lsb, value_msb) = value.into();
                self.write_channel_message(0xE0, channel.into(), &[value_lsb, value_msb])?;
            }
            &MidiMessage::QuarterFrame(value) => {
                block!(self.tx.write(0xF1))?;
                block!(self.tx.write(value.into()))?;
                self.last_status = None;
            }
            &MidiMessage::SongPositionPointer(value) => {
                let (value_lsb, value_msb) = value.into();
                block!(self.tx.write(0xF2))?;
                block!(self.tx.write(value_lsb))?;
                block!(self.tx.write(value_msb))?;
                self.last_status = None;
            }
            &MidiMessage::SongSelect(value) => {
                block!(self.tx.write(0xF3))?;
                block!(self.tx.write(value.into()))?;
                self.last_status = None;
            }
            &MidiMessage::TuneRequest => {
                block!(self.tx.write(0xF6))?;
                self.last_status = None;
            }
            &MidiMessage::TimingClock => {
                block!(self.tx.write(0xF8))?;
            }
            &MidiMessage::Start => {
                block!(self.tx.write(0xFA))?;
            }
            &MidiMessage::Continue => {
                block!(self.tx.write(0xFB))?;
            }
            &MidiMessage::Stop => {
                block!(self.tx.write(0xFC))?;
            }
            &MidiMessage::ActiveSensing => {
                block!(self.tx.write(0xFE))?;
            }
            &MidiMessage::Reset => {
                block!(self.tx.write(0xFF))?;
            }
        }

        Ok(())
    }

    fn write_channel_message(&mut self, status_msb: u8, channel: u8, data: &[u8]) -> Result<(), E> {
        let status = status_msb + channel;
        // If the last command written had the same status/channel, the MIDI protocol allows us to
        // omit sending the status byte again.
        if self.last_status != Some(status) {
            block!(self.tx.write(status))?;
        }
        for byte in data {
            block!(self.tx.write(*byte))?;
        }
        self.last_status = Some(status);

        Ok(())
    }
}
