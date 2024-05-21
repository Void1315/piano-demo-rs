use std::error::Error;

use super::*;

#[derive(Clone, Debug)]
pub struct MidiMessage {
    pub m_delta_time: Vec<MidiInt>, // 间隔时间
    pub m_status: MidiStatusByte,   // 状态
    pub m_ment_event: Event,        // 事件

    m_message_size: usize, // 消息的大小
}

impl MidiMessage {
    pub fn get_message_size(&self) -> usize {
        self.m_message_size
    }
}

impl MidiMessage {
    fn new(_: &[u8]) -> MidiMessage {
        MidiMessage {
            m_delta_time: Vec::new(),
            m_status: MidiStatusByte::empty(),
            m_ment_event: Event::None,
            m_message_size: 0,
        }
    }

    fn get_raw(&self) -> Vec<u8> {
        todo!()
    }

    pub fn parse(raw_data: &[u8], pre_status: &Option<MidiStatusByte>) -> Result<MidiMessage, Box<dyn Error>> {
        let mut midi_message = MidiMessage::new(raw_data);
        let mut cursor = 0;
        let mut delta_time: Vec<MidiInt> = Vec::new();
        // deleta_time 的解析方式是 midi变长int 每次读一个字节， 判断最高位是否为1 如果是1则继续读取下一个字节
        loop {
            if let Some(midi_delta_time) = MidiInt::from_bits(raw_data[cursor]) {
                delta_time.push(midi_delta_time);
                cursor += 1;
                if midi_delta_time.contains(MidiInt::flag) {
                    continue;
                } else {
                    break;
                }
            } else {
                return Err("delta time error".into());
            }
        }
        // 解析状态字节
        if let Some(status) = MidiStatusByte::from_bits(raw_data[cursor]) {
            // 如果status 的最高位为1 则表示这是一个状态字节
            // 如果不为1 则使用pre_status
            if !status.contains(MidiStatusByte::flag){
                if let Some(pre_status) = pre_status {
                    midi_message.m_status = pre_status.clone();
                } else {
                    return Err("status error".into());
                }
            }else {
                midi_message.m_status = status;
                cursor += 1;
            }
        } else {
            return Err("status error".into());
        }
        let ret: MidiStatusByte = midi_message.m_status.intersection(MidiStatusByte::command);
        match ret.bits() {
            NOTE_OFF_VALUE => {
                midi_message.m_ment_event = Event::Midi {
                    message: MessageEvent::NoteOff {
                        key: MidiDataByte::from_bits(raw_data[cursor]).unwrap(),
                        velocity: MidiDataByte::from_bits(raw_data[cursor + 1]).unwrap(),
                    },
                };
                cursor += 2;
            }
            NOTE_ON_VALUE => {
                midi_message.m_ment_event = Event::Midi {
                    message: MessageEvent::NoteOn {
                        key: MidiDataByte::from_bits(raw_data[cursor]).unwrap(),
                        velocity: MidiDataByte::from_bits(raw_data[cursor + 1]).unwrap(),
                    },
                };
                cursor += 2;
            }
            AFTERTOUCH_VALUE => {
                midi_message.m_ment_event = Event::Midi {
                    message: MessageEvent::Aftertouch {
                        key: raw_data[cursor],
                        value: raw_data[cursor + 1],
                    },
                };
                cursor += 2;
            }
            CONTROLLER_VALUE => {
                midi_message.m_ment_event = Event::Midi {
                    message: MessageEvent::Controller {
                        controller: raw_data[cursor],
                        value: raw_data[cursor + 1],
                    },
                };
                cursor += 2;
            }
            PROGRAM_CHANGE_VALUE => {
                midi_message.m_ment_event = Event::Midi {
                    message: MessageEvent::ProgramChange {
                        program: raw_data[cursor],
                    },
                };
                cursor += 1;
            }
            CHANNEL_PRESSURE_VALUE => {
                midi_message.m_ment_event = Event::Midi {
                    message: MessageEvent::ChannelAftertouch {
                        value: raw_data[cursor],
                    },
                };
                cursor += 1;
            }
            PITCH_WHEEL_VALUE => {
                midi_message.m_ment_event = Event::Midi {
                    message: MessageEvent::PitchWheel {
                        value: u16::from_be_bytes(raw_data[cursor..cursor + 2].try_into()?),
                    },
                };
                cursor += 2;
            }
            SYSTEM_EXCLUSIVE_VALUE => {
                // 系统消息
                let system_type = raw_data[cursor];
                let system_length = raw_data[cursor + 1];
                let system_data =
                    raw_data[cursor + 2..cursor + 2 + system_length as usize].to_vec();
                midi_message.m_ment_event = Event::Midi {
                    message: MessageEvent::SystemMessage {
                        system_type,
                        system_length,
                        system_data,
                    },
                };
                cursor += 2 + system_length as usize;
            }
            _ => todo!("这是什么？"),
        }
        midi_message.m_message_size = cursor;
        Ok(midi_message)
    }
}


#[derive(Clone, Debug)]
pub enum Event {
    Midi { message: MessageEvent },
    None,
}

#[derive(Clone, Debug)]
pub enum MessageEvent {
    NoteOff {
        key: MidiDataByte,
        velocity: MidiDataByte,
    },
    NoteOn {
        key: MidiDataByte,
        velocity: MidiDataByte,
    },
    Controller {
        controller: u8,
        value: u8,
    },
    SystemMessage {
        system_type: u8,
        system_length: u8,
        system_data: Vec<u8>,
    },
    ProgramChange {
        program: u8,
    },
    ChannelAftertouch {
        value: u8,
    },
    Aftertouch {
        key: u8,
        value: u8,
    },
    PitchWheel {
        value: u16, // TODO 弯音要处理两个字节的数据 一个字节的最高位 和 一个字节的最低位要扔掉
    },
}
