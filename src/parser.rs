use crate::{Channel, Control, MidiEvent, Note};

pub struct MidiParser {
    state: MidiParserState,
}

enum MidiParserState {
    Idle,
    NoteOnRecvd { channel: Channel },
    NoteOnNoteRecvd { channel: Channel, note: Note },

    NoteOffRecvd { channel: Channel },
    NoteOffNoteRecvd { channel: Channel, note: Note },

    KeyPressureRecvd { channel: Channel },
    KeyPressureNoteRecvd { channel: Channel, note: Note },

    ControlChangeRecvd { channel: Channel },
    ControlChangeControlRecvd { channel: Channel, control: Control },

    ProgramChangeRecvd { channel: Channel },

    ChannelPressureRecvd { channel: Channel },

    PitchBendRecvd { channel: Channel },
    PitchBendFirstByteRecvd { channel: Channel, byte1: u8 },
}

/// Check if most significant bit is set which signifies a Midi status byte
fn is_status_byte(byte: u8) -> bool {
    byte & 0x80 == 0x80
}

/// Check if the byte corresponds to 0x11110xxx which signifies a system common message
fn is_system_common(byte: u8) -> bool {
    byte & 0xf8 == 0xf0
}

/// Check if the byte corresponds to 0x11111xxx which signifies a system realtime message
fn is_system_realtime(byte: u8) -> bool {
    byte & 0xf8 == 0xf8
}

/// Split the message and channel part of a channel voice message
fn split_message_and_channel(byte: u8) -> (u8, Channel) {
    (byte & 0xf0u8, (byte & 0x0fu8).into())
}

