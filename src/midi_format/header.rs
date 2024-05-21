use super::base::Parser;
use std::{error::Error, io::Cursor, mem::take};

#[derive(Debug)]
pub struct Header {
    pub m_magic: [u8; 4],     // MIDI头部标识 值为"MThd"或者 "MTrk"
    pub m_header_size: u32,   // 头部大小
    pub m_format: u16,        // MIDI格式
    pub m_num_tracks: u16,    // 音轨数
    pub m_time_division: u16, // 时间分辨率
}

const HEADER_MAGIC_SIZE: usize = 4;
const HEADER_SCELEN_SIZE: usize = 4;
const HEADER_FORMAT_SIZE: usize = 2;
const HEADER_NTRACKS_SIZE: usize = 2;
const HEADER_TICKDIV_SIZE: usize = 2;

const HEADER_SIZE: usize = 14;
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
        todo!()
    }
    fn parse(raw_data: &[u8]) -> Result<Header, Box<dyn Error>> {
        let mut cursor = 0;
        let m_magic: [u8; 4] = raw_data[..HEADER_MAGIC_SIZE].try_into().unwrap();
        cursor += HEADER_MAGIC_SIZE;
        let m_header_size = u32::from_be_bytes(
            raw_data[cursor..cursor + HEADER_SCELEN_SIZE]
                .try_into()
                .unwrap(),
        );
        cursor += HEADER_SCELEN_SIZE;
        let m_format = u16::from_be_bytes(
            raw_data[cursor..cursor + HEADER_FORMAT_SIZE]
                .try_into()
                .unwrap(),
        );
        cursor += HEADER_FORMAT_SIZE;
        let m_num_tracks = u16::from_be_bytes(
            raw_data[cursor..cursor + HEADER_NTRACKS_SIZE]
                .try_into()
                .unwrap(),
        );
        cursor += HEADER_NTRACKS_SIZE;
        let m_time_division = u16::from_be_bytes(
            raw_data[cursor..cursor + HEADER_TICKDIV_SIZE]
                .try_into()
                .unwrap(),
        );
        Ok(Header {
            m_magic,
            m_header_size,
            m_format,
            m_num_tracks,
            m_time_division,
        })
    }
}
