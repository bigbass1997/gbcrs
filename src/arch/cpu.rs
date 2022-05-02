#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use std::fmt::{Debug, Formatter};
use crate::arch::{Bus, BusAccessable, SystemMode};
use bitflags::bitflags;
use log::debug;

#[derive(Copy, Clone)]
pub struct InstructionProcedure {
    pub done: bool,
    func: fn(&mut Self, &mut Cpu, &mut Bus),
    mcycle: u8,
    tmp0: u8,
    tmp1: u8,
    tmp_addr: u16,
}
impl Debug for InstructionProcedure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InstructionProcedure")
         .field("done", &self.done)
         .field("mcycle", &self.mcycle)
         .finish()
    }
}
impl InstructionProcedure {
    pub fn new(step_func: fn(&mut InstructionProcedure, &mut Cpu, &mut Bus)) -> Self {
        Self {
            done: false,
            func: step_func,
            mcycle: 1,
            tmp0: 0,
            tmp1: 0,
            tmp_addr: 0
        }
    }
    
    pub fn step(&mut self, cpu: &mut Cpu, bus: &mut Bus) {
        (self.func)(self, cpu, bus);
        self.mcycle += 1;
    }
}

bitflags! {
    pub struct FlagsReg: u8 {
        const Zero      = 0b10000000;
        const Negative  = 0b01000000;
        const HalfCarry = 0b00100000;
        const Carry     = 0b00010000;
    }
}
impl std::fmt::Display for FlagsReg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        if self.intersects(FlagsReg::Zero)      { s.push('Z') } else { s.push('z') }
        if self.intersects(FlagsReg::Negative)  { s.push('N') } else { s.push('n') }
        if self.intersects(FlagsReg::HalfCarry) { s.push('H') } else { s.push('h') }
        if self.intersects(FlagsReg::Carry)     { s.push('C') } else { s.push('c') }
        
        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug)]
pub struct Regs {
    pub a: u8,
    pub f: FlagsReg,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}
impl Regs {
    pub fn new(mode: SystemMode) -> Self {
        use SystemMode::*;
        match mode {
            Gameboy => Self {
                a: 0x01, f: FlagsReg::from_bits_truncate(0xB0),
                b: 0x00, c: 0x13,
                d: 0x00, e: 0xD8,
                h: 0x01, l: 0x4D,
                sp: 0xFFFE,
                pc: 0x0000,
            },
            GameboyPocket => Self {
                a: 0xFF, f: FlagsReg::from_bits_truncate(0xB0),
                b: 0x00, c: 0x13,
                d: 0x00, e: 0xD8,
                h: 0x01, l: 0x4D,
                sp: 0xFFFE,
                pc: 0x0000,
            },
            SuperGameboy => Self {
                a: 0x01, f: FlagsReg::from_bits_truncate(0x00),
                b: 0x00, c: 0x14,
                d: 0x00, e: 0x00,
                h: 0xC0, l: 0x60,
                sp: 0xFFFE,
                pc: 0x0000,
            },
            SuperGameboy2 => unimplemented!(),
            GameboyColorDMG => Self {
                a: 0x11, f: FlagsReg::from_bits_truncate(0x80),
                b: 0x00, c: 0x00,
                d: 0x00, e: 0x08,
                h: 0x00, l: 0x7C,
                sp: 0xFFFE,
                pc: 0x0000,
            },
            GameboyColorGBC => Self {
                a: 0x11, f: FlagsReg::from_bits_truncate(0x80),
                b: 0x00, c: 0x00,
                d: 0xFF, e: 0x56,
                h: 0x00, l: 0x0D,
                sp: 0xFFFE,
                pc: 0x0000,
            },
        }
    }
    
