#![allow(dead_code)]
use core::slice;
use std::{error::Error, fs::{File, OpenOptions}, io::Write, mem::size_of, os::raw, path::Display};
use bitflags::bitflags;
use self::base::*;
use self::header::*;
use self::tracks::*;

mod base;
mod header;
mod tracks;
mod midi_message;

// 只看三位
const NOTE_OFF_VALUE: u8 = 0x00 << 4;
const NOTE_ON_VALUE: u8 = 0x01 << 4;
const AFTERTOUCH_VALUE: u8 = 0x02 << 4;
const CONTROLLER_VALUE: u8 = 0x03 << 4;
const PROGRAM_CHANGE_VALUE: u8 = 0x04 << 4;
const CHANNEL_PRESSURE_VALUE: u8 = 0x05 << 4;
const PITCH_WHEEL_VALUE: u8 = 0x06 << 4;
const SYSTEM_EXCLUSIVE_VALUE: u8 = 0x07 << 4;


const MIDI_HEADER_TRACKS_OFFSET:usize = 8;

#[derive(Debug)]
pub struct MidiFile {
    pub header: Header,
    pub tracks: Tracks,
}
impl Parser for MidiFile {
    fn new(raw_data: &[u8]) -> MidiFile {
        MidiFile {
            header: Header::new(raw_data),
            tracks: Tracks(Vec::new()),
        }
    }
    fn get_raw(&self) -> Vec<u8> {
        todo!()
    }

    fn parse(raw_data: &[u8]) -> Result<MidiFile, Box<dyn Error>> {
        let mut midi_file = MidiFile::new(raw_data);
        let header = Header::parse(raw_data)?; // 解析头部
        let cursor = header.m_header_size + MIDI_HEADER_TRACKS_OFFSET as u32;
        let tracks = Tracks::parse(&raw_data[cursor as usize..])?; // 解析音轨
        midi_file.header = header;
        midi_file.tracks = tracks;

        Ok(midi_file)
    }
}


pub fn test() {
    let raw_data = include_bytes!("../../test_assets/sanye.mid");
    let _midi_file: MidiFile = MidiFile::parse(raw_data).unwrap();
}
