use std::time::Instant;
use clap::{AppSettings, Arg, Command};
use log::{info, LevelFilter};
use crate::arch::{Gameboy, SystemMode};

pub mod arch;
pub mod logger;
pub mod util;

fn main() {
    let matches = Command::new("remote64-server")
        .version(clap::crate_version!())
        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .takes_value(true)
            .default_missing_value("debug")
            .default_value("info")
            .possible_values(["error", "warn", "info", "debug", "trace"])
            .help("Specify the console log level. Environment variable 'RUST_LOG' will override this option."))
        .next_line_help(true)
        .setting(AppSettings::DeriveDisplayOrder)
        .get_matches();
    
    // Setup program-wide logger format
    let level = match std::env::var("RUST_LOG").unwrap_or(matches.value_of("verbose").unwrap_or("info").to_owned()).as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info
    };
    {
        let mut logbuilder = logger::builder();
        logbuilder.filter_level(level);
        logbuilder.init();
    }
    
    
    let mut gb = Gameboy::new(SystemMode::Gameboy);
    gb.bus.get_mut().boot_rom = *include_bytes!("../bootroms/DMG1.rom");
    gb.bus.get_mut().cart.rom.extend_from_slice(include_bytes!("/data/storage/roms/gb-nointro/Boxxle (USA).gb"));
    
    let mut log = String::new();
    
    let start = Instant::now();
    
    use crate::arch::BusAccessable;
    let mut last_count = 0usize;
    let mut counter = 0usize;
    loop {
        /*let bus = gb.bus.get_mut();
        let cpu = &gb.bus.get().cpu;
        if cpu.instr_count > last_count || cpu.instr_count == 0 {
            last_count = cpu.instr_count;
            let s = format!("A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})\n",
                cpu.regs.a, cpu.regs.f.bits(), cpu.regs.b, cpu.regs.c, cpu.regs.d, cpu.regs.e, cpu.regs.h, cpu.regs.l, cpu.regs.sp, cpu.regs.pc, bus.read(cpu.regs.pc), bus.read(cpu.regs.pc + 1), bus.read(cpu.regs.pc + 2), bus.read(cpu.regs.pc + 3)
            );
            log.push_str(&s);
        }
        
        if cpu.instr_count == 47932 {
            std::fs::write("log.txt", log).unwrap();
            std::process::exit(0);
        }*/
        
        gb.mcycle();
        counter += 1;
        
        if counter == 2097152 * 60 {
            let elapsed = start.elapsed().as_secs_f64();
            info!("Time: {:.3}us | Factor: {:.3}", elapsed * 1000000.0, 60.0 / elapsed);
            
            break;
        }
    }
    
    
}
