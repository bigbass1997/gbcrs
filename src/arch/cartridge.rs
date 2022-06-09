
use crate::arch::{BusAccessable, SystemMode};

#[derive(Clone, Debug)]
pub struct Cartridge {
    pub rom: Vec<u8>,
}
impl Cartridge {
    pub fn new() -> Self { Self {
        rom: vec![],
    }}
}

impl BusAccessable for Cartridge {
    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x00FF => (),
            _ => todo!("write {:#04X} to {:#06X}", data, addr)
        }
    }

    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => *self.rom.get(addr as usize).unwrap_or(&0xFF),
            _ => todo!("read from {:#06X}", addr)
        }
    }
}