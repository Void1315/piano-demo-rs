#![allow(dead_code)]

use core::slice;
use std::{error::Error, fs::{File, OpenOptions}, io::Write, mem::size_of, os::raw, path::Display};

use bitflags::bitflags;

// 只看三位
const NOTE_OFF_VALUE: u8 = 0x00 << 4;
const NOTE_ON_VALUE: u8 = 0x01 << 4;
const AFTERTOUCH_VALUE: u8 = 0x02 << 4;
const CONTROLLER_VALUE: u8 = 0x03 << 4;
const PROGRAM_CHANGE_VALUE: u8 = 0x04 << 4;
const CHANNEL_PRESSURE_VALUE: u8 = 0x05 << 4;
const PITCH_WHEEL_VALUE: u8 = 0x06 << 4;
const SYSTEM_EXCLUSIVE_VALUE: u8 = 0x07 << 4;

trait Parser
where
    Self: Sized,
{
    fn new(raw_data: &[u8]) -> Self;
    fn parse(raw_data: &[u8]) -> Result<Self, Box<dyn Error>>;
    fn get_raw(&self) -> Vec<u8>;
}
#[derive(Debug)]
pub struct Tracks(Vec<MidiTrack>);
impl Parser for Tracks {
    fn new(_: &[u8]) -> Self {
        Tracks(Vec::new())
    }

    fn get_raw(&self) -> Vec<u8> {
        todo!()
    }

    fn parse(raw_data: &[u8]) -> Result<Tracks, Box<dyn Error>> {
        let mut tracks = Vec::new();
        let mut cursor = 0;
        loop {
            let track = MidiTrack::parse(&raw_data[cursor as usize..])?;
            cursor += track.m_track_size + 8;
            tracks.push(track);
            if cursor >= raw_data.len() as u32 {
                break;
            }
        }
        Ok(Tracks(tracks))
    }
}

impl Parser for MidiTrack {
    fn new(_: &[u8]) -> MidiTrack {
        MidiTrack {
            m_magic: [0; 4],
            m_track_size: 0,
            m_midi_message: Vec::new(),
        }
    }

    fn get_raw(&self) -> Vec<u8> {
        todo!()
    }

    fn parse(raw_data: &[u8]) -> Result<MidiTrack, Box<dyn Error>> {
        let mut midi_track = MidiTrack::new(raw_data);
        let m_magic = raw_data[0..4].try_into()?; // MIDI头部标识 值为"MThd"或者 "MTrk"
        let m_track_size = u32::from_be_bytes(raw_data[4..8].try_into()?); // 音轨大小
        let mut cursor = 0;
        let mut message_num = 0;
        let mut midi_message = Vec::new();
        let raw_data = &raw_data[8..];
        let mut pre_status = Option::None;
        loop {
            let _midi_message = MidiMessage::parse(&raw_data[cursor as usize..], &pre_status)?;
            cursor += _midi_message.get_message_size() as u32;
            message_num += 1;
            pre_status = Some(_midi_message.m_status.clone());
            midi_message.push(_midi_message);
            println!("cursor: {cursor}, m_track_size: {m_track_size}, message_num: {message_num}");
            if cursor >= m_track_size{
                break;
            }
        }

        midi_track.m_magic = m_magic;
        midi_track.m_track_size = m_track_size;
        midi_track.m_midi_message = midi_message;
        Ok(midi_track)
    }
}
#[derive(Debug)]
pub struct MidiFile {
    pub header: Header,
    pub tracks: Tracks,
}
impl Parser for MidiFile {
    fn new(_: &[u8]) -> MidiFile {
        MidiFile {
            header: Header {
                m_magic: [0; 4],
                m_header_size: 0,
                m_format: 0,
                m_num_tracks: 0,
                m_time_division: 0,
            },
            tracks: Tracks(Vec::new()),
        }
    }
    fn get_raw(&self) -> Vec<u8> {
        let mut raw: Vec<u8> = Vec::new();
        // self.header.get_raw();
        raw.extend_from_slice(&self.header.get_raw());
        // self.tracks.get_raw();
        raw.extend_from_slice(&self.tracks.get_raw());
        raw
    }

