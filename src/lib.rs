//! *Midi driver on top of embedded hal serial communications*

#![no_std]
#![warn(missing_debug_implementations)]
use core::fmt::Debug;
use embedded_hal_nb::serial;
use midi_convert::midi_types::MidiMessage;

use midi_convert::{
    parse::MidiParser,
    render::{MidiRenderer, MidiTransport},
};
use nb::block;

pub use midi_convert::midi_types;

#[derive(Debug)]
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

        match self.parser.parse(byte) {
            Some(event) => Ok(event),
            None => Err(nb::Error::WouldBlock),
        }
    }
}

#[derive(Debug)]
struct SerialTransport<TX>(TX);

impl<TX, E> MidiTransport for SerialTransport<TX>
where
    TX: serial::Write<u8, Error = E>,
    E: Debug,
{
    type Error = E;

    fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        bytes
            .iter()
            .try_for_each(|value| block!(self.0.write(*value)))
    }
}

#[derive(Debug)]
pub struct MidiOut<TX> {
    renderer: MidiRenderer<SerialTransport<TX>, true>,
}

impl<TX, E> MidiOut<TX>
where
    TX: serial::Write<u8, Error = E>,
    E: Debug,
{
    pub fn new(tx: TX) -> Self {
        MidiOut {
            renderer: MidiRenderer::new(SerialTransport(tx)),
        }
    }

    pub fn release(self) -> TX {
        self.renderer.release().0
    }

    pub fn write(&mut self, message: &MidiMessage) -> Result<(), E> {
        self.renderer.render(message)
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use embedded_hal_mock::eh1::serial;
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
    fn should_write_midi_message() {
        verify_writes(
            &[MidiMessage::NoteOn(0x02.into(), 0x76.into(), 0x34.into())],
            &[0x92, 0x76, 0x34],
        );
    }

    #[test]
    fn should_use_running_status() {
        verify_writes(
            &[
                MidiMessage::NoteOn(0x02.into(), 0x76.into(), 0x34.into()),
                MidiMessage::NoteOn(0x02.into(), 0x33.into(), 0x65.into()),
            ],
            &[0x92, 0x76, 0x34, 0x33, 0x65],
        );
    }
}