    #[inline(always)]
    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f.bits as u16)
    }
    
    #[inline(always)]
    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }
    
    #[inline(always)]
    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }
    
    #[inline(always)]
    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }
    
    #[inline(always)]
    pub fn splo(&self) -> u8 {
        self.sp as u8
    }
    #[inline(always)]
    pub fn sphi(&self) -> u8 {
        (self.sp >> 8) as u8
    }
    
    #[inline(always)]
    pub fn pclo(&self) -> u8 {
        self.pc as u8
    }
    #[inline(always)]
    pub fn pchi(&self) -> u8 {
        (self.pc >> 8) as u8
    }
    
    #[inline(always)]
    pub fn set_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f.bits = (val & 0x00FF) as u8;
    }
    
    #[inline(always)]
    pub fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = (val & 0x00FF) as u8;
    }
    
    #[inline(always)]
    pub fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = (val & 0x00FF) as u8;
    }
    
    #[inline(always)]
    pub fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = (val & 0x00FF) as u8;
    }
    
    #[inline(always)]
    pub fn set_splo(&mut self, val: u8) {
        self.sp = (self.sp & 0xFF00) | (val as u16);
    }
    #[inline(always)]
    pub fn set_sphi(&mut self, val: u8) {
        self.sp = ((val as u16) << 8) | (self.sp & 0x00FF);
    }
    
    #[inline(always)]
    pub fn set_pclo(&mut self, val: u8) {
        self.pc = (self.pc & 0xFF00) | (val as u16);
    }
    #[inline(always)]
    pub fn set_pchi(&mut self, val: u8) {
        self.pc = ((val as u16) << 8) | (self.pc & 0x00FF);
    }
}


#[derive(Clone, Debug)]
pub struct Cpu {
    instr_count: usize, // debug only
    mode: SystemMode,
    tcount: u8,
    procedure: Option<InstructionProcedure>,
    pub regs: Regs,
}
impl Cpu {
    pub fn new(mode: SystemMode) -> Self { Self {
        instr_count: 1,
        mode,
        tcount: 0,
        procedure: None,
        regs: Regs::new(mode),
    }}
    
    pub fn tcycle(&mut self, bus: &mut Bus) {
        if self.tcount == 0 {
            //debug!("ROW: {:06} | PC: {:04X} = {:02X} | F: {} {:02X} | SP: {:04X} | HL: {:04X}", self.instr_count, self.regs.pc, bus.read(self.regs.pc), self.regs.f, self.regs.f, self.regs.sp, self.regs.hl());
            debug!("{:06}| A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X})",
                self.instr_count, self.regs.a, self.regs.f.bits, self.regs.b, self.regs.c, self.regs.d, self.regs.e, self.regs.h, self.regs.l, self.regs.sp, self.regs.pc, bus.read(self.regs.pc), bus.read(self.regs.pc + 1)
            );
            
            if self.procedure.is_none() {
                let opcode = self.fetch(bus);
                let x = (opcode & 0b11000000) >> 6;
                let y = (opcode & 0b00111000) >> 3;
                let z = opcode & 0b00000111;
                let p = y >> 1;
                let q = y & 0b1;
                debug!("x: {} | z: {} | y: {} | p: {} | q: {}", x, z, y, p, q);
                
                self.procedure = Some(match opcode {
                    0xDD | 0xFD => unimplemented!(),
                    0xED => unimplemented!(),
                    0xCB => { 
                        let opcode = self.fetch(bus);
                        let x = (opcode & 0b11000000) >> 6;
                        let y = (opcode & 0b00111000) >> 3;
                        debug!("op: {:02X} | x: {} | y: {}", opcode, x, y);
                        
                        match x {
                            0 => InstructionProcedure::new(rot),
                            1 => InstructionProcedure::new(bit),
                            2 => todo!(),
                            3 => todo!(),
                            _ => panic!("unreachable")
                        }
                    },
                    _ => match x {
                        0 => match z {
                            0 => match y {
                                0 => InstructionProcedure::new(nop),
                                1 => InstructionProcedure::new(ld_u16sp),
                                2 => InstructionProcedure::new(stop),
                                3 => InstructionProcedure::new(jr_d),
                                4..=7 => InstructionProcedure::new(jr_cond),
                                _ => panic!("unreachable")
                            },
                            1 => match q {
                                0 => InstructionProcedure::new(ld_rpu16),
                                1 => todo!(), // add_hlrp
                                _ => panic!("unreachable")
                            },
                            2 => match q {
                                0 => InstructionProcedure::new(ld_toindirect),
                                1 => InstructionProcedure::new(ld_fromindirect),
                                _ => panic!("unreachable")
                            },
                            
                            4 => InstructionProcedure::new(inc_r),
                            5 => InstructionProcedure::new(dec_r),
                            6 => InstructionProcedure::new(ld_ru8),
                            _ => todo!()
                        },
                        1 => if y == 6 && z == 6 {
                                todo!() // HALT
                            } else {
                                InstructionProcedure::new(ld_rr)
                        },
                        2 => match y {
                            5 => InstructionProcedure::new(xor_ar),
                            _ => todo!()
                        },
                        3 => match z {
                            0 => match y {
                                4 => InstructionProcedure::new(ld_toio_u8),
                                6 => InstructionProcedure::new(ld_fromio_u8),
                                _ => todo!()
                            },
                            1 => match q {
                                0 => InstructionProcedure::new(pop),
                                1 => match p {
                                    0 => InstructionProcedure::new(ret),
                                    
                                    3 => InstructionProcedure::new(ld_sphl),
                                    _ => todo!()
                                }
                                _ => panic!("unreachable")
                            },
                            2 => match y {
                                4 => InstructionProcedure::new(ld_toio_c),
                                6 => InstructionProcedure::new(ld_fromio_c),
                                _ => todo!()
                            },
                            
                            5 => match q {
                                0 => InstructionProcedure::new(push),
                                1 => match p {
                                    0 => InstructionProcedure::new(call_u16),
                                    1..=3 => panic!("removed opcode"),
                                    _ => panic!("unreachable")
                                }
                                _ => panic!("unreachable")
                            } 
                            _ => todo!()
                        },
                        _ => panic!("unreachable")
                    }
                });
            }
            
            let mut proc = self.procedure.unwrap();
            proc.step(self, bus);
            
            if proc.done {
                self.procedure = None;
                self.instr_count += 1;
            } else {
                self.procedure = Some(proc);
            }
        }
        
        self.tcount += 1;
        if self.tcount == 4 {
            self.tcount = 0;
        }
    }
    
