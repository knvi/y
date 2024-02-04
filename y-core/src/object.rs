use crate::timing::MapTimestamp;

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum Object {
    HitObject {
        timestamp: MapTimestamp // todo timestamps
    },
    LongNote {
        start_timestamp: MapTimestamp, // start holding
        end_timestamp: MapTimestamp, // finish holding
    }
}

impl Object {
    pub fn start_timestamp(&self) -> MapTimestamp {
        match *self {
            Object::HitObject { timestamp } => timestamp,
            Object::LongNote { start_timestamp, .. } => start_timestamp
        }
    }

    pub fn end_timestamp(&self) -> MapTimestamp {
        match *self {
            Object::HitObject { timestamp } => timestamp,
            Object::LongNote { end_timestamp, .. } => end_timestamp
        }
    }
}