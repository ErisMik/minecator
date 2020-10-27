use nbt;
use std::fs::File;

#[derive(Debug)]
pub struct LevelDat {
    filename: String,
    blob: nbt::Blob,
}

impl LevelDat {
    pub fn new(leveldat_filename: &str) -> std::io::Result<LevelDat> {
        let mut leveldat_file = File::open(leveldat_filename)?;
        let nbt_data = nbt::Blob::from_gzip_reader(&mut leveldat_file)?;

        return Ok(LevelDat {
            filename: String::from(leveldat_filename),
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