    fn fetch(&mut self, bus: &mut Bus) -> u8 {
        let fetch = bus.read(self.regs.pc);
        self.regs.pc += 1;
        
        fetch
    }
    
    fn stack_push(&mut self, bus: &mut Bus, data: u8) {
        self.regs.sp -= 1;
        bus.write(self.regs.sp, data);
    }
    
    fn stack_pop(&mut self, bus: &mut Bus) -> u8 {
        let val = bus.read(self.regs.sp);
        self.regs.sp += 1;
        
        val
    }
}

impl BusAccessable for Cpu {
    fn write(&mut self, addr: u16, data: u8) {
        todo!("write {:#04X} to {:#06X}", data, addr)
    }

    fn read(&mut self, addr: u16) -> u8 {
        todo!("read from {:#06X}", addr)
    }
}


// Instruction Functions

/// 0x00
fn nop(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) {
    proc.done = true;
}
/// 0x10
fn stop(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) {
    panic!("STOP instruction called");
}
/// 0x08
fn ld_u16sp(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) {
    match proc.mcycle {
        2 => proc.tmp0 = cpu.fetch(bus),
        3 => proc.tmp1 = cpu.fetch(bus),
        4 => cpu.stack_push(bus, proc.tmp0),
        5 => {
            cpu.stack_push(bus, proc.tmp1);
            
            proc.done = true;
        },
        _ => ()
    }
}
/// 0x18
fn jr_d(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) {
    match proc.mcycle {
        2 => proc.tmp0 = cpu.fetch(bus),
        3 => {
            cpu.regs.pc = cpu.regs.pc.wrapping_add(proc.tmp0 as i8 as u16);
            
            proc.done = true;
        },
        _ => ()
    }
}
/// 0x20, 0x28, 0x30, 0x38
fn jr_cond(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) {
    match proc.mcycle {
        2 => {
            let opcode = bus.read(cpu.regs.pc - 1);
            proc.tmp0 = ((opcode & 0b00111000) >> 3) - 4; // y-4
            proc.tmp1 = cpu.fetch(bus); // d (displacement)
        },
        3 => {
            let cond = match proc.tmp0 {
                0 => !cpu.regs.f.contains(FlagsReg::Zero),
                1 => cpu.regs.f.contains(FlagsReg::Zero),
                2 => !cpu.regs.f.contains(FlagsReg::Carry),
                3 => cpu.regs.f.contains(FlagsReg::Carry),
                _ => panic!("unreachable")
            };
            
            if !cond {
                proc.done = true;
            }
        },
        4 => (),
        5 => {
            cpu.regs.pc = cpu.regs.pc.wrapping_add(proc.tmp1 as i8 as u16);
            
            proc.done = true;
        },
        _ => ()
    }
}

