pub struct MyConfig {
    pub sample_rate: u32,
    pub channels_count: u32,
    pub channel_sample_count: u32,
}

pub static CONFIG: MyConfig = MyConfig {
    sample_rate: 48000,
    channels_count: 2,
    channel_sample_count: 480,
};