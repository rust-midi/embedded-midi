type MidiResult<T, E> = core::result::Result<T, MidiError<E>>;

pub enum MidiError<E> {
    Serial(E),
    ParseError,
}
