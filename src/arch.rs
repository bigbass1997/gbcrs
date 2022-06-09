use crate::arch::apu::Apu;
use crate::arch::cartridge::Cartridge;
use crate::arch::cpu::Cpu;
use crate::arch::memory::Memory;
use crate::arch::ppu::Ppu;
use crate::util::InfCell;

pub mod apu;
pub mod cartridge;
pub mod cpu;
pub mod memory;
pub mod ppu;


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SystemMode {
    Gameboy,
    GameboyPocket,
    SuperGameboy,
    SuperGameboy2,
    GameboyColorDMG,
    GameboyColorGBC,
}
impl Default for SystemMode {
    fn default() -> Self { Self::Gameboy }
}


pub trait BusAccessable {
    fn write(&mut self, addr: u16, data: u8);
    fn read(&mut self, addr: u16) -> u8;
}

#[derive(Clone, Debug)]
pub struct Bus {
    pub cpu: Cpu,
    pub ppu: Ppu,
    pub mem: Memory,
    pub cart: Cartridge,
    pub apu: Apu,
    pub boot_rom: [u8; 0x100],
    pub boot_disabled: u8,
}
impl Bus {
    pub fn new(mode: SystemMode) -> Self { Self {
        cpu: Cpu::new(mode),
        ppu: Ppu::new(mode),
        mem: Memory::new(mode),
        cart: Cartridge::new(),
        apu: Apu::new(),
        boot_rom: [0u8; 0x100],
        boot_disabled: 0,
    }}
}

impl BusAccessable for Bus {
    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x00FF if self.boot_disabled == 0 => (), // Boot ROM is read-only
            
            0x0000..=0x7FFF => self.cart.write(addr, data), // Cart ROM bank 00-NN
            0x8000..=0x9FFF => self.ppu.write(addr, data),  // VRAM
            0xA000..=0xBFFF => self.cart.write(addr, data), // Cart RAM
            0xC000..=0xFDFF => self.mem.write(addr, data),  // WRAM and ECHO RAM
            0xFE00..=0xFEFF => self.ppu.write(addr, data),  // OAM and prohibited
            
            0xFF00..=0xFF02 | 0xFF04..=0xFF07 => self.cpu.write(addr, data), // Input, Serial, and Timer/Divider
            0xFF0F => self.cpu.write(addr, data),                            // Interrupt Flag
            0xFF10..=0xFF26 | 0xFF30..=0xFF3F => self.apu.write(addr, data), // Sound and Wave Pattern
            0xFF40..=0xFF4B | 0xFF4F => self.ppu.write(addr, data),          // PPU controls and VRAM Bank Select
            0xFF50 => self.boot_disabled = data,                             // Disable boot ROM
            0xFF51..=0xFF55 | 0xFF68..=0xFF69 => self.ppu.write(addr, data), // VRAM DMA and BG/OBJ Palettes
            0xFF70 => self.mem.write(addr, data),                            // WRAM Bank Select
            0xFF72..=0xFF75 => self.mem.write(addr, data),                   // Undocumented registers
            0xFF76..=0xFF77 => self.apu.write(addr, data),                   // Undocumented registers
            
            0xFF80..=0xFFFE => self.mem.write(addr, data), // HRAM
            0xFFFF => self.cpu.write(addr, data), // Interrupt Enable
            
            _ => unimplemented!(),
        }
    }

    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x00FF if self.boot_disabled == 0 => self.boot_rom[addr as usize],
            
            0x0000..=0x7FFF => self.cart.read(addr), // Cart ROM bank 00-NN
            0x8000..=0x9FFF => self.ppu.read(addr),  // VRAM
            0xA000..=0xBFFF => self.cart.read(addr), // Cart RAM
            0xC000..=0xFDFF => self.mem.read(addr),  // WRAM and ECHO RAM
            0xFE00..=0xFEFF => self.ppu.read(addr),  // OAM and prohibited
            
            0xFF00..=0xFF02 | 0xFF04..=0xFF07 => self.cpu.read(addr), // Input, Serial, and Timer/Divider
            0xFF0F => self.cpu.read(addr),                            // Interrupt Flag
            0xFF10..=0xFF26 | 0xFF30..=0xFF3F => self.apu.read(addr), // Sound and Wave Pattern
            0xFF40..=0xFF4B | 0xFF4F => self.ppu.read(addr),          // PPU controls and VRAM Bank Select
            0xFF50 => self.boot_disabled,                             // Disable boot ROM
            0xFF51..=0xFF55 | 0xFF68..=0xFF69 => self.ppu.read(addr), // VRAM DMA and BG/OBJ Palettes
            0xFF70 => self.mem.read(addr),                            // WRAM Bank Select
            0xFF72..=0xFF75 => self.mem.read(addr),                   // Undocumented registers
            0xFF76..=0xFF77 => self.apu.read(addr),                   // Undocumented registers
            
            0xFF80..=0xFFFE => self.mem.read(addr), // HRAM
            0xFFFF => self.cpu.read(addr), // Interrupt Enable
            
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct Gameboy {
    pub bus: InfCell<Bus>,
    pub tcycles: usize,
}
impl Gameboy {
    pub fn new(mode: SystemMode) -> Self { Self {
        bus: InfCell::new(Bus::new(mode)),
        tcycles: 0,
    }}
    
    /// Performs one t-cycle on the system.
    /// 
    /// Note that some components may not do anything until the first of every 4 cycles. While other
    /// components may require the precision of t-cycles.
    pub fn tcycle(&mut self) {
        let bus = self.bus.get_mut();
        let passed_bus = self.bus.get_mut();
        
        bus.cpu.tcycle(passed_bus);
        bus.ppu.tcycle(passed_bus);
        
        self.tcycles += 1;
    }
    
    /// Simply calls [tcycle()] 4 times.
    pub fn mcycle(&mut self) {
        self.tcycle();
        self.tcycle();
        self.tcycle();
        self.tcycle();
    }
}