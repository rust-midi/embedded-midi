pub struct MidiIn<RX> {
    rx: RX,
}

impl<RX> MidiIn<RX> where RX: serial::read<u8> {}

struct MidiOut<TX> {
    tx: TX,
}

impl<TX> MidiOut<TX> where TX: serial::write<u8> {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
