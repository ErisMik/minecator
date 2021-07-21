use hashbrown::HashMap;
use serde_json;
use std::hash::Hash;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Coordinate {
    pub y: i64,
    pub x: i64,
    pub z: i64,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Block {
    pub id: String,
    pub properties: HashMap<String, String>,
}

impl Hash for Block {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.id.as_bytes());
        state.write(serde_json::to_string(&self.properties).unwrap().as_bytes());
    }
}
