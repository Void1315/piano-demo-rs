use std::error::Error;
use super::base::Parser;

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
