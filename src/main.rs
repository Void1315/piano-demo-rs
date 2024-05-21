#![allow(unused_imports)]
#![allow(dead_code)]
use bitflags::Flags;
use config::CONFIG;
use core::time;
use cpal::{
    traits::{DeviceTrait, StreamTrait},
    Device, FromSample, SizedSample, StreamConfig,
};
use midi_format::MidiFile;
use midir::{MidiInput, MidiInputConnection, MidiInputPort};
use rustysynth::Synthesizer;
use std::{
    error::Error,
    sync::{Arc, Mutex},
    thread::sleep,
};

use crate::{
    midi_derive::init_midi_derive,
    midi_format::{
        base::*,
        midi_message::{Event, MessageEvent, MidiMessage},
    },
    output_derive::init_output_derive,
    synthesizers::init_synthesizers,
};
mod config;
mod midi_derive;
mod midi_format;
mod output_derive;
mod synthesizers;
fn main() {
    // midi_format::test();
    play_midi();
}

fn play_midi() {
    let (mut synthesizer, out_put_derive) = init_conn().unwrap();
    let _output_conn = bind_synthesizer_to_output::<f32>(&mut synthesizer, &out_put_derive);
    _output_conn.play().unwrap();
    let midi_file = MidiFile::parse(include_bytes!("../test_assets/sanye.mid")).unwrap();
    let trackdiv = midi_file.header.m_time_division; // 一个四分音符的时间
    let one_tick: u32 = 545455 / 480; // TODO: 计算出来
    for track in midi_file.tracks.0.iter() {
        for message in track.m_midi_message.iter() {
            let event: &Event = &message.m_ment_event;
            let mut sum_tick: u64 = 0;

            for (index, ti) in message.m_delta_time.iter().rev().enumerate() {
                let _ti = (MidiInt::data & *ti).bits() as u32;
                let bi = _ti * 128u32.pow(index as u32) as u32;
                sum_tick += bi as u64;
            }

            let _r: u64 = sum_tick * one_tick as u64;
            let delay_time = time::Duration::from_micros(_r);
            sleep(delay_time);
            println!("delay_time: {_r}ms  event: {event:?}");
            match event {
                Event::Midi {
                    message: message_event,
                } => match message_event {
                    // TODO: 还有一些事件需要实现
                    MessageEvent::NoteOn { key, velocity } => synthesizer.lock().unwrap().note_on(
                        0,
                        key.bits() as i32,
                        velocity.bits() as i32,
                    ),
                    MessageEvent::NoteOff { key, velocity } => {
                        synthesizer.lock().unwrap().note_off(0, key.bits() as i32)
                    }
                    _ => (),
                },
                Event::None => todo!(),
            }
        }
    }
}

fn init_conn() -> Result<(Arc<Mutex<Synthesizer>>, Device), Box<dyn Error>> {
    let synthesizer: Arc<Mutex<Synthesizer>> = init_synthesizers()?;
    let out_put_derive: Device = init_output_derive()?;
    Ok((synthesizer, out_put_derive))
}

fn run() -> Result<(), Box<dyn Error>> {
    let (midi_in, port) = init_midi_derive()?;
    let mut synthesizer = init_synthesizers()?;
    let out_put_derive = init_output_derive()?;

    // 1. 将midi输入链接到合成器
    let _midi_conn = bind_midi_to_synthesizer(midi_in, &port, &mut synthesizer);
    // 2. 将合成器链接到输出设备
    let _output_conn = bind_synthesizer_to_output::<f32>(&mut synthesizer, &out_put_derive);
    _output_conn.play()?;

    loop {} // 防止主线程退出
    #[allow(unreachable_code)]
    Ok(())
}
fn bind_midi_to_synthesizer(
    midi_in: MidiInput,
    port: &MidiInputPort,
    synthesizer: &mut Arc<Mutex<Synthesizer>>,
) -> MidiInputConnection<()> {
    let _synthesizer = synthesizer.clone();
    let _conn: MidiInputConnection<()> = midi_in
        .connect(
            port,
            "midir-read-input",
            move |_, message, _| {
                let (channel, key, velocity) =
                    (message[0] as i32, message[1] as i32, message[2] as i32);
                // 获取MIDI信号 判断是否按下键
                match channel {
                    144 => _synthesizer.lock().unwrap().note_on(0, key, velocity),
                    128 => _synthesizer.lock().unwrap().note_off(0, key),
                    _ => (),
                }
                println!("{:?}", message);
            },
            (),
        )
        .unwrap();
    _conn
}

fn bind_synthesizer_to_output<T>(
    synthesizer: &mut Arc<Mutex<Synthesizer>>,
    output_device: &Device,
) -> cpal::Stream
where
    T: SizedSample + FromSample<f64>,
{
    let mut _synthesizer = synthesizer.clone();
    let config = output_device.default_output_config().unwrap();
    dbg!(config.clone());
    let config: StreamConfig = config.into();
    let channels = config.channels as usize;
    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
    let stream: cpal::Stream = output_device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                write_data(data, channels, &mut _synthesizer)
            },
            err_fn,
            None,
        )
        .unwrap();
    stream
}

fn write_data(data: &mut [f32], _channels: usize, synthesizer: &Arc<Mutex<Synthesizer>>) {
    let mut synthesizer = synthesizer.lock().unwrap();
    let mut left: Vec<f32> = vec![0f32; CONFIG.channel_sample_count as usize];
    let mut right: Vec<f32> = vec![0f32; CONFIG.channel_sample_count as usize];
    synthesizer.render(&mut left, &mut right);
    for index in 0..CONFIG.channel_sample_count as usize {
        data[index * 2] = left[index];
        data[index * 2 + 1] = right[index];
    }
}
