pub enum MidiError<E> {
    Serial(E),
    ParseError,
}
