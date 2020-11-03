extern crate serde_json;

use byteorder::{BigEndian, ByteOrder};
use hashbrown::{HashMap, HashSet};
use log::*;
use nbt;
use std::io::Cursor;

use crate::minecraft::block::{Block, Coordinate};

#[derive(Clone, Debug)]
pub struct Chunk {
    pub timestamp: u32,
    pub blocks: HashMap<Coordinate, Block>,
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
                                            let y_idx = Chunk::get_y_index(section);
                                            let palette = Chunk::get_palette(section);

                                            Chunk::add_section_blocks(
                                                &mut blocks,
                                                y_idx,
                                                &palette,
                                                section,
                                            );
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

    fn add_section_blocks(
        blocks: &mut HashMap<Coordinate, Block>,
        y_idx: i8,
        palette: &Vec<Block>,
        section: &nbt::Map<String, nbt::Value>,
    ) {
        if let Some(blockstates) = section.get("BlockStates") {
            match blockstates {
                nbt::Value::LongArray(blockstates) => {
                    let IGNORED_BLOCKTYPES: HashSet<&'static str> =
                        ["minecraft:air", "minecraft:cave_air"]
                            .iter()
                            .cloned()
                            .collect();

                    let bit_width = (palette.len() as f64).log2() as usize;
                    let bit_width = if bit_width < 4 { 4 } else { bit_width };

                    let base: u128 = 2;
                    let mut bitmask: u128 = 0;
                    for i in 0..bit_width {
                        bitmask += base.pow(i as u32);
                    }
                    let bitmask: u128 = bitmask;
                    let mut bitmask_shift = 0;

                    let mut blockcount: isize = 0;
                    let mut i = 1;
                    while i < blockstates.len() {
                        let lower_blockstates = i64::from_be(blockstates[i - 1]) as u128;
                        let upper_blockstates = i64::from_be(blockstates[i]) as u128;

                        let blockstates_chunk = (upper_blockstates << 64) | lower_blockstates;

                        while bitmask_shift <= (128 - 1 - bit_width) {
                            // info!("bitmask_shift {}", bitmask_shift);
                            let shifted_bitmask = bitmask << bitmask_shift;
                            let index =
                                ((blockstates_chunk & shifted_bitmask) >> bitmask_shift) as usize;

                            if index < palette.len() {
                                let block = &palette[index];
                                if !IGNORED_BLOCKTYPES.contains::<str>(&block.id.to_string()) {
                                    let coord = Coordinate {
                                        x: (blockcount % 16) as i64,
                                        z: ((blockcount / 16) % 16) as i64,
                                        y: (((blockcount / (16 * 16)) % 16) + (y_idx as isize * 16))
                                            as i64,
                                    };
                                    // warn!("{} {} {}", coord.y, coord.z, coord.x);
                                    blocks.insert(coord, block.clone());
                                }
                            } else {
                                // TODO: Find out why these are missing the palette!
                                // warn!(
                                //     "Palette miss: bit_width {}, bitmask_shift {}, blockcount {}, index {}, max {}",
                                //     bit_width,
                                //     bitmask_shift,
                                //     blockcount,
                                //     index,
                                //     palette.len()
                                // );
                            }

                            blockcount += 1;
                            bitmask_shift += bit_width;
                        }
                        bitmask_shift -= 64;
                        i += 1;
                    }
                }
                _ => {}
            }
        }
    }

    fn get_y_index(section: &nbt::Map<String, nbt::Value>) -> i8 {
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