/// 0x04, 0x14, 0x24, 0x34, 0x0C, 0x1C, 0x2C, 0x3C
fn inc_r(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) {
    match proc.mcycle {
        1 => {
            #[inline]
            fn mut_overflow_add(reg: &mut u8, over: &mut bool) -> u8 {
                let (val, carry) = reg.overflowing_add(1);
                *reg = val;
                *over = carry;

                val
            }
            
            let opcode = bus.read(cpu.regs.pc - 1);
            
            let mut over = false;
            let result = match (opcode & 0b00111000) >> 3 { // r[y]
                0 => mut_overflow_add(&mut cpu.regs.b, &mut over),
                1 => mut_overflow_add(&mut cpu.regs.c, &mut over),
                2 => mut_overflow_add(&mut cpu.regs.d, &mut over),
                3 => mut_overflow_add(&mut cpu.regs.e, &mut over),
                4 => mut_overflow_add(&mut cpu.regs.h, &mut over),
                5 => mut_overflow_add(&mut cpu.regs.l, &mut over),
                6 => return,
                7 => mut_overflow_add(&mut cpu.regs.a, &mut over),
                _ => panic!("unreachable")
            };
            cpu.regs.f.set(FlagsReg::Zero, result == 0);
            cpu.regs.f.set(FlagsReg::Negative, false);
            cpu.regs.f.set(FlagsReg::HalfCarry, over);
            
            proc.done = true;
        },
        2 => {
            let (val, carry) = bus.read(cpu.regs.hl()).overflowing_add(1);
            proc.tmp0 = val;
            proc.tmp1 = carry as u8;
        },
        3 => {
            bus.write(cpu.regs.hl(), proc.tmp0);
            cpu.regs.f.set(FlagsReg::Zero, proc.tmp0 == 0);
            cpu.regs.f.set(FlagsReg::Negative, false);
            cpu.regs.f.set(FlagsReg::HalfCarry, proc.tmp1 != 0);
            
            proc.done = true;
        },
        _ => ()
    }
}
/// 0x05, 0x15, 0x25, 0x35, 0x0D, 0x1D, 0x2D, 0x3D
fn dec_r(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) {
    match proc.mcycle {
        1 => {
            #[inline]
            fn mut_overflow_sub(reg: &mut u8, over: &mut bool) -> u8 {
                let (val, carry) = reg.overflowing_sub(1);
                *reg = val;
                *over = carry;

                val
            }
            
            let opcode = bus.read(cpu.regs.pc - 1);
            
            let mut over = false;
            let result = match (opcode & 0b00111000) >> 3 { // r[y]
                0 => mut_overflow_sub(&mut cpu.regs.b, &mut over),
                1 => mut_overflow_sub(&mut cpu.regs.c, &mut over),
                2 => mut_overflow_sub(&mut cpu.regs.d, &mut over),
                3 => mut_overflow_sub(&mut cpu.regs.e, &mut over),
                4 => mut_overflow_sub(&mut cpu.regs.h, &mut over),
                5 => mut_overflow_sub(&mut cpu.regs.l, &mut over),
                6 => return,
                7 => mut_overflow_sub(&mut cpu.regs.a, &mut over),
                _ => panic!("unreachable")
            };
            cpu.regs.f.set(FlagsReg::Zero, result == 0);
            cpu.regs.f.set(FlagsReg::Negative, false);
            cpu.regs.f.set(FlagsReg::HalfCarry, over);
            
            proc.done = true;
        },
        2 => {
            let (val, carry) = bus.read(cpu.regs.hl()).overflowing_sub(1);
            proc.tmp0 = val;
            proc.tmp1 = carry as u8;
        },
        3 => {
            bus.write(cpu.regs.hl(), proc.tmp0);
            cpu.regs.f.set(FlagsReg::Zero, proc.tmp0 == 0);
            cpu.regs.f.set(FlagsReg::Negative, false);
            cpu.regs.f.set(FlagsReg::HalfCarry, proc.tmp1 != 0);
            
            proc.done = true;
        },
        _ => ()
    }
}

