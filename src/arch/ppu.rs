
use crate::arch::{Bus, BusAccessable, SystemMode};

#[derive(Clone, Debug)]
pub struct Ppu {
    mode: SystemMode,
    pub vram: [u8; 0x2000],
    /// LCD Control (0xFF40) (R/W)
    pub lcdc: u8,
    /// LCD Status (0xFF41) (R/W)
    pub stat: u8,
    /// Scroll Y (0xFF42) (R/W)
    pub scy: u8,
    /// Scroll X (0xFF43) (R/W)
    pub scx: u8,
    /// LCD Y Coordinate (0xFF44) (R)
    pub ly: u8,
    /// LY Compare (0xFF45) (R/W)
    pub lyc: u8,
    /// BG Palette Data (0xFF47) (R/W)
    pub bgp: u8,
    /// Window Y Position (0xFF4A) (R/W)
    pub wy: u8,
    /// Window X Position + 7 (0xFF4B) (R/W)
    pub wx: u8, //TODO: Implement hardware bugs when wx == 0 or 166
}
impl Ppu {
    pub fn new(mode: SystemMode) -> Self { Self {
        mode,
        vram: [0xAAu8; 0x2000],
        lcdc: 0,
        stat: 0,
        bgp: 0,
        scy: 0,
        scx: 0,
        ly: 0x90,
        lyc: 0,
        wy: 0,
        wx: 0,
    }}
    
    pub fn tcycle(&mut self, bus: &mut Bus) {
        
    }
}

impl BusAccessable for Ppu {
    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr & 0x1FFF) as usize] = data,
            0xFF40 => self.lcdc = data,
            0xFF41 => self.stat = data,
            0xFF42 => self.scy = data,
            0xFF43 => self.scx = data,
            0xFF44 => (),
            0xFF45 => self.lyc = data,
            0xFF47 => self.bgp = data,
            0xFF4A => self.wy = data, //TODO: Check if register can be set above value 143
            0xFF4B => self.wx = data, //TODO: Check if register can be set above value 166
            _ => todo!("write {:#04X} to {:#06X}", data, addr)
        }
    }

    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr & 0x1FFF) as usize],
            0xFF40 => self.lcdc,
            0xFF41 => self.stat,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF47 => self.bgp,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            _ => todo!("read from {:#06X}", addr)
        }
    }
}