use std::error::Error;
use bitflags::bitflags;

pub trait Parser
where
    Self: Sized,
{
    fn new(raw_data: &[u8]) -> Self;
    fn parse(raw_data: &[u8]) -> Result<Self, Box<dyn Error>>;
    fn get_raw(&self) -> Vec<u8>;
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