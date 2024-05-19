use midir::{MidiInput, MidiInputPort};
use std::{
    error::Error,
    io::{stdin, stdout, Write},
};

pub fn init_midi_derive() -> Result<(MidiInput, MidiInputPort), Box<dyn Error>> {
    let client_name = "输入设备";
    let midi_in: MidiInput = MidiInput::new(client_name).unwrap();
    let ports = midi_in.ports();
    let port: MidiInputPort = match ports.len() {
        0 => panic!("未找到任何端口"),
        1 => ports[0].clone(),
        _ => match chose_port(&ports, &midi_in) {
            Some(port) => port,
            None => panic!("未找到任何端口"),
        },
    };

    Ok((midi_in, port))
}

fn chose_port<'a>(ports: &Vec<MidiInputPort>, mide: &MidiInput) -> Option<MidiInputPort> {
    println!("找到多个端口，请选择一个端口连接：");
    for (i, port) in ports.iter().enumerate() {
        println!("第{}个: 端口名为 {}", i, mide.port_name(port).unwrap());
    }
    println!("请输入端口编号：");
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let port = ports
        .get(input.trim().parse::<usize>().unwrap())
        .ok_or("错误的端口编号");
    match port {
        Ok(port) => Some(port.clone()),
        Err(_) => None,
    }
}
