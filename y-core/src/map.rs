use crate::object::Object;

// Singular line in the map
#[derive(Debug, Eq, Clone, PartialEq)]
pub struct Lane {
    pub objects: Vec<Object>
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct TimeSignature {
    pub beat_count: u8,
    pub beat_unit: u8,
}

pub struct Map {
    pub song_artist: Option<String>,
    pub song_name: Option<String>,

    pub difficulty_name: Option<String>,
    pub mapper: Option<String>,

    pub lanes: Vec<Lane>
}