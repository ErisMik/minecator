use log::*;
use std::path::PathBuf;

pub fn get_all_files(src: &PathBuf) -> std::io::Result<Vec<PathBuf>> {
    let dir = src.read_dir()?;
    let mut files = Vec::new();

    for file in dir {
        if file.is_err() {
            error!("{}", file.err().unwrap());
            continue;
        }

        let file = file.unwrap();
        let metadata = file.metadata();

        if metadata.is_err() {
            error!(
                "Error -- Reading metadata of {:?} {}",
                file.path(),
                metadata.err().unwrap()
            );
            continue;
        }

        let metadata = metadata.unwrap();
        let path = file.path();

        if metadata.is_file() {
            files.push(path);
        }
    }

    Ok(files)
}
