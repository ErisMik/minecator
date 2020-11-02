extern crate serde_json;

use byteorder::{BigEndian, ByteOrder};
use hashbrown::HashMap;
use log::*;
use nbt;
use std::io::Cursor;

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Coordinate {
    y: i64,
    x: i64,
    z: i64,
}

#[derive(Clone, Debug)]
pub struct Block {
    id: String,
    properties: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Chunk {
    timestamp: u32,
    blocks: HashMap<Coordinate, Block>,
}

impl Chunk {
    pub fn new(timestamp: u32, data: Vec<u8>) -> std::io::Result<Chunk> {
        let chunk_length = BigEndian::read_u32(&data[0..4]) as usize;
        let compression_type = u8::from_be(data[4]);
        let mut data_reader = Cursor::new(&data[5..chunk_length]);

        let nbt_data = match compression_type {
            1 => nbt::Blob::from_gzip_reader(&mut data_reader)?,
            2 => nbt::Blob::from_zlib_reader(&mut data_reader)?,
            _ => nbt::Blob::from_reader(&mut data_reader)?,
        };

        let mut blocks: HashMap<Coordinate, Block> = HashMap::new();

        if let Some(level) = nbt_data.get("Level") {
            match level {
                nbt::Value::Compound(level) => {
                    if let Some(sections) = level.get("Sections") {
                        match sections {
                            nbt::Value::List(sections) => {
                                for section in sections {
                                    match section {
                                        nbt::Value::Compound(section) => {
                                            let y = Chunk::get_y_coord(section);
                                            let palette = Chunk::get_palette(section);

                                            blocks = Chunk::get_blocks(y, &palette, section);
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        return Ok(Chunk {
            timestamp: timestamp,
            blocks: blocks,
        });
    }

    fn get_blocks(y: i8, palette: &Vec<Block>, section: &nbt::Map<String, nbt::Value>) -> HashMap<Coordinate, Block> {
        let mut blocks: HashMap<Coordinate, Block> = HashMap::new();

        if let Some(blockstates) = section.get("BlockStates") {
            match blockstates {
                nbt::Value::LongArray(blockstates) => {
                    let size = blockstates.len() * 64;
                    let bit_width = size / 4096;

                    let base: i128 = 2;
                    let mut bitmask: i128 = 0;
                    for i in 0..bit_width {
                        bitmask += base.pow(i as u32);
                    }
                    let bitmask = bitmask;
                    let mut bitmask_shift = 0;

                    let mut i = 1;
                    while i < blockstates.len() {
                        let lower_blockstates = i64::from_be(blockstates[0]) as i128;
                        let upper_blockstates = i64::from_be(blockstates[1]) as i128;

                        let blockstates_chunk = lower_blockstates + (upper_blockstates << 64);

                        let mut x = 0;
                        while bitmask_shift <= (128 - bit_width) {
                            let shifted_bitmask = bitmask << bitmask_shift;
                            let index = ((blockstates_chunk & shifted_bitmask) >> bitmask_shift) as usize;

                            if index < palette.len() {
                                let block = &palette[index];
                                let coord = Coordinate {
                                    y: y as i64,
                                    x: (x % 16) as i64,
                                    z: ((i / 16) % 16) as i64,
                                };
                                blocks.insert(coord, block.clone());
                            } else {
                                //TODO: Figure out why these are missing
                                // warn!("Palette miss");
                            }
                            bitmask_shift += bit_width;
                            x += 1;
                        }
                        bitmask_shift -= 64;
                        i += 1;
                    }
                }
                _ => {}
            }
        }

        return blocks;
    }

    fn get_y_coord(section: &nbt::Map<String, nbt::Value>) -> i8 {
        let y: i8 = match section.get("Y") {
            Some(y) => match y {
                nbt::Value::Byte(y) => *y,
                _ => -1,
            },
            _ => -1,
        };
        return y;
    }

    fn get_palette_properties(block: &nbt::Map<String, nbt::Value>) -> HashMap<String, String> {
        let mut properties_map = HashMap::new();

        if let Some(properties) = block.get("Properties") {
            match properties {
                nbt::Value::Compound(properties) => {
                    for (key, value) in properties.iter() {
                        let value = match value {
                            nbt::Value::String(value) => value.to_string(),
                            _ => String::from(""),
                        };
                        properties_map.insert(key.to_string(), value);
                    }
                }
                _ => {}
            }
        }
        return properties_map;
    }

    fn get_palette(section: &nbt::Map<String, nbt::Value>) -> Vec<Block> {
        let mut palette_vec = Vec::new();

        if let Some(palette) = section.get("Palette") {
            match palette {
                nbt::Value::List(palette) => {
                    for block in palette {
                        match block {
                            nbt::Value::Compound(block) => {
                                let properties = Chunk::get_palette_properties(block);
                                let id =
                                    match block.get("Name").expect("Palette block has no name!") {
                                        nbt::Value::String(name) => name.to_string(),
                                        _ => String::from(""),
                                    };

                                palette_vec.push(Block {
                                    id: id,
                                    properties: properties,
                                });
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        return palette_vec;
    }
}
