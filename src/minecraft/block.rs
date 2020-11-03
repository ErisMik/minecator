use hashbrown::HashMap;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Coordinate {
    pub y: i64,
    pub x: i64,
    pub z: i64,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub id: String,
    pub properties: HashMap<String, String>,
}
