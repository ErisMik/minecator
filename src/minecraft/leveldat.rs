use nbt;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug)]
pub struct LevelDat {
    filename: PathBuf,
    blob: nbt::Blob,
}

impl LevelDat {
    pub fn new(leveldat_filename: PathBuf) -> std::io::Result<LevelDat> {
        let mut leveldat_file = File::open(leveldat_filename.clone())?;
        let nbt_data = nbt::Blob::from_gzip_reader(&mut leveldat_file)?;

        return Ok(LevelDat {
            filename: leveldat_filename,
            blob: nbt_data,
        });
    }

    pub fn version(&self) -> Option<&nbt::Value> {
        if let Some(data_value) = self.blob.get("Data") {
            match data_value {
                nbt::Value::Compound(data_map) => {
                    return Some(&data_map["Version"]);
                }
                _ => return None,
            }
        } else {
            return None;
        }
    }
}
