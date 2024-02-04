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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Map {
    // TODO: separate these out? Leave only the actual object info here?
    // Idea for separation: Mapset contains Difficulties, which have this info plus a Map which has
    // just lanes, timing, etc.
    /// Artist of the song.
    pub song_artist: Option<String>,
    /// Title of the song.
    pub song_title: Option<String>,
    /// Difficulty name.
    pub difficulty_name: Option<String>,
    /// Mapper's name.
    pub mapper: Option<String>,

    /// Filename of the audio track.
    pub audio_file: Option<String>,

    /// Lanes constituting the map.
    pub lanes: Vec<Lane>,
}