/// 0xA8 - 0xAF
fn xor_ar(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) {
    match proc.mcycle {
        1 => {
            let opcode = bus.read(cpu.regs.pc - 1);
            
            cpu.regs.a ^= match opcode & 0b00000111 {
                0 => cpu.regs.b,
                1 => cpu.regs.c,
                2 => cpu.regs.d,
                3 => cpu.regs.d,
                4 => cpu.regs.h,
                5 => cpu.regs.l,
                6 => return, // wait another mcycle for special case 'XOR A,(HL)'
                7 => cpu.regs.a,
                _ => panic!("unreachable")
            };
            cpu.regs.f.bits = 0;
            cpu.regs.f.set(FlagsReg::Zero, cpu.regs.a == 0);
            
            proc.done = true;
        },
        2 => {
            cpu.regs.a ^= bus.read(cpu.regs.hl());
            cpu.regs.f.bits = 0;
            cpu.regs.f.set(FlagsReg::Zero, cpu.regs.a == 0);
            
            proc.done = true;
        }
        _ => ()
    }
}
/// 0x06, 0x16, 0x26, 0x36, 0x0E, 0x1E, 0x2E, 0x3E
fn ld_ru8(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) {
    match proc.mcycle {
        2 => {
            let opcode = bus.read(cpu.regs.pc - 1);
            proc.tmp0 = cpu.fetch(bus);
            
            match (opcode & 0b00111000) >> 3 {
                0 => cpu.regs.b = proc.tmp0,
                1 => cpu.regs.c = proc.tmp0,
                2 => cpu.regs.d = proc.tmp0,
                3 => cpu.regs.e = proc.tmp0,
                4 => cpu.regs.h = proc.tmp0,
                5 => cpu.regs.l = proc.tmp0,
                6 => return,
                7 => cpu.regs.a = proc.tmp0,
                _ => panic!("unreachable")
            }
            
            proc.done = true;
        },
        3 => {
            bus.write(cpu.regs.hl(), proc.tmp0);
            
            proc.done = true;
        },
        _ => ()
    }
}
/// 0x40 - 0x7F (EXCEPT 0x76 aka HALT)
fn ld_rr(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) {
    match proc.mcycle {
        1 => {
            let opcode = bus.read(cpu.regs.pc - 1);
            let y = (opcode & 0b00111000) >> 3;
            let z = opcode & 0b00000111;
            proc.tmp0 = y;
            proc.tmp1 = z;
            
            let val = match z {
                0 => cpu.regs.b,
                1 => cpu.regs.c,
                2 => cpu.regs.d,
                3 => cpu.regs.e,
                4 => cpu.regs.h,
                5 => cpu.regs.l,
                6 => return,
                7 => cpu.regs.a,
                _ => panic!("unreachable")
            };
            
            match y {
                0 => cpu.regs.b = val,
                1 => cpu.regs.c = val,
                2 => cpu.regs.d = val,
                3 => cpu.regs.e = val,
                4 => cpu.regs.h = val,
                5 => cpu.regs.l = val,
                6 => return,
                7 => cpu.regs.a = val,
                _ => panic!("unreachable")
            };
            
            proc.done = true;
        },
        2 => {
            let y = proc.tmp0;
            let z = proc.tmp1;
            
            if y == 6 {
                bus.write(cpu.regs.hl(), match z {
                    0 => cpu.regs.b,
                    1 => cpu.regs.c,
                    2 => cpu.regs.d,
                    3 => cpu.regs.e,
                    4 => cpu.regs.h,
                    5 => cpu.regs.l,
                    6 => panic!("unreachable"),
                    7 => cpu.regs.a,
                    _ => panic!("unreachable")
                })
            } else if z == 6 {
                let val = bus.read(cpu.regs.hl());
                match y {
                    0 => cpu.regs.b = val,
                    1 => cpu.regs.c = val,
                    2 => cpu.regs.d = val,
                    3 => cpu.regs.e = val,
                    4 => cpu.regs.h = val,
                    5 => cpu.regs.l = val,
                    6 => panic!("unreachable"),
                    7 => cpu.regs.a = val,
                    _ => panic!("unreachable")
                }
            } else { panic!("unreachable") }
            
            proc.done = true;
        }
        _ => ()
    }
}


