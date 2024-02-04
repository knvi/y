#[derive(Debug, Eq, Clone, PartialEq)]
pub enum Object {
    HitObject {
        timestamp: i32 // todo timestamps
    },
    LongNote {
        start_timestamp: i32, // start holding
        end_timestamp: i32, // finish holding
    }
}