    fn parse(raw_data: &[u8]) -> Result<MidiFile, Box<dyn Error>> {
        let mut midi_file = MidiFile::new(raw_data);
        let header = Header::parse(raw_data)?; // 解析头部

        let cursor = header.m_header_size + 8;
        let tracks = Tracks::parse(&raw_data[cursor as usize..])?; // 解析音轨
        midi_file.header = header;
        midi_file.tracks = tracks;

        Ok(midi_file)
    }
}
#[derive(Debug)]
pub struct Header {
    pub m_magic: [u8; 4],     // MIDI头部标识 值为"MThd"或者 "MTrk"
    pub m_header_size: u32,   // 头部大小
    pub m_format: u16,        // MIDI格式
    pub m_num_tracks: u16,    // 音轨数
    pub m_time_division: u16, // 时间分辨率
}

impl Parser for Header {
    fn new(_: &[u8]) -> Self {
        Header {
            m_magic: [0; 4],
            m_header_size: 0,
            m_format: 0,
            m_num_tracks: 0,
            m_time_division: 0,
        }
    }

    fn get_raw(&self) -> Vec<u8> {
        // let mut raw: Vec<u8> = Vec::new();
        // raw.extend_from_slice(&self.m_magic);
        // let m_header_size_raw = self.m_header_size.to_be_bytes();
        // raw.extend_from_slice(&m_header_size_raw);

        // raw.extend_from_slice(&self.m_format.to_be_bytes());

        // raw
        todo!()
    }

    fn parse(raw_data: &[u8]) -> Result<Header, Box<dyn Error>> {
        let m_magic = raw_data[0..4].try_into()?; // MIDI头部标识 值为"MThd"或者 "MTrk"
        let m_header_size = u32::from_be_bytes(raw_data[4..8].try_into()?); // 头部大小
        let m_format = u16::from_be_bytes(raw_data[8..10].try_into()?); // MIDI格式
        let m_num_tracks = u16::from_be_bytes(raw_data[10..12].try_into()?); // 音轨数
        let m_time_division = u16::from_be_bytes(raw_data[12..14].try_into()?); // 时间分辨率
        let header = Header {
            m_magic,
            m_header_size,
            m_format,
            m_num_tracks,
            m_time_division,
        };
        Ok(header)
    }
}

#[derive(Debug)]
pub struct MidiTrack {
    pub m_magic: [u8; 4],                 // MIDI头部标识 值为"MThd"或者 "MTrk"
    pub m_track_size: u32,                // 音轨大小
    pub m_midi_message: Vec<MidiMessage>, // 事件
}
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

    fn parse(raw_data: &[u8], pre_status: &Option<MidiStatusByte>) -> Result<MidiMessage, Box<dyn Error>> {
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

bitflags! {
    // midi中的变长int可以用多个 u8表示 每个u8的最高位为1表示后面还有数据
    // 所以 可以用Vec<Flags>来表示
    #[derive(Debug, Clone, Copy)]
    pub struct MidiInt: u8 { // 用来表示MIDI中的变长int
        const flag = 0b1000_0000; // 最高位为1时 表示需要再读取一个字节 最高位为0时 表示这是最后一个字节
        const data = 0b0111_1111; // 低7位表示数据
    }

    // 虽然形式和MidiInt一样 但是含义不同 这个表示的是MIDI中的数据字节
    #[derive(Debug, Clone, Copy)]
    pub struct MidiDataByte: u8 {
        const flag = 0b1000_0000; // 数据字节最高位为0
        const value = 0b0111_1111; // 数据字节的值
    }
    #[derive(Debug, Clone, Copy)]
    pub struct MidiStatusByte: u8 {
        const flag = 0b1000_0000;       // 最高位标示这是一个状态字节
        const command = 0b0111_0000;    // 高三位表示命令 也就是消息类型
        const channel = 0b0000_1111;    // 低四位表示通道
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

pub fn test() {
    let raw_data = include_bytes!("../test_assets/sanye.mid");
    let _midi_file: MidiFile = MidiFile::parse(raw_data).unwrap();
    // println!("{:?}", midi_file);
}
