//! *Midi driver on top of embedded hal serial communications*
//!
#![no_std]
use embedded_hal::serial;
use nb::block;

mod error;
mod midi;

pub use midi::MidiEvent;

pub struct MidiIn<RX> {
    rx: RX,
}

impl<RX, E> MidiIn<RX>
where
    RX: serial::Read<u8, Error = E>,
{
    pub fn new(rx: RX) -> Self {
        MidiIn { rx }
    }

    // naive implementation, block until we've received a midi event we understand
    pub fn read(&mut self) -> Option<MidiEvent> {
        match block!(self.rx.read()) {
            Ok(byte) => {
                let message = byte & 0xf0u8;
                let channel = byte & 0x0fu8;

                if message == 0x90u8 {
                    Some(MidiEvent::note_on(
                        midi::Channel::from(channel),
                        midi::Note::from(block!(self.rx.read()).unwrap_or(0)),
                        midi::Velocity::from(block!(self.rx.read()).unwrap_or(0))))
                }
                else if message == 0x80 {
                    Some(MidiEvent::note_on(
                        midi::Channel::from(channel),
                        midi::Note::from(block!(self.rx.read()).unwrap_or(0)),
                        midi::Velocity::from(block!(self.rx.read()).unwrap_or(0))))
                }
                else { None }
            }
            _ => None,
        }
    }
}

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