/// 0xEE
fn xor_au8(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) {
    todo!()
}

/// Load A into indirect
/// 
/// 0x02, 0x12, 0x22, 0x32
fn ld_toindirect(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) {
    match proc.mcycle {
        1 => {
            let opcode = bus.read(cpu.regs.pc - 1);
            proc.tmp0 = (opcode & 0b00110000) >> 4; // p
        },
        2 => {
            let addr = match proc.tmp0 {
                0 => cpu.regs.bc(),
                1 => cpu.regs.de(),
                2 => {
                    let tmp = cpu.regs.hl();
                    cpu.regs.set_hl(tmp.wrapping_add(1));
                    
                    tmp
                },
                3 => {
                    let tmp = cpu.regs.hl();
                    cpu.regs.set_hl(tmp.wrapping_sub(1));
                    
                    tmp
                },
                _ => panic!("unreachable")
            };
            
            bus.write(addr, cpu.regs.a);
            
            proc.done = true;
        },
        _ => ()
    }
}
/// Load indirect into A
/// 
/// 0x
fn ld_fromindirect(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) {
    match proc.mcycle {
        1 => {
            let opcode = bus.read(cpu.regs.pc - 1);
            proc.tmp0 = (opcode & 0b00110000) >> 4; // p
        },
        2 => {
            let addr = match proc.tmp0 {
                0 => cpu.regs.bc(),
                1 => cpu.regs.de(),
                2 => {
                    let tmp = cpu.regs.hl();
                    cpu.regs.set_hl(tmp.wrapping_add(1));
                    
                    tmp
                },
                3 => {
                    let tmp = cpu.regs.hl();
                    cpu.regs.set_hl(tmp.wrapping_sub(1));
                    
                    tmp
                },
                _ => panic!("unreachable")
            };
            
            cpu.regs.a = bus.read(addr);
            
            proc.done = true;
        },
        _ => ()
    }
}


/// 0xC1, 0xD1, 0xE1, 0xF1
fn pop(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) {
    match proc.mcycle {
        1 => {
            let opcode = bus.read(cpu.regs.pc - 1);
            proc.tmp0 = (opcode & 0b00110000) >> 4;
        },
        2 => match proc.tmp0 {
            0 => cpu.regs.c = cpu.stack_pop(bus),
            1 => cpu.regs.e = cpu.stack_pop(bus),
            2 => cpu.regs.l = cpu.stack_pop(bus),
            3 => cpu.regs.f.bits = cpu.stack_pop(bus) & 0xF0,
            _ => panic!("unreachable")
        },
        3 => {
            match proc.tmp0 {
                0 => cpu.regs.b = cpu.stack_pop(bus),
                1 => cpu.regs.d = cpu.stack_pop(bus),
                2 => cpu.regs.h = cpu.stack_pop(bus),
                3 => cpu.regs.a = cpu.stack_pop(bus),
                _ => panic!("unreachable")
            }
            
            proc.done = true;
        }
        _ => ()
    }
}
/// 0xC5, 0xD5, 0xE5, 0xF5
fn push(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) {
    match proc.mcycle {
        1 => {
            let opcode = bus.read(cpu.regs.pc - 1);
            proc.tmp0 = (opcode & 0b00110000) >> 4;
        },
        2 => (),
        3 => match proc.tmp0 {
            0 => cpu.stack_push(bus, cpu.regs.c),
            1 => cpu.stack_push(bus, cpu.regs.e),
            2 => cpu.stack_push(bus, cpu.regs.l),
            3 => cpu.stack_push(bus, cpu.regs.f.bits & 0xF0),
            _ => panic!("unreachable")
        },
        4 => {
            match proc.tmp0 {
                0 => cpu.stack_push(bus, cpu.regs.b),
                1 => cpu.stack_push(bus, cpu.regs.d),
                2 => cpu.stack_push(bus, cpu.regs.h),
                3 => cpu.stack_push(bus, cpu.regs.a),
                _ => panic!("unreachable")
            }
            
            proc.done = true;
        }
        _ => ()
    }
}

