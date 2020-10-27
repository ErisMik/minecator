#[derive(Debug)]
pub struct Chunk {
    timestamp: u32,
    data: Vec<u8>,
}

impl Chunk {
    pub fn new(timestamp: u32, data: Vec<u8>) -> std::io::Result<Chunk> {
        return Ok(Chunk {
            timestamp: timestamp,
            data: data,
        });
    }
}
