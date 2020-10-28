use byteorder::{BigEndian, ByteOrder};
use std::fs::File;
use std::io::SeekFrom;
use std::io::{Read, Seek};

use crate::minecraft::chunk::Chunk;
use crate::progress;

#[derive(Debug)]
pub struct Region {
    x: i64,
    z: i64,
    chunks: Vec<Chunk>,
}

impl Region {
    fn mca_to_chunks(region_filename: &str) -> std::io::Result<Vec<Chunk>> {
        let mut region_file = File::open(region_filename)?;
        let mut chunks: Vec<Chunk> = Vec::new();

        for i in 0..1024 {
            progress::PROGRESS_BAR.inc(1);

            let mut location_buf = [0u8; 4];
            let mut timestamp_buf = [0u8; 4];

            let location_offset = i * 4;
            region_file.seek(SeekFrom::Start(location_offset))?;
            region_file.read(&mut location_buf)?;

            let timestamp_offset = location_offset + 4096;
            region_file.seek(SeekFrom::Start(timestamp_offset))?;
            region_file.read(&mut timestamp_buf)?;

            if BigEndian::read_u32(&location_buf) == 0 {
                continue;
            }

            let chunk_offset = BigEndian::read_u24(&location_buf[0..3]) as u64 * 4096;
            let chunk_size = u8::from_be(location_buf[3]) as usize * 4096;
            let mut chunk_data_buf: Vec<u8> = vec![0; chunk_size];
            region_file.seek(SeekFrom::Start(chunk_offset))?;
            region_file.read(&mut chunk_data_buf[..chunk_size])?;

            chunks.push(Chunk::new(
                BigEndian::read_u32(&timestamp_buf),
                chunk_data_buf,
            )?);
        }

        return Ok(chunks);
    }

    pub fn new(region_filename: &str) -> std::io::Result<Region> {
        let (x, z) = region_parsers::region_filename_parser(region_filename);
        let chunks = Region::mca_to_chunks(region_filename)?;

        return Ok(Region {
            x: x,
            z: z,
            chunks: chunks,
        });
    }
}

mod region_parsers {
    use nom::{
        bytes::complete::is_not, bytes::complete::tag, sequence::preceded,
        sequence::separated_pair, IResult,
    };
    use std::path::Path;

    fn coord_parser(input: &str) -> IResult<&str, (&str, &str)> {
        return separated_pair(is_not("."), tag("."), is_not("."))(input);
    }

    fn filename_parser(filepath: &str) -> IResult<&str, (&str, &str)> {
        let filename = Path::new(filepath).file_stem().unwrap().to_str().unwrap();
        return preceded(tag("r."), coord_parser)(filename);
    }

    pub fn region_filename_parser(filepath: &str) -> (i64, i64) {
        let (_, (x_str, z_str)) = filename_parser(filepath).unwrap();
        return (x_str.parse::<i64>().unwrap(), z_str.parse::<i64>().unwrap());
    }
}
