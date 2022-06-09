use std::cmp::max;
use log::info;
use crate::arch::{BusAccessable, SystemMode};
use crate::SystemMode::GameboyColorGBC;

#[derive(Clone, Debug)]
pub struct Memory {
    mode: SystemMode,
    pub wram: [[u8; 0x1000]; 8],
    pub wbank: u8,
    pub undoc_regs: [u8; 4],
    pub hram: [u8; 0x7F],
}
impl Memory {
    pub fn new(mode: SystemMode) -> Self { Self {
        mode,
        wram: [[0u8; 0x1000]; 8],
        wbank: 0,
        undoc_regs: [0u8; 4],
        hram: [0u8; 0x7F],
    }}
}

impl BusAccessable for Memory {
    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0xC000..=0xCFFF => self.wram[0][(addr & 0x0FFF) as usize] = data,
            0xD000..=0xDFFF => self.wram[max(self.wbank as usize, 1)][(addr & 0x0FFF) as usize] = data,
            0xE000..=0xFDFF => self.write(addr - 0x2000, data), // ECHO RAM
            0xFF70 => self.wbank = data & 0x07, // WRAM Bank Select
            
            0xFF72 if self.mode == GameboyColorGBC => self.undoc_regs[0] = data,
            0xFF73 if self.mode == GameboyColorGBC => self.undoc_regs[1] = data,
            0xFF74 if self.mode == GameboyColorGBC => self.undoc_regs[2] = data,
            0xFF75 if self.mode == GameboyColorGBC => self.undoc_regs[3] = data & 0b01110000,
            
            0xFF80..=0xFFFE => self.hram[(addr & 0x7F) as usize] = data,
            
            _ => unimplemented!()
        }
        if addr == 0xDD02 {
            info!("Write to 0xDD02: {:02X}", data);
            if data != self.read(addr) {
                panic!("mismatched data")
            }
            
            if data == 0x00 {
                panic!("wrote 00");
            }
        }
        if data == 0x01 {
            info!("Writing 0x01 to: {:04X}", addr);
        }
    }

    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0xC000..=0xCFFF => self.wram[0][(addr & 0x0FFF) as usize],
            0xD000..=0xDFFF => self.wram[max(self.wbank as usize, 1)][(addr & 0x0FFF) as usize],
            0xE000..=0xFDFF => self.read(addr - 0x2000), // ECHO RAM
            0xFF70 => self.wbank & 0x07, // WRAM Bank Select
            
            0xFF72 if self.mode == GameboyColorGBC => self.undoc_regs[0],
            0xFF73 if self.mode == GameboyColorGBC => self.undoc_regs[1],
            0xFF74 if self.mode == GameboyColorGBC => self.undoc_regs[2],
            0xFF74 => 0xFF,
            0xFF75 if self.mode == GameboyColorGBC => self.undoc_regs[3] & 0b01110000,
            
            0xFF80..=0xFFFE => self.hram[(addr & 0x7F) as usize],
            
            _ => unimplemented!()
        }
    }
}