/// 0xCD
fn call_u16(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) {
    match proc.mcycle {
        2 => proc.tmp0 = cpu.fetch(bus),
        3 => proc.tmp1 = cpu.fetch(bus),
        4 => (),
        5 => cpu.stack_push(bus, cpu.regs.pchi()),
        6 => {
            cpu.stack_push(bus, cpu.regs.pclo());
            
            cpu.regs.set_pclo(proc.tmp0);
            cpu.regs.set_pchi(proc.tmp1);
            
            proc.done = true;
        },
        _ => ()
    }
}

fn ret(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) {
    match proc.mcycle {
        2 => proc.tmp0 = cpu.stack_pop(bus),
        3 => proc.tmp1 = cpu.stack_pop(bus),
        4 => {
            cpu.regs.set_pclo(proc.tmp0);
            cpu.regs.set_pchi(proc.tmp1);
            
            proc.done = true;
        },
        _ => ()
    }
}

/// 0xF9
fn ld_sphl(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) {
    match proc.mcycle {
        2 => {
            cpu.regs.sp = cpu.regs.hl();
            
            proc.done = true;
        }
        _ => ()
    }
}

/// 0x01, 0x11, 0x21, 0x31
fn ld_rpu16(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) {
    match proc.mcycle {
        1 => {
            let opcode = bus.read(cpu.regs.pc - 1);
            proc.tmp0 = (opcode & 0b00110000) >> 4;
        },
        2 => match proc.tmp0 {
            0 => cpu.regs.c = cpu.fetch(bus),
            1 => cpu.regs.e = cpu.fetch(bus),
            2 => cpu.regs.l = cpu.fetch(bus),
            3 => { let tmp = cpu.fetch(bus); cpu.regs.set_splo(tmp); },
            _ => panic!("unreachable")
        },
        3 => {
            match proc.tmp0 {
                0 => cpu.regs.b = cpu.fetch(bus),
                1 => cpu.regs.d = cpu.fetch(bus),
                2 => cpu.regs.h = cpu.fetch(bus),
                3 => { let tmp = cpu.fetch(bus); cpu.regs.set_sphi(tmp); },
                _ => panic!("unreachable")
            }
            
            proc.done = true;
        }
        _ => ()
    }
}

/// 0xE2
fn ld_toio_c(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) {
    match proc.mcycle {
        2 => {
            bus.write(0xFF00 + (cpu.regs.c as u16), cpu.regs.a);
            
            proc.done = true;
        },
        _ => ()
    }
}
/// 0xF2
fn ld_fromio_c(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) {
    match proc.mcycle {
        2 => {
            cpu.regs.a = bus.read(0xFF00 + (cpu.regs.c as u16));
            
            proc.done = true;
        },
        _ => ()
    }
}
/// 0xE0
fn ld_toio_u8(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) {
    match proc.mcycle {
        2 => proc.tmp0 = cpu.fetch(bus),
        3 => {
            bus.write(0xFF00 + (proc.tmp0 as u16), cpu.regs.a);
            
            proc.done = true;
        },
        _ => ()
    }
}
/// 0xF0
fn ld_fromio_u8(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) {
    match proc.mcycle {
        2 => proc.tmp0 = cpu.fetch(bus),
        3 => {
            cpu.regs.a = bus.read(0xFF00 + (proc.tmp0 as u16));
            
            proc.done = true;
        },
        _ => ()
    }
}

// CB-Prefixed Instructions
//   Due to how the initial pipeline fetches the 0xCB "opcode", and then the real opcode after,
// these instructions still have the correct overall timing, but the real opcode fetch is happening
// one cycle earlier than it should be.
//   This quirk shouldn't be a problem unless we're fetching from something that could be manipulated
// between fetch cycles.

