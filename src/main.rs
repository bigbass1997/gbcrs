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
    
    let start = Instant::now();
    
    let mut counter = 0usize;
    loop {
        gb.mcycle();
        counter += 1;
        
        if counter == 2097152 * 60 * 60 * 24 {
            let elapsed = start.elapsed().as_secs_f64();
            info!("Time: {:.3}us | Factor: {:.3}", elapsed * 1000000.0, (60.0 * 60.0 * 24.0) / elapsed);
            
            break;
        }
    }
    
    
}
