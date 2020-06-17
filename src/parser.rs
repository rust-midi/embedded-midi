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

    ControlChangeRecvd { channel: u8 },
    ControlChangeControllerRecvd { channel: u8, controller: u8 },
}

fn is_status_byte(byte: u8) -> bool {
    byte & 0x80 == 0x80
}

fn split_message_and_channel(byte: u8) -> (u8, u8) {
    (byte & 0xf0u8, byte & 0x0fu8)
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
        if is_status_byte(byte) {
            let (message, channel) = split_message_and_channel(byte);

            match message {
                0x80 => {
                    self.state = MidiParserState::NoteOffRecvd { channel };
                    None
                }
                0x90 => {
                    self.state = MidiParserState::NoteOnRecvd { channel };
                    None
                }
                0xB0 => {
                    self.state = MidiParserState::ControlChangeRecvd { channel };
                    None
                }
                _ => None,
            }
        } else {
            match self.state {
                MidiParserState::NoteOnRecvd { channel } => {
                    self.state = MidiParserState::NoteOnNoteRecvd {
                        channel,
                        note: byte,
                    };
                    None
                }
                MidiParserState::NoteOnNoteRecvd { channel, note } => {
                    self.state = MidiParserState::NoteOnRecvd { channel };
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
                    self.state = MidiParserState::NoteOffRecvd { channel };
                    Some(MidiEvent::note_off(
                        channel.into(),
                        note.into(),
                        byte.into(),
                    ))
                }
                MidiParserState::ControlChangeRecvd { channel } => {
                    self.state = MidiParserState::ControlChangeControllerRecvd {
                        channel,
                        controller: byte,
                    };
                    None
                }
                MidiParserState::ControlChangeControllerRecvd {
                    channel,
                    controller,
                } => Some(MidiEvent::controller_change(
                    channel.into(),
                    controller,
                    byte,
                )),
                _ => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_status_byte() {
        assert!(is_status_byte(0x80u8));
        assert!(is_status_byte(0x94u8));
        assert!(!is_status_byte(0x00u8));
        assert!(!is_status_byte(0x78u8));
    }

    #[test]
    fn should_split_message_and_channel() {
        let (message, channel) = split_message_and_channel(0x91u8);
        assert_eq!(message, 0x90u8);
        assert_eq!(channel, 1);
    }

    #[test]
    fn should_parse_note_on() {
        let mut parser = MidiParser::new();

        parser.parse_byte(0x91);
        parser.parse_byte(0x04);
        assert_eq!(
            parser.parse_byte(0x34),
            Some(MidiEvent::note_on(1.into(), 4.into(), 0x34.into()))
        );
    }

    #[test]
    fn should_parse_note_off() {
        let mut parser = MidiParser::new();

        parser.parse_byte(0x82);
        parser.parse_byte(0x76);
        assert_eq!(
            parser.parse_byte(0x34),
            Some(MidiEvent::note_off(2.into(), 0x76.into(), 0x34.into()))
        );
    }

    #[test]
    fn should_parse_control_change() {
        let mut parser = MidiParser::new();

        assert_eq!(parser.parse_byte(0xB4), None);
        assert_eq!(parser.parse_byte(0x14), None);
        assert_eq!(
            parser.parse_byte(0x65),
            Some(MidiEvent::controller_change(4.into(), 0x14, 0x65))
        );
    }

    #[test]
    fn should_ignore_incomplete_messages() {
        let mut parser = MidiParser::new();

        // start note off message but do not finish
        assert_eq!(parser.parse_byte(0x92), None);

        // continue with a complete note on message
        assert_eq!(parser.parse_byte(0x82), None);
        assert_eq!(parser.parse_byte(0x76), None);
        assert_eq!(
            parser.parse_byte(0x34),
            Some(MidiEvent::note_off(2.into(), 0x76.into(), 0x34.into()))
        );
    }

    #[test]
    fn should_handle_running_state() {
        let mut parser = MidiParser::new();

        // send note-on message
        assert_eq!(parser.parse_byte(0x82), None);
        assert_eq!(parser.parse_byte(0x76), None);
        assert_eq!(
            parser.parse_byte(0x34),
            Some(MidiEvent::note_off(2.into(), 0x76.into(), 0x34.into()))
        );

        // continue with a note on on the same channel by just sending the data bytes
        assert_eq!(parser.parse_byte(0x33), None);
        assert_eq!(
            parser.parse_byte(0x65),
            Some(MidiEvent::note_off(2.into(), 0x33.into(), 0x65.into()))
        );
    }
}
