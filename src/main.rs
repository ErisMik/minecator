use clap::{load_yaml, App};
use log::*;
use rayon::prelude::*;
use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};
use std::path::Path;

mod file_ops;
mod minecraft;
mod progress;
mod render;

fn main() -> std::io::Result<()> {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed).unwrap();

    let yaml = load_yaml!("cli.yml");
    let args = App::from_yaml(yaml).get_matches();
    let world_dir = args.value_of("world_dir").unwrap();
    let world_dirpath = Path::new(world_dir);
    info!("Using minecraft world: {}", world_dirpath.to_str().unwrap());

    let leveldat_filename = world_dirpath.join("level.dat");
    let leveldat = minecraft::leveldat::LevelDat::new(leveldat_filename.to_str().unwrap())?;
    info!("Minecraft version: {:?}", leveldat.version());

    let region_dirpath = world_dirpath.join("region/");
    let region_files = file_ops::get_all_files(&region_dirpath)?;
    progress::progress_init(region_files.len() as u64 * 1024, "Loading chunks");
    let regions = region_files
        .par_iter()
        .map(|region_filename| {
            minecraft::region::Region::new(&region_filename.to_str().unwrap()).unwrap()
        })
        .collect::<Vec<_>>();
    progress::PROGRESS_BAR.finish();

    return Ok(());
}
