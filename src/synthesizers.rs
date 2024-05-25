use std::error::Error;
use std::fs::File;
use std::io;
use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};
use std::sync::{Arc, Mutex};

use crate::config::CONFIG;

const SF2_PATH: &str = "sf2/TimGM6mb.sf2";

pub fn init_synthesizers() -> Result<Arc<Mutex<Synthesizer>>, Box<dyn Error>> {
    let mut sf2 = open_sf2(SF2_PATH)?;
    let sound_font = Arc::new(SoundFont::new(&mut sf2)?);
    let settings = SynthesizerSettings::new(CONFIG.sample_rate as i32);
    let synthesizer: Synthesizer =  Synthesizer::new(&sound_font, &settings)?;
    let synthesizer: Arc<Mutex<Synthesizer>> = Arc::new(Mutex::new(synthesizer));
    synthesizer.lock().unwrap().set_master_volume(12f32);
    Ok(synthesizer)
}

pub fn init_unlocked_synthesizers() -> Result<Synthesizer, Box<dyn Error>> {
    let mut sf2 = open_sf2(SF2_PATH)?;
    let sound_font = Arc::new(SoundFont::new(&mut sf2)?);
    let settings = SynthesizerSettings::new(CONFIG.sample_rate as i32);
    let mut synthesizer: Synthesizer =  Synthesizer::new(&sound_font, &settings)?;
    synthesizer.set_master_volume(12f32);
    Ok(synthesizer)
}


fn open_sf2(path: &str) -> io::Result<File> {
    File::open(path)
}