/// State machine for parsing Midi data, can be fed bytes one-by-one, and returns parsed Midi
/// messages whenever one is completed.
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
            if is_system_common(byte) {
                match byte {
                    0xf0 => {
                        // System exclusive
                        self.state = MidiParserState::Idle;
                        None
                    }
                    0xf1 => {
                        // Midi time code quarter frame
                        self.state = MidiParserState::Idle;
                        None
                    }
                    0xf2 => {
                        // Song position pointer
                        self.state = MidiParserState::Idle;
                        None
                    }
                    0xf3 => {
                        // Song select
                        self.state = MidiParserState::Idle;
                        None
                    }
                    0xf4 => {
                        // Undefined
                        self.state = MidiParserState::Idle;
                        None
                    }
                    0xf5 => {
                        // Undefined
                        self.state = MidiParserState::Idle;
                        None
                    }
                    0xf6 => {
                        // Tune request
                        self.state = MidiParserState::Idle;
                        None
                    }
                    0xf7 => {
                        // End of exclusive
                        self.state = MidiParserState::Idle;
                        None
                    }
                    _ => None,
                }
            } else if is_system_realtime(byte) {
                match byte {
                    0xf8 => Some(MidiEvent::TimingClock),
                    0xf9 => None, // Reserved
                    0xfa => Some(MidiEvent::Start),
                    0xfb => Some(MidiEvent::Continue),
                    0xfc => Some(MidiEvent::Stop),
                    0xfd => None, // Reserved
                    0xfe => Some(MidiEvent::ActiveSensing),
                    0xff => Some(MidiEvent::Reset),
                    _ => None,
                }
            } else {
                // Channel voice message

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
                    0xA0 => {
                        self.state = MidiParserState::KeyPressureRecvd { channel };
                        None
                    }
                    0xB0 => {
                        self.state = MidiParserState::ControlChangeRecvd { channel };
                        None
                    }
                    0xC0 => {
                        self.state = MidiParserState::ProgramChangeRecvd { channel };
                        None
                    }
                    0xD0 => {
                        self.state = MidiParserState::ChannelPressureRecvd { channel };
                        None
                    }
                    0xE0 => {
                        self.state = MidiParserState::PitchBendRecvd { channel };
                        None
                    }
                    _ => None,
                }
            }
        } else {
            match self.state {
                MidiParserState::NoteOffRecvd { channel } => {
                    self.state = MidiParserState::NoteOffNoteRecvd {
                        channel,
                        note: byte.into(),
                    };
                    None
                }
                MidiParserState::NoteOffNoteRecvd { channel, note } => {
                    self.state = MidiParserState::NoteOffRecvd { channel };
                    Some(MidiEvent::NoteOff {
                        channel,
                        note,
                        velocity: byte.into(),
                    })
                }

                MidiParserState::NoteOnRecvd { channel } => {
                    self.state = MidiParserState::NoteOnNoteRecvd {
                        channel,
                        note: byte.into(),
                    };
                    None
                }
                MidiParserState::NoteOnNoteRecvd { channel, note } => {
                    self.state = MidiParserState::NoteOnRecvd { channel };
                    Some(MidiEvent::NoteOn {
                        channel,
                        note,
                        velocity: byte.into(),
                    })
                }

                MidiParserState::KeyPressureRecvd { channel } => {
                    self.state = MidiParserState::KeyPressureNoteRecvd {
                        channel,
                        note: byte.into(),
                    };
                    None
                }
                MidiParserState::KeyPressureNoteRecvd { channel, note } => {
                    self.state = MidiParserState::KeyPressureRecvd { channel };
                    Some(MidiEvent::KeyPressure {
                        channel,
                        note,
                        value: byte.into(),
                    })
                }

                MidiParserState::ControlChangeRecvd { channel } => {
                    self.state = MidiParserState::ControlChangeControlRecvd {
                        channel,
                        control: byte.into(),
                    };
                    None
                }
                MidiParserState::ControlChangeControlRecvd { channel, control } => {
                    self.state = MidiParserState::ControlChangeRecvd { channel };
                    Some(MidiEvent::ControlChange {
                        channel,
                        control,
                        value: byte.into(),
                    })
                }

                MidiParserState::ProgramChangeRecvd { channel } => Some(MidiEvent::ProgramChange {
                    channel,
                    program: byte.into(),
                }),

                MidiParserState::ChannelPressureRecvd { channel } => {
                    Some(MidiEvent::ChannelPressure {
                        channel,
                        value: byte.into(),
                    })
                }

                MidiParserState::PitchBendRecvd { channel } => {
                    self.state = MidiParserState::PitchBendFirstByteRecvd {
                        channel,
                        byte1: byte,
                    };
                    None
                }
                MidiParserState::PitchBendFirstByteRecvd { channel, byte1 } => {
                    self.state = MidiParserState::PitchBendRecvd { channel };
                    Some(MidiEvent::PitchBendChange {
                        channel,
                        value: (byte1, byte).into(),
                    })
                }
                _ => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use std::vec::Vec;

    #[test]
    fn should_parse_status_byte() {
        assert!(is_status_byte(0x80u8));
        assert!(is_status_byte(0x94u8));
        assert!(!is_status_byte(0x00u8));
        assert!(!is_status_byte(0x78u8));
    }

    #[test]
    fn should_parse_system_common() {
        assert!(is_system_common(0xf0));
        assert!(is_system_common(0xf4));
        assert!(!is_system_common(0xf8));
        assert!(!is_system_common(0x78));
    }

    #[test]
    fn should_parse_system_realtime() {
        assert!(is_system_realtime(0xf8));
        assert!(is_system_realtime(0xfA));
        assert!(!is_system_realtime(0xf7));
        assert!(!is_system_realtime(0x78));
    }

    #[test]
    fn should_split_message_and_channel() {
        let (message, channel) = split_message_and_channel(0x91u8);
        assert_eq!(message, 0x90u8);
        assert_eq!(channel, 1.into());
    }

    #[test]
    fn should_parse_note_off() {
        MidiParser::new().assert_result(
            &[0x82, 0x76, 0x34],
            &[MidiEvent::NoteOff {
                channel: 2.into(),
                note: 0x76.into(),
                velocity: 0x34.into(),
            }],
        );
    }

    #[test]
    fn should_handle_note_off_running_state() {
        MidiParser::new().assert_result(
            &[
                0x82, 0x76, 0x34, // First note_off
                0x33, 0x65, // Second note_off without status byte
            ],
            &[
                MidiEvent::NoteOff {
                    channel: 2.into(),
                    note: 0x76.into(),
                    velocity: 0x34.into(),
                },
                MidiEvent::NoteOff {
                    channel: 2.into(),
                    note: 0x33.into(),
                    velocity: 0x65.into(),
                },
            ],
        );
    }

    #[test]
    fn should_parse_note_on() {
        MidiParser::new().assert_result(
            &[0x91, 0x04, 0x34],
            &[MidiEvent::NoteOn {
                channel: 1.into(),
                note: 4.into(),
                velocity: 0x34.into(),
            }],
        );
    }

    #[test]
    fn should_handle_note_on_running_state() {
        MidiParser::new().assert_result(
            &[
                0x92, 0x76, 0x34, // First note_on
                0x33, 0x65, // Second note on without status byte
            ],
            &[
                MidiEvent::NoteOn {
                    channel: 2.into(),
                    note: 0x76.into(),
                    velocity: 0x34.into(),
                },
                MidiEvent::NoteOn {
                    channel: 2.into(),
                    note: 0x33.into(),
                    velocity: 0x65.into(),
                },
            ],
        );
    }

    #[test]
    fn should_parse_keypressure() {
        MidiParser::new().assert_result(
            &[0xAA, 0x13, 0x34],
            &[MidiEvent::KeyPressure {
                channel: 10.into(),
                note: 0x13.into(),
                value: 0x34.into(),
            }],
        );
    }

    #[test]
    fn should_handle_keypressure_running_state() {
        MidiParser::new().assert_result(
            &[
                0xA8, 0x77, 0x03, // First key_pressure
                0x14, 0x56, // Second key_pressure without status byte
            ],
            &[
                MidiEvent::KeyPressure {
                    channel: 8.into(),
                    note: 0x77.into(),
                    value: 0x03.into(),
                },
                MidiEvent::KeyPressure {
                    channel: 8.into(),
                    note: 0x14.into(),
                    value: 0x56.into(),
                },
            ],
        );
    }

    #[test]
    fn should_parse_control_change() {
        MidiParser::new().assert_result(
            &[0xB2, 0x76, 0x34],
            &[MidiEvent::ControlChange {
                channel: 2.into(),
                control: 0x76.into(),
                value: 0x34.into(),
            }],
        );
    }

    #[test]
    fn should_parse_control_change_running_state() {
        MidiParser::new().assert_result(
            &[
                0xb3, 0x3C, 0x18, // First control change
                0x43, 0x01, // Second control change without status byte
            ],
            &[
                MidiEvent::ControlChange {
                    channel: 3.into(),
                    control: 0x3C.into(),
                    value: 0x18.into(),
                },
                MidiEvent::ControlChange {
                    channel: 3.into(),
                    control: 0x43.into(),
                    value: 0x01.into(),
                },
            ],
        );
    }

    #[test]
    fn should_parse_program_change() {
        MidiParser::new().assert_result(
            &[0xC9, 0x15],
            &[MidiEvent::ProgramChange {
                channel: 9.into(),
                program: 0x15.into(),
            }],
        );
    }

    #[test]
    fn should_parse_program_change_running_state() {
        MidiParser::new().assert_result(
            &[
                0xC3, 0x67, // First program change
                0x01, // Second program change without status byte
            ],
            &[
                MidiEvent::ProgramChange {
                    channel: 3.into(),
                    program: 0x67.into(),
                },
                MidiEvent::ProgramChange {
                    channel: 3.into(),
                    program: 0x01.into(),
                },
            ],
        );
    }

    #[test]
    fn should_parse_channel_pressure() {
        MidiParser::new().assert_result(
            &[0xDD, 0x37],
            &[MidiEvent::ChannelPressure {
                channel: 13.into(),
                value: 0x37.into(),
            }],
        );
    }

    #[test]
    fn should_parse_channel_pressure_running_state() {
        MidiParser::new().assert_result(
            &[
                0xD6, 0x77, // First channel pressure
                0x43, // Second channel pressure without status byte
            ],
            &[
                MidiEvent::ChannelPressure {
                    channel: 6.into(),
                    value: 0x77.into(),
                },
                MidiEvent::ChannelPressure {
                    channel: 6.into(),
                    value: 0x43.into(),
                },
            ],
        );
    }

    #[test]
    fn should_parse_pitchbend() {
        MidiParser::new().assert_result(
            &[0xE8, 0x14, 0x56],
            &[MidiEvent::PitchBendChange {
                channel: 8.into(),
                value: (0x14, 0x56).into(),
            }],
        );
    }

    #[test]
    fn should_parse_pitchbend_running_state() {
        MidiParser::new().assert_result(
            &[
                0xE3, 0x3C, 0x18, // First pitchbend
                0x43, 0x01, // Second pitchbend without status byte
            ],
            &[
                MidiEvent::PitchBendChange {
                    channel: 3.into(),
                    value: (0x3C, 0x18).into(),
                },
                MidiEvent::PitchBendChange {
                    channel: 3.into(),
                    value: (0x43, 0x01).into(),
                },
            ],
        );
    }

    #[test]
    fn should_parse_timingclock_message() {
        MidiParser::new().assert_result(&[0xf8], &[MidiEvent::TimingClock]);
    }

    #[test]
    fn should_parse_timingclock_message_as_realtime() {
        MidiParser::new().assert_result(
            &[
                0xD6, // Start channel pressure event
                0xf8, // interupt with midi timing clock
                0x77, // Finish channel pressure
            ],
            &[
                MidiEvent::TimingClock,
                MidiEvent::ChannelPressure {
                    channel: 6.into(),
                    value: 0x77.into(),
                },
            ],
        );
    }

    #[test]
    fn should_parse_start_message() {
        MidiParser::new().assert_result(&[0xfa], &[MidiEvent::Start]);
    }

    #[test]
    fn should_parse_start_message_as_realtime() {
        MidiParser::new().assert_result(
            &[
                0xD6, // Start channel pressure event
                0xfa, // interupt with start
                0x77, // Finish channel pressure
            ],
            &[
                MidiEvent::Start,
                MidiEvent::ChannelPressure {
                    channel: 6.into(),
                    value: 0x77.into(),
                },
            ],
        );
    }

    #[test]
    fn should_parse_continue_message() {
        MidiParser::new().assert_result(&[0xfb], &[MidiEvent::Continue]);
    }

    #[test]
    fn should_parse_continue_message_as_realtime() {
        MidiParser::new().assert_result(
            &[
                0xD6, // Start channel pressure event
                0xfb, // interupt with continue
                0x77, // Finish channel pressure
            ],
            &[
                MidiEvent::Continue,
                MidiEvent::ChannelPressure {
                    channel: 6.into(),
                    value: 0x77.into(),
                },
            ],
        );
    }

    #[test]
    fn should_parse_stop_message() {
        MidiParser::new().assert_result(&[0xfc], &[MidiEvent::Stop]);
    }

    #[test]
    fn should_parse_stop_message_as_realtime() {
        MidiParser::new().assert_result(
            &[
                0xD6, // Start channel pressure event
                0xfc, // interupt with stop
                0x77, // Finish channel pressure
            ],
            &[
                MidiEvent::Stop,
                MidiEvent::ChannelPressure {
                    channel: 6.into(),
                    value: 0x77.into(),
                },
            ],
        );
    }

    #[test]
    fn should_parse_activesensing_message() {
        MidiParser::new().assert_result(&[0xfe], &[MidiEvent::ActiveSensing]);
    }

    #[test]
    fn should_parse_activesensing_message_as_realtime() {
        MidiParser::new().assert_result(
            &[
                0xD6, // Start channel pressure event
                0xfe, // interupt with activesensing
                0x77, // Finish channel pressure
            ],
            &[
                MidiEvent::ActiveSensing,
                MidiEvent::ChannelPressure {
                    channel: 6.into(),
                    value: 0x77.into(),
                },
            ],
        );
    }

    #[test]
    fn should_parse_reset_message() {
        MidiParser::new().assert_result(&[0xff], &[MidiEvent::Reset]);
    }

    #[test]
    fn should_parse_reset_message_as_realtime() {
        MidiParser::new().assert_result(
            &[
                0xD6, // Start channel pressure event
                0xff, // interupt with reset
                0x77, // Finish channel pressure
            ],
            &[
                MidiEvent::Reset,
                MidiEvent::ChannelPressure {
                    channel: 6.into(),
                    value: 0x77.into(),
                },
            ],
        );
    }

    #[test]
    fn should_ignore_incomplete_messages() {
        MidiParser::new().assert_result(
            &[
                0x92, 0x1b, // Start note off message
                0x82, 0x76, 0x34, // continue with a complete note on message
            ],
            &[MidiEvent::NoteOff {
                channel: 2.into(),
                note: 0x76.into(),
                velocity: 0x34.into(),
            }],
        );
    }

    impl MidiParser {
        /// Test helper function, asserts if a slice of bytes parses to some set of midi events
        fn assert_result(&mut self, bytes: &[u8], expected_events: &[MidiEvent]) {
            let events: Vec<MidiEvent> = bytes
                .into_iter()
                .filter_map(|byte| self.parse_byte(*byte))
                .collect();

            assert_eq!(expected_events, events.as_slice());
        }
    }
}
