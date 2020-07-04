use crate::{Channel, Control, MidiMessage, Note};

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

    QuarterFrameRecvd,

    SongPositionRecvd,
    SongPositionLsbRecvd { lsb: u8 },
}

/// Check if most significant bit is set which signifies a Midi status byte
fn is_status_byte(byte: u8) -> bool {
    byte & 0x80 == 0x80
}

/// Check if a byte corresponds to 0x1111xxxx which signifies either a system common or realtime message
fn is_system_message(byte: u8) -> bool {
    byte & 0xf0 == 0xf0
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
    pub fn parse_byte(&mut self, byte: u8) -> Option<MidiMessage> {
        if is_status_byte(byte) {
            if is_system_message(byte) {
                match byte {
                    // System common messages, these should reset parsing other messages
                    0xf0 => {
                        // System exclusive
                        self.state = MidiParserState::Idle;
                        None
                    }
                    0xf1 => {
                        // Midi time code quarter frame
                        self.state = MidiParserState::QuarterFrameRecvd;
                        None
                    }
                    0xf2 => {
                        // Song position pointer
                        self.state = MidiParserState::SongPositionRecvd;
                        None
                    }
                    0xf3 => {
                        // Song select
                        self.state = MidiParserState::Idle;
                        None
                    }
                    0xf6 => {
                        // Tune request
                        self.state = MidiParserState::Idle;
                        Some(MidiMessage::TuneRequest)
                    }
                    0xf7 => {
                        // End of exclusive
                        self.state = MidiParserState::Idle;
                        Some(MidiMessage::EndOfExclusive)
                    }

                    // System realtime messages
                    0xf8 => Some(MidiMessage::TimingClock),
                    0xf9 => None, // Reserved
                    0xfa => Some(MidiMessage::Start),
                    0xfb => Some(MidiMessage::Continue),
                    0xfc => Some(MidiMessage::Stop),
                    0xfd => None, // Reserved
                    0xfe => Some(MidiMessage::ActiveSensing),
                    0xff => Some(MidiMessage::Reset),

                    _ => {
                        // Undefined messages like 0xf4 and should end up here
                        self.state = MidiParserState::Idle;
                        None
                    }
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
                    Some(MidiMessage::NoteOff {
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
                    Some(MidiMessage::NoteOn {
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
                    Some(MidiMessage::KeyPressure {
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
                    Some(MidiMessage::ControlChange {
                        channel,
                        control,
                        value: byte.into(),
                    })
                }

                MidiParserState::ProgramChangeRecvd { channel } => {
                    Some(MidiMessage::ProgramChange {
                        channel,
                        program: byte.into(),
                    })
                }

                MidiParserState::ChannelPressureRecvd { channel } => {
                    Some(MidiMessage::ChannelPressure {
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
                    Some(MidiMessage::PitchBendChange {
                        channel,
                        value: (byte1, byte).into(),
                    })
                }
                MidiParserState::SongPositionRecvd => {
                    self.state = MidiParserState::SongPositionLsbRecvd { lsb: byte };
                    None
                }
                MidiParserState::SongPositionLsbRecvd { lsb } => {
                    self.state = MidiParserState::SongPositionRecvd;
                    Some(MidiMessage::SongPositionPointer {
                        pointer: (lsb, byte).into(),
                    })
                }
                MidiParserState::QuarterFrameRecvd => Some(MidiMessage::QuarterFrame {
                    frame_data: byte.into(),
                }),
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
    fn should_parse_system_message() {
        assert!(is_system_message(0xf0));
        assert!(is_system_message(0xf4));
        assert!(!is_system_message(0x0f));
        assert!(!is_system_message(0x77));
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
            &[MidiMessage::NoteOff {
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
                MidiMessage::NoteOff {
                    channel: 2.into(),
                    note: 0x76.into(),
                    velocity: 0x34.into(),
                },
                MidiMessage::NoteOff {
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
            &[MidiMessage::NoteOn {
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
                MidiMessage::NoteOn {
                    channel: 2.into(),
                    note: 0x76.into(),
                    velocity: 0x34.into(),
                },
                MidiMessage::NoteOn {
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
            &[MidiMessage::KeyPressure {
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
                MidiMessage::KeyPressure {
                    channel: 8.into(),
                    note: 0x77.into(),
                    value: 0x03.into(),
                },
                MidiMessage::KeyPressure {
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
            &[MidiMessage::ControlChange {
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
                MidiMessage::ControlChange {
                    channel: 3.into(),
                    control: 0x3C.into(),
                    value: 0x18.into(),
                },
                MidiMessage::ControlChange {
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
            &[MidiMessage::ProgramChange {
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
                MidiMessage::ProgramChange {
                    channel: 3.into(),
                    program: 0x67.into(),
                },
                MidiMessage::ProgramChange {
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
            &[MidiMessage::ChannelPressure {
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
                MidiMessage::ChannelPressure {
                    channel: 6.into(),
                    value: 0x77.into(),
                },
                MidiMessage::ChannelPressure {
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
            &[MidiMessage::PitchBendChange {
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
                MidiMessage::PitchBendChange {
                    channel: 3.into(),
                    value: (0x3C, 0x18).into(),
                },
                MidiMessage::PitchBendChange {
                    channel: 3.into(),
                    value: (0x43, 0x01).into(),
                },
            ],
        );
    }

    #[test]
    fn should_parse_quarter_frame() {
        MidiParser::new().assert_result(
            &[0xf1, 0x7f],
            &[MidiMessage::QuarterFrame {
                frame_data: 0x7f.into(),
            }],
        );
    }

    #[test]
    fn should_handle_quarter_frame_running_state() {
        MidiParser::new().assert_result(
            &[
                0xf1, 0x7f, // Send quarter frame
                0x56, // Only send data of next quarter frame
            ],
            &[
                MidiMessage::QuarterFrame {
                    frame_data: 0x7f.into(),
                },
                MidiMessage::QuarterFrame {
                    frame_data: 0x56.into(),
                },
            ],
        );
    }

    #[test]
    fn should_parse_song_position_pointer() {
        MidiParser::new().assert_result(
            &[0xf2, 0x7f, 0x68],
            &[MidiMessage::SongPositionPointer {
                pointer: (0x7f, 0x68).into(),
            }],
        );
    }

    #[test]
    fn should_handle_song_position_pointer_running_state() {
        MidiParser::new().assert_result(
            &[
                0xf2, 0x7f, 0x68, // Send song position pointer
                0x23, 0x7b, // Only send data of next song position pointer
            ],
            &[
                MidiMessage::SongPositionPointer {
                    pointer: (0x7f, 0x68).into(),
                },
                MidiMessage::SongPositionPointer {
                    pointer: (0x23, 0x7b).into(),
                },
            ],
        );
    }

    #[test]
    fn should_parse_tune_request() {
        MidiParser::new().assert_result(&[0xf6], &[MidiMessage::TuneRequest]);
    }

    #[test]
    fn should_interrupt_parsing_for_tune_request() {
        MidiParser::new().assert_result(
            &[
                0x92, 0x76, // start note_on message
                0xf6, // interrupt with tune request
                0x34, // finish note on, this should be ignored
            ],
            &[MidiMessage::TuneRequest],
        );
    }

    #[test]
    fn should_parse_end_exclusive() {
        MidiParser::new().assert_result(&[0xf7], &[MidiMessage::EndOfExclusive]);
    }

    #[test]
    fn should_interrupt_parsing_for_end_of_exclusive() {
        MidiParser::new().assert_result(
            &[
                0x92, 0x76, // start note_on message
                0xf7, // interrupt with end of exclusive
                0x34, // finish note on, this should be ignored
            ],
            &[MidiMessage::EndOfExclusive],
        );
    }

    #[test]
    fn should_interrupt_parsing_for_undefined_message() {
        MidiParser::new().assert_result(
            &[
                0x92, 0x76, // start note_on message
                0xf5, // interrupt with undefined message
                0x34, // finish note on, this should be ignored
            ],
            &[],
        );
    }

    #[test]
    fn should_parse_timingclock_message() {
        MidiParser::new().assert_result(&[0xf8], &[MidiMessage::TimingClock]);
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
                MidiMessage::TimingClock,
                MidiMessage::ChannelPressure {
                    channel: 6.into(),
                    value: 0x77.into(),
                },
            ],
        );
    }

    #[test]
    fn should_parse_start_message() {
        MidiParser::new().assert_result(&[0xfa], &[MidiMessage::Start]);
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
                MidiMessage::Start,
                MidiMessage::ChannelPressure {
                    channel: 6.into(),
                    value: 0x77.into(),
                },
            ],
        );
    }

    #[test]
    fn should_parse_continue_message() {
        MidiParser::new().assert_result(&[0xfb], &[MidiMessage::Continue]);
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
                MidiMessage::Continue,
                MidiMessage::ChannelPressure {
                    channel: 6.into(),
                    value: 0x77.into(),
                },
            ],
        );
    }

    #[test]
    fn should_parse_stop_message() {
        MidiParser::new().assert_result(&[0xfc], &[MidiMessage::Stop]);
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
                MidiMessage::Stop,
                MidiMessage::ChannelPressure {
                    channel: 6.into(),
                    value: 0x77.into(),
                },
            ],
        );
    }

    #[test]
    fn should_parse_activesensing_message() {
        MidiParser::new().assert_result(&[0xfe], &[MidiMessage::ActiveSensing]);
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
                MidiMessage::ActiveSensing,
                MidiMessage::ChannelPressure {
                    channel: 6.into(),
                    value: 0x77.into(),
                },
            ],
        );
    }

    #[test]
    fn should_parse_reset_message() {
        MidiParser::new().assert_result(&[0xff], &[MidiMessage::Reset]);
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
                MidiMessage::Reset,
                MidiMessage::ChannelPressure {
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
            &[MidiMessage::NoteOff {
                channel: 2.into(),
                note: 0x76.into(),
                velocity: 0x34.into(),
            }],
        );
    }

    impl MidiParser {
        /// Test helper function, asserts if a slice of bytes parses to some set of midi events
        fn assert_result(&mut self, bytes: &[u8], expected_events: &[MidiMessage]) {
            let events: Vec<MidiMessage> = bytes
                .into_iter()
                .filter_map(|byte| self.parse_byte(*byte))
                .collect();

            assert_eq!(expected_events, events.as_slice());
        }
    }
}
