//! *Midi driver on top of embedded hal serial communications*
//!
#![no_std]
use embedded_hal::serial;
use nb::block;

mod error;
mod midi;

use core::fmt::Debug;
pub use midi::{Channel, MidiEvent, Note, Velocity};

pub struct MidiIn<RX> {
    rx: RX,
}

impl<RX, E> MidiIn<RX>
where
    RX: serial::Read<u8, Error = E>,
    E: Debug,
{
    pub fn new(rx: RX) -> Self {
        MidiIn { rx }
    }

    // naive implementation, block until we've received a midi event we understand
    pub fn read(&mut self) -> Result<MidiEvent, E> {
        let mut result: Option<MidiEvent> = None;

        while result.is_none() {
            let byte = block!(self.rx.read())?;

            let message = byte & 0xf0u8;
            let channel = byte & 0x0fu8;

            if message == 0x90u8 {
                result = Some(MidiEvent::note_on(
                    midi::Channel::from(channel),
                    midi::Note::from(block!(self.rx.read())?),
                    midi::Velocity::from(block!(self.rx.read())?),
                ))
            } else if message == 0x80 {
                result = Some(MidiEvent::note_off(
                    midi::Channel::from(channel),
                    midi::Note::from(block!(self.rx.read())?),
                    midi::Velocity::from(block!(self.rx.read())?),
                ))
            }
        }

        Ok(result.unwrap())
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
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
