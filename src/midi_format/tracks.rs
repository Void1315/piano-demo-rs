use super::{midi_message::MidiMessage, Parser};
use std::error::Error;

#[derive(Debug)]
pub struct Tracks(pub Vec<MidiTrack>);
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
#[derive(Debug)]
pub struct MidiTrack {
    pub m_magic: [u8; 4],                 // MIDI头部标识 值为"MThd"或者 "MTrk"
    pub m_track_size: u32,                // 音轨大小
    pub m_midi_message: Vec<MidiMessage>, // 事件
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