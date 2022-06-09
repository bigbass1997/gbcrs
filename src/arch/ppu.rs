use log::info;
use crate::arch::{Bus, BusAccessable, SystemMode};

#[derive(Clone, Debug, Default)]
pub struct Tile {
    pub pixels: [[u32; 8]; 8],
}

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
        vram: [0u8; 0x2000],
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
    
    pub fn render(&self, buf: &mut [u32]) {
        /*for i in 0..buf.len() {
            if let Some(vram_pix) = self.vram.get(i) {
                buf[i] = *vram_pix as u32;
            }
        }*/
        
        
        
        let mut x = 0;
        let mut y = 0;
        let width = 160;
        for tile in self.tiles() {
            for ty in 0..8 {
                for tx in 0..8 {
                    if (y * width) + x >= buf.len() { return; }
                    //println!("{}, {}", x, y);
                    buf[(y * width) + x] = tile.pixels[tx][ty];
                    x += 1;
                }
                x -= 8;
                y += 1;
            }
            y -= 8;
            x += 8;
            
            if x >= 150 {
                x = 0;
                y += 8;
            }
        }
    }
    
    fn tiles(&self) -> Vec<Tile> {
        let mut tiles = vec![];
        let mut chunks = self.vram[0..=0x17FF].chunks_exact(16);
        
        for _ in 0..chunks.len() {
            let chunk = chunks.next().unwrap();
            let mut tile = Tile::default();
            let mut row = 0;
            for bi in (0..16).step_by(2) {
                let lsb = chunk[bi];
                let msb = chunk[bi + 1];
                
                let mut col = 0;
                for idi in (0..8).rev() {
                    let msb = msb & (1 << idi) >> idi;
                    let lsb = lsb & (1 << idi) >> idi;
                    
                    let colori = (msb << 1) | lsb;
                    tile.pixels[row][col] = self.palette(colori);
                    
                    col += 1;
                }
                row += 1;
            }
            
            tiles.push(tile);
        }
        
        tiles
    }
    
    fn palette(&self, index: u8) -> u32 {
        match index {
            0 => 0x00331111,
            1 => 0x00116611,
            2 => 0x001111AA,
            3 => 0x00FFFFFF,
            _ => {
                0x00FF0000
            },
        }
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
        
        match addr {
            0x8000..=0x9FFF => {
                info!("Wrote to VRAM: {:02X} ({:02X}) at {:04X}", data, self.vram[(addr & 0x1FFF) as usize], addr);
            },
            _ => ()
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