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

    pub fn release(self) -> TX {
        self.tx
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

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use embedded_hal_mock::serial;
    use std::vec::Vec;

    fn verify_writes(messages: &[MidiMessage], bytes: &[u8]) {
        let expectations: Vec<serial::Transaction<u8>> = bytes
            .into_iter()
            .map(|byte| serial::Transaction::write(*byte))
            .collect();
        let serial = serial::Mock::new(&expectations);
        let mut midi_out = MidiOut::new(serial);
        for message in messages {
            midi_out.write(&message).unwrap();
        }
        let mut serial = midi_out.release();
        serial.done();
    }

    #[test]
    fn note_on_should_write_successfully() {
        verify_writes(
            &[MidiMessage::NoteOn(0x02.into(), 0x76.into(), 0x34.into())],
            &[0x92, 0x76, 0x34],
        );
    }

    #[test]
    fn note_on_second_note_should_skip_status() {
        verify_writes(
            &[
                MidiMessage::NoteOn(0x02.into(), 0x76.into(), 0x34.into()),
                MidiMessage::NoteOn(0x02.into(), 0x33.into(), 0x65.into()),
            ],
            &[0x92, 0x76, 0x34, 0x33, 0x65],
        );
    }
    #[test]
    fn note_on_second_note_different_channel_should_not_skip_status() {
        verify_writes(
            &[
                MidiMessage::NoteOn(0x02.into(), 0x76.into(), 0x34.into()),
                MidiMessage::NoteOn(0x03.into(), 0x33.into(), 0x65.into()),
            ],
            &[0x92, 0x76, 0x34, 0x93, 0x33, 0x65],
        );
    }
    #[test]
    fn note_on_note_off_should_not_skip_status() {
        verify_writes(
            &[
                MidiMessage::NoteOn(0x02.into(), 0x76.into(), 0x34.into()),
                MidiMessage::NoteOff(0x02.into(), 0x33.into(), 0x65.into()),
            ],
            &[0x92, 0x76, 0x34, 0x82, 0x33, 0x65],
        );
    }
    #[test]
    fn note_off_should_write_successfully() {
        verify_writes(
            &[MidiMessage::NoteOff(0x02.into(), 0x76.into(), 0x34.into())],
            &[0x82, 0x76, 0x34],
        );
    }
    #[test]
    fn key_pressure_should_write_successfully() {
        verify_writes(
            &[MidiMessage::KeyPressure(
                0x02.into(),
                0x76.into(),
                0x34.into(),
            )],
            &[0xA2, 0x76, 0x34],
        );
    }
    #[test]
    fn control_change_should_write_successfully() {
        verify_writes(
            &[MidiMessage::ControlChange(
                0x02.into(),
                0x76.into(),
                0x34.into(),
            )],
            &[0xB2, 0x76, 0x34],
        );
    }
    #[test]
    fn program_change_should_write_successfully() {
        verify_writes(
            &[MidiMessage::ProgramChange(0x02.into(), 0x76.into())],
            &[0xC2, 0x76],
        );
    }
    #[test]
    fn channel_pressure_should_write_successfully() {
        verify_writes(
            &[MidiMessage::ChannelPressure(0x02.into(), 0x76.into())],
            &[0xD2, 0x76],
        );
    }
    #[test]
    fn pitch_bend_should_write_successfully() {
        verify_writes(
            &[MidiMessage::PitchBendChange(
                0x02.into(),
                (0x76, 0x34).into(),
            )],
            &[0xE2, 0x76, 0x34],
        );
    }
    #[test]
    fn quarter_frame_should_write_successfully() {
        verify_writes(&[MidiMessage::QuarterFrame(0x76.into())], &[0xF1, 0x76]);
    }
    #[test]
    fn song_position_pointer_should_write_successfully() {
        verify_writes(
            &[MidiMessage::SongPositionPointer((0x76, 0x34).into())],
            &[0xF2, 0x76, 0x34],
        );
    }
    #[test]
    fn song_select_should_write_successfully() {
        verify_writes(&[MidiMessage::SongSelect(0x76.into())], &[0xF3, 0x76]);
    }
    #[test]
    fn tune_request_should_write_successfully() {
        verify_writes(&[MidiMessage::TuneRequest], &[0xF6]);
    }
    #[test]
    fn timing_clock_should_write_successfully() {
        verify_writes(&[MidiMessage::TimingClock], &[0xF8]);
    }
    #[test]
    fn start_should_write_successfully() {
        verify_writes(&[MidiMessage::Start], &[0xFA]);
    }
    #[test]
    fn continue_should_write_successfully() {
        verify_writes(&[MidiMessage::Continue], &[0xFB]);
    }
    #[test]
    fn stop_should_write_successfully() {
        verify_writes(&[MidiMessage::Stop], &[0xFC]);
    }
    #[test]
    fn active_sensing_should_write_successfully() {
        verify_writes(&[MidiMessage::ActiveSensing], &[0xFE]);
    }
    #[test]
    fn reset_should_write_successfully() {
        verify_writes(&[MidiMessage::Reset], &[0xFF]);
    }
}
