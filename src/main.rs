extern crate core;

use std::fs::File;
use std::io::{LineWriter, Write};
use std::time::{Duration, Instant};
use clap::{AppSettings, Arg, Command};
use log::{info, LevelFilter};
use minifb::{Key, KeyRepeat, Scale, ScaleMode, Window, WindowOptions};
use crate::arch::{Gameboy, SystemMode};

pub mod arch;
pub mod logger;
pub mod util;

fn main() {
    let matches = Command::new("gbcrs")
        .version(clap::crate_version!())
        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .takes_value(true)
            .default_missing_value("debug")
            .default_value("info")
            .possible_values(["error", "warn", "info", "debug", "trace"])
            .help("Specify the console log level. Environment variable 'RUST_LOG' will override this option."))
        .arg(Arg::new("log")
            .long("log")
            .hide(true))
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
    const width: usize = 160;
    const height: usize = 144;
    //const width: usize = 256;
    //const height: usize = 256;
    
    
    let mut window = Window::new("gbcrs", width, height, WindowOptions {
        borderless: false,
        title: true,
        resize: false,
        scale: Scale::X4,
        scale_mode: ScaleMode::Stretch,
        topmost: false,
        transparency: false,
        none: false
    }).unwrap();
    window.limit_update_rate(Some(Duration::from_secs_f64(1.0 / 60.0)));
    //window.limit_update_rate(None);
    let mut window_buf = [0u32; width * height];
    
    let mut gb = Gameboy::new(SystemMode::Gameboy);
    gb.bus.get_mut().boot_rom = *include_bytes!("../bootroms/DMG1.rom");
    //gb.bus.get_mut().cart.rom.extend_from_slice(include_bytes!("../testroms/mooneye/acceptance/serial/boot_sclk_align-dmgABCmgb.gb"));
    gb.bus.get_mut().cart.rom.extend_from_slice(include_bytes!("../testroms/blargg/cpu_instrs/individual/03-op sp,hl.gb"));
    
    let mut writer = None;
    if matches.is_present("log") {
        std::fs::remove_file("log.txt").unwrap_or_default();
        let file = File::create("log.txt").unwrap();
        writer = Some(LineWriter::new(file));
    }
    
    let mut line_count = 0usize;
    let mut last_instr = 0;
    //let mut frames = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        //let start = Instant::now();
        
        //if window.is_key_pressed(Key::Space, KeyRepeat::No) || window.is_key_down(Key::M) {
        //    info!("f: {}", frames);
            for _ in 0..(2097152 / 2 / 60) {
                if let Some(writer) = writer.as_mut() {
                    use crate::arch::BusAccessable;
                    let bus = gb.bus.get_mut();
                    let cpu = &gb.bus.get().cpu;
                    let count = cpu.instr_count;
                    if (count != last_instr || count == 0) && bus.boot_disabled > 0 {
                        writer.write_all(format!("A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})\n",
                            cpu.regs.a, cpu.regs.f.bits(), cpu.regs.b, cpu.regs.c, cpu.regs.d, cpu.regs.e, cpu.regs.h, cpu.regs.l, cpu.regs.sp, cpu.regs.pc, bus.read(cpu.regs.pc), bus.read(cpu.regs.pc + 1), bus.read(cpu.regs.pc + 2), bus.read(cpu.regs.pc + 3)
                        ).as_bytes()).unwrap();
                        line_count += 1;
                    }
                    last_instr = count;
                }
                if line_count == 180000 {
                    if let Some(mut writer) = writer {
                        writer.flush().unwrap();
                    }
                    info!("Stopping");
                    std::thread::sleep(Duration::from_secs_f64(1.5));
                    
                    return;
                }
                
                gb.mcycle();
            }
        //    frames += 1;
        //}
        if last_instr >= 1068423 { break }
        
        gb.bus.get().ppu.render(&mut window_buf);
        window.update_with_buffer(&window_buf, width, height).unwrap();
        
        //let elapsed = start.elapsed().as_secs_f64();
        //info!("Time: {:.3}us | Factor: {:.3}", elapsed * 1000000.0, (1.0 / 60.0) / elapsed);
    }
    
    if let Some(mut writer) = writer {
        writer.flush().unwrap();
    }
}
