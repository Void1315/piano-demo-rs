use config::CONFIG;
use cpal::{
    traits::{DeviceTrait, StreamTrait},
    Device, FromSample, SizedSample, StreamConfig,
};
use midir::{MidiInput, MidiInputConnection, MidiInputPort};
use rustysynth::{MidiFile as RustysynthMidiFile, MidiFileSequencer, Synthesizer};
use std::{
    error::Error,
    fs,
    sync::{Arc, Mutex},
    thread, time,
};
use synthesizers::init_unlocked_synthesizers;

use crate::{
    midi_derive::init_midi_derive, output_derive::init_output_derive,
    synthesizers::init_synthesizers,
};
mod config;
mod midi_derive;
mod midi_format;
mod output_derive;
mod synthesizers;
fn main() {
    // let _ = run();
    let handle = thread::spawn(|| {
        play_midi_for_file("test_assets/sanye.mid");
    });
    handle.join().unwrap();
}

fn play_midi_for_file(file_path: &str) {
    let synthesizer = init_unlocked_synthesizers().unwrap();
    let out_put_derive = init_output_derive().unwrap();

    let mut buffer = fs::File::open(file_path).unwrap();
    let midi_file = Arc::new(RustysynthMidiFile::new(&mut buffer).unwrap());
    let end_time = midi_file.get_length(); // 时长
    display_midi_info(end_time,file_path);
    let midi_file_sequencer = Arc::new(Mutex::new(MidiFileSequencer::new(synthesizer)));
    midi_file_sequencer.lock().unwrap().play(&midi_file, false);
    let _output_conn =
        bind_midifilesequencer_to_output(&midi_file_sequencer.clone(), &out_put_derive);
    _output_conn.play().unwrap();
    thread::sleep(time::Duration::from_secs_f64(end_time));
}

fn display_midi_info(midi_duration: f64, file_path: &str){
    println!("播放 {file_path} 文件\n 音乐时长为 {midi_duration}s");
}

fn init_conn() -> Result<(Arc<Mutex<Synthesizer>>, Device), Box<dyn Error>> {
    let synthesizer: Arc<Mutex<Synthesizer>> = init_synthesizers()?;
    let out_put_derive: Device = init_output_derive()?;
    Ok((synthesizer, out_put_derive))
}

#[allow(dead_code)]
fn run() -> Result<(), Box<dyn Error>> {
    let (midi_in, port) = init_midi_derive()?;
    let (mut synthesizer, out_put_derive) = init_conn()?;

    // 1. 将midi输入链接到合成器
    let _midi_conn = bind_midi_to_synthesizer(midi_in, &port, &mut synthesizer);
    // 2. 将合成器链接到输出设备
    let _output_conn = bind_synthesizer_to_output::<f32>(&mut synthesizer, &out_put_derive);
    _output_conn.play()?;

    loop {} // 防止主线程退出
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
                write_data2_synthesizer(data, channels, &mut _synthesizer)
            },
            err_fn,
            None,
        )
        .unwrap();
    stream
}

fn bind_midifilesequencer_to_output(
    midi_file_sequencer: &Arc<Mutex<MidiFileSequencer>>,
    output_device: &Device,
) -> cpal::Stream {
    let _midi_file_sequencer = midi_file_sequencer.clone();
    let config = output_device.default_output_config().unwrap();
    let config: StreamConfig = config.into();
    let channels = config.channels as usize;
    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
    let stream: cpal::Stream = output_device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                write_data2_midi_file_sequencer(data, channels, &_midi_file_sequencer)
            },
            err_fn,
            None,
        )
        .unwrap();
    stream
}

fn write_data2_midi_file_sequencer(
    data: &mut [f32],
    _channels: usize,
    midi_file_sequencer: &Arc<Mutex<MidiFileSequencer>>,
) {
    let mut left: Vec<f32> = vec![0f32; CONFIG.channel_sample_count as usize];
    let mut right: Vec<f32> = vec![0f32; CONFIG.channel_sample_count as usize];
    midi_file_sequencer
        .lock()
        .unwrap()
        .render(&mut left, &mut right);
    for index in 0..CONFIG.channel_sample_count as usize {
        data[index * 2] = left[index];
        data[index * 2 + 1] = right[index];
    }
}

fn write_data2_synthesizer(
    data: &mut [f32],
    _channels: usize,
    synthesizer: &Arc<Mutex<Synthesizer>>,
) {
    let mut synthesizer = synthesizer.lock().unwrap();
    let mut left: Vec<f32> = vec![0f32; CONFIG.channel_sample_count as usize];
    let mut right: Vec<f32> = vec![0f32; CONFIG.channel_sample_count as usize];
    synthesizer.render(&mut left, &mut right);
    for index in 0..CONFIG.channel_sample_count as usize {
        data[index * 2] = left[index];
        data[index * 2 + 1] = right[index];
    }
}
