use crate::MidiEvent;

pub struct MidiParser {
    state: MidiParserState,
}

enum MidiParserState {
    Idle,
    NoteOnRecvd { channel: u8 },
    NoteOnNoteRecvd { channel: u8, note: u8 },

    NoteOffRecvd { channel: u8 },
    NoteOffNoteRecvd { channel: u8, note: u8 },
}

impl MidiParser {
    /// Initialize midiparser state
    pub fn new() -> Self {
        MidiParser {
            state: MidiParserState::Idle,
        }
    }

    /// Parse midi event byte by byte. Call this whenever a byte is received. When a midi-event is
    /// completed it is returned, otherwise this method updates the internal midiparser state and
    /// and returns none.
    pub fn parse_byte(&mut self, byte: u8) -> Option<MidiEvent> {
        match self.state {
            MidiParserState::Idle => {
                // expect the start of a new message
                let message = byte & 0xf0u8;
                let channel = byte & 0x0fu8;

                match message {
                    0x80 => {
                        self.state = MidiParserState::NoteOffRecvd { channel };
                        None
                    }
                    0x90 => {
                        self.state = MidiParserState::NoteOnRecvd { channel };
                        None
                    }
                    _ => None,
                }
            }

            MidiParserState::NoteOnRecvd { channel } => {
                self.state = MidiParserState::NoteOnNoteRecvd {
                    channel,
                    note: byte,
                };
                None
            }
            MidiParserState::NoteOnNoteRecvd { channel, note } => {
                self.state = MidiParserState::Idle;
                Some(MidiEvent::note_on(channel.into(), note.into(), byte.into()))
            }

            MidiParserState::NoteOffRecvd { channel } => {
                self.state = MidiParserState::NoteOffNoteRecvd {
                    channel,
                    note: byte,
                };
                None
            }
            MidiParserState::NoteOffNoteRecvd { channel, note } => {
                self.state = MidiParserState::Idle;
                Some(MidiEvent::note_off(
                    channel.into(),
                    note.into(),
                    byte.into(),
                ))
            }
        }
    }
}