fn rot(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) { //TODO: Test this instruction to make sure everything is accurate
    match proc.mcycle {
        2 => {
            let opcode = bus.read(cpu.regs.pc - 1);
            let y = (opcode & 0b00111000) >> 3;
            let z = opcode & 0b00000111;
            proc.tmp0 = y;
            
            let reg = match z {
                0 => &mut cpu.regs.b,
                1 => &mut cpu.regs.c,
                2 => &mut cpu.regs.d,
                3 => &mut cpu.regs.e,
                4 => &mut cpu.regs.h,
                5 => &mut cpu.regs.l,
                6 => return,
                7 => &mut cpu.regs.a,
                _ => panic!("unreachable")
            };
            
            let carry = cpu.regs.f.intersects(FlagsReg::Carry) as u8;
            let (carry, result) = match y { // rot[y]
                0 => (*reg & 0x80, reg.rotate_left(1)), // RLC - Rotate Left
                1 => (*reg & 0x01, reg.rotate_right(1)), // RRC - Rotate Right
                2 => (*reg & 0x80, (*reg << 1) | carry), // RL  - Rotate Left Through Carry
                3 => (*reg & 0x01, (*reg >> 1) | (carry << 7)), // RR  - Rotate Right Through Carry
                4 => (*reg & 0x80, *reg << 1), // SLA - Shift Left Arithmetic
                5 => (*reg & 0x01, (*reg >> 1) | (*reg & 0x80)), // SRA - Shift Right Arithmetic
                6 => (carry, (*reg >> 4) | (*reg & 0xF0)), // SWAP - Swap Nibbles
                7 => (*reg & 0x01, *reg >> 1), // SRL - Shift Right Logical
                _ => panic!("unreachable")
            };
            cpu.regs.f.bits = 0;
            cpu.regs.f.set(FlagsReg::Zero, result == 0);
            cpu.regs.f.set(FlagsReg::Carry, carry != 0);
            *reg = result;
            
            proc.done = true;
        },
        3 => proc.tmp1 = bus.read(cpu.regs.hl()),
        4 => {
            let val = proc.tmp1;
            
            let carry = cpu.regs.f.intersects(FlagsReg::Carry) as u8;
            let (carry, result) = match proc.tmp0 { // rot[y]
                0 => (val & 0x80, val.rotate_left(1)), // RLC - Rotate Left
                1 => (val & 0x01, val.rotate_right(1)), // RRC - Rotate Right
                2 => (val & 0x80, (val << 1) | carry), // RL  - Rotate Left Through Carry
                3 => (val & 0x01, (val >> 1) | (carry << 7)), // RR  - Rotate Right Through Carry
                4 => (val & 0x80, val << 1), // SLA - Shift Left Arithmetic
                5 => (val & 0x01, (val >> 1) | (val & 0x80)), // SRA - Shift Right Arithmetic
                6 => (carry, (val >> 4) | (val & 0xF0)), // SWAP - Swap Nibbles
                7 => (val & 0x01, val >> 1), // SRL - Shift Right Logical
                _ => panic!("unreachable")
            };
            cpu.regs.f.bits = 0;
            cpu.regs.f.set(FlagsReg::Zero, result == 0);
            cpu.regs.f.set(FlagsReg::Carry, carry != 0);
            bus.write(cpu.regs.hl(), val);
            
            proc.done = true;
        },
        _ => () 
    }
}

/// 0b01nn_nnnn
fn bit(proc: &mut InstructionProcedure, cpu: &mut Cpu, bus: &mut Bus) {
    match proc.mcycle {
        2 => {
            let opcode = bus.read(cpu.regs.pc - 1);
            let y = (opcode & 0b00111000) >> 3;
            let z = opcode & 0b00000111;
            proc.tmp0 = y;
            
            let val = match z {
                0 => cpu.regs.b,
                1 => cpu.regs.c,
                2 => cpu.regs.d,
                3 => cpu.regs.e,
                4 => cpu.regs.h,
                5 => cpu.regs.l,
                6 => return,
                7 => cpu.regs.a,
                _ => panic!("unreachable")
            } & (1 << y);
            
            cpu.regs.f.set(FlagsReg::Zero, val == 0);
            cpu.regs.f.set(FlagsReg::Negative, false);
            cpu.regs.f.set(FlagsReg::HalfCarry, true);
            
            proc.done = true;
        },
        3 => {
            let val = bus.read(cpu.regs.hl()) & (1 << proc.tmp0);
            
            cpu.regs.f.set(FlagsReg::Zero, val == 0);
            cpu.regs.f.set(FlagsReg::Negative, false);
            cpu.regs.f.set(FlagsReg::HalfCarry, true);
            
            proc.done = true;
        }
        _ => ()
    }
}