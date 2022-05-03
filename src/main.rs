use std::time::{Duration, Instant};
use clap::{AppSettings, Arg, Command};
use log::{info, LevelFilter};
use minifb::{Key, KeyRepeat, Scale, ScaleMode, Window, WindowOptions};
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
    gb.bus.get_mut().cart.rom.extend_from_slice(include_bytes!("../testroms/mooneye/acceptance/serial/boot_sclk_align-dmgABCmgb.gb"));
    
    let mut frames = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let start = Instant::now();
        
        //if window.is_key_pressed(Key::Space, KeyRepeat::No) || window.is_key_down(Key::M) {
        //    info!("f: {}", frames);
            for _ in 0..(2097152 / 2 / 60) {
                gb.mcycle();
            }
        //    frames += 1;
        //}
        
        
        gb.bus.get().ppu.render(&mut window_buf);
        window.update_with_buffer(&window_buf, width, height).unwrap();
        
        let elapsed = start.elapsed().as_secs_f64();
        //info!("Time: {:.3}us | Factor: {:.3}", elapsed * 1000000.0, (1.0 / 60.0) / elapsed);
    }
    
    
}
