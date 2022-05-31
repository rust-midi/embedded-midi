//! *Midi driver on top of embedded hal serial communications*
//!
#![no_std]
#[warn(missing_debug_implementations, missing_docs)]
use core::fmt::Debug;
use embedded_hal::serial;
pub use midi_types::{Channel, Control, MidiByteStreamParser, MidiMessage, Note, Program};
use nb::block;

pub struct MidiIn<RX> {
    rx: RX,
    parser: MidiByteStreamParser,
}

impl<RX, E> MidiIn<RX>
where
    RX: serial::Read<u8, Error = E>,
    E: Debug,
{
    pub fn new(rx: RX) -> Self {
        MidiIn {
            rx,
            parser: MidiByteStreamParser::new(),
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

    pub fn release(self) -> TX {
        self.tx
    }

    pub fn write(&mut self, message: &MidiMessage) -> Result<(), E> {
        let mut buf = [0u8; 3];
        if let Ok(l) = message.render(buf.as_mut()) {
            for b in buf.iter().take(l) {
                block!(self.tx.write(*b))?;
            }
        }

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

    /*
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
    */

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
