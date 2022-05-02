
use crate::arch::{BusAccessable, SystemMode};

#[derive(Clone, Debug)]
pub struct Ppu {
    mode: SystemMode,
    pub vram: [u8; 0x2000],
    /// BG Palette Data
    pub bgp: u8,
}
impl Ppu {
    pub fn new(mode: SystemMode) -> Self { Self {
        mode,
        vram: [0xAAu8; 0x2000],
        bgp: 0xFC,
    }}
}

impl BusAccessable for Ppu {
    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr & 0x1FFF) as usize] = data,
            0xFF47 => self.bgp = data,
            _ => todo!("write {:#04X} to {:#06X}", data, addr)
        }
    }

    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr & 0x1FFF) as usize],
            0xFF47 => self.bgp,
            _ => todo!("read from {:#06X}", addr)
        }
    }
}