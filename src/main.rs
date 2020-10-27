use clap::{load_yaml, App};
use std::path::Path;

mod chunk;
mod leveldat;
mod region;

fn main() -> std::io::Result<()> {
    let yaml = load_yaml!("cli.yml");
    let args = App::from_yaml(yaml).get_matches();
    let world_dir = args.value_of("world_dir").unwrap();
    let world_dirpath = Path::new(world_dir);
    println!("Using minecraft world: {}", world_dirpath.to_str().unwrap());

    let leveldat_filename = world_dirpath.join("level.dat");
    let leveldat = leveldat::LevelDat::new(leveldat_filename.to_str().unwrap())?;
    println!("Minecraft version: {:?}", leveldat.version());

    let region_filename = world_dirpath.join("region/r.0.0.mca");
    let region = region::Region::new(&region_filename.to_str().unwrap())?;

    return Ok(());
}
