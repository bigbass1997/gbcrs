
use crate::arch::{BusAccessable, SystemMode};

#[derive(Clone, Debug)]
pub struct Apu {
    
}
impl Apu {
    pub fn new() -> Self { Self {
        
    }}
}

impl BusAccessable for Apu {
    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            _ => (),
            _ => todo!("write {:#04X} to {:#06X}", data, addr)
        }
    }

    fn read(&mut self, addr: u16) -> u8 {
        todo!("read from {:#06X}", addr)
    }
}