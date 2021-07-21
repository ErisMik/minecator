use clap;
use log::*;
use rayon::prelude::*;
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};
use std::path::PathBuf;

mod file_ops;
mod minecraft;
mod progress;
mod render;

fn main() -> std::io::Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .expect("Unable to init logger");

    let args = clap::App::new("Minecator")
        .version("1.0")
        .arg(
            clap::Arg::with_name("world_dir")
                .value_name("WORLD_DIR")
                .help("The path to the minecraft world dir ")
                .required(true)
                .index(1),
        )
        .get_matches();

    let world_dir = args.value_of("world_dir").unwrap();
    let world_dirpath = PathBuf::from(world_dir);
    info!("Using minecraft world: {:?}", world_dirpath);

    let leveldat_filename = world_dirpath.join("level.dat");
    let leveldat = minecraft::leveldat::LevelDat::new(leveldat_filename.clone())?;
    info!("Minecraft version: {:?}", leveldat.version());

    let region_dirpath = world_dirpath.join("region/");
    let region_files = file_ops::get_all_files(region_dirpath)?;
    progress::progress_init(
        region_files.len() as u64 * 1024,
        String::from("Loading chunks"),
    );
    let regions = region_files
        .par_iter()
        .map(|region_filename| {
            minecraft::region::Region::new(&region_filename.to_str().unwrap()).unwrap()
        })
        .collect::<Vec<_>>();
    progress::PROGRESS_BAR.finish();
    info!("Chunks loaded");

    info!("Rendering & Saving image");
    render::render_image(regions);

    return Ok(());
}
