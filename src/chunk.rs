use byteorder::{BigEndian, ByteOrder};
use nbt;
use std::io::Cursor;

#[derive(Debug)]
pub struct Chunk {
    timestamp: u32,
    blob: nbt::Blob,
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

        return Ok(Chunk {
            timestamp: timestamp,
            blob: nbt_data,
        });
    }
}
