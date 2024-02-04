use std::sync::Arc;

use crate::{map::Map, object::Object, scroll::ScrollSpeed, timing::{GameTimestamp, GameTimestampDifference, MapTimestamp, TimestampConverter}};

#[derive(Debug, Clone, Eq, Copy, PartialEq)]
pub enum HitObjectState {
    NotHit,
    Hit {
        diff: GameTimestampDifference,
    }
}

#[derive(Debug, Clone, Eq, Copy, PartialEq)]
pub enum LongNoteState {
    NotHit,
    Held {
        diff: GameTimestampDifference,
    },
    Hit {
        press_diff: GameTimestampDifference,
        rel_diff: GameTimestampDifference
    },
    Missed {
        held: Option<MapTimestamp>,
        press_diff: Option<GameTimestampDifference>
    }
}

#[derive(Debug, Clone, Eq, Copy, PartialEq)]
pub enum ObjectState {
    HitObject(HitObjectState),
    LongNote(LongNoteState),
}

pub struct LaneState {
    pub object_states: Vec<ObjectState>,
    first_object: usize,
}

#[derive(Debug, Clone, Eq, Copy, PartialEq)]
pub struct Hit {
    pub timestamp: GameTimestamp,
    pub diff: GameTimestampDifference,
}

pub struct Game {
    pub map: Arc<Map>,
    pub scroll_speed: ScrollSpeed,

    pub cap_fps: bool,
    pub timestamp_converter: TimestampConverter,
    pub lane_states: Vec<LaneState>,
}

impl Game {
    pub fn new(mut map: Map) -> Self {
        let mut lane_states = Vec::with_capacity(map.lanes.len());

        for lane in &mut map.lanes {
            lane.objects.sort_unstable_by_key(Object::start_timestamp);
            for window in lane.objects.windows(2) {
                let (a, b) = (window[0], window[1]);
                assert!(a.end_timestamp() < b.start_timestamp());
            }

            let mut objects = Vec::with_capacity(lane.objects.len());
            for object in &lane.objects {
                let state = match object {
                    Object::HitObject { .. } => ObjectState::HitObject(HitObjectState::NotHit),
                    Object::LongNote { .. } => ObjectState::LongNote(LongNoteState::NotHit),
                };
                objects.push(state);
            }

            lane_states.push(LaneState {
                object_states: objects,
                first_object: 0
            });
        }

        let timestamp_converter = TimestampConverter {
            global_offset: GameTimestampDifference::from_millis(0)
        };

        Self {
            map: Arc::new(map),
            cap_fps: false,
            scroll_speed: ScrollSpeed(25),
            timestamp_converter,
            lane_states,
        }
    }

    pub fn has_active_objects(&self, lane: usize) -> bool {
        let lane_state = &self.lane_states[lane];
        lane_state.first_object < lane_state.object_states.len()
    }

    pub fn update(&mut self, lane: usize, timestamp: GameTimestamp) {
        if !self.has_active_objects(lane) {
            return;
        }

        let hit_window = GameTimestampDifference::from_millis(76);

        let map_timestamp = timestamp.to_map(&self.timestamp_converter);
        let map_hit_window = hit_window.to_map(&self.timestamp_converter);

        let lane_state = &mut self.lane_states[lane];
        let objects = &self.map.lanes[lane].objects[lane_state.first_object..];
        let object_states = &mut lane_state.object_states[lane_state.first_object..];

        for (object, state) in objects.iter().zip(object_states.iter_mut()) {
            lane_state.first_object += 1;

            if object.end_timestamp() + map_hit_window < map_timestamp {
                // can't be hit
                if let ObjectState::LongNote(state) = state {
                    if let LongNoteState::Held { diff } = *state {
                        *state = LongNoteState::Hit {
                            press_diff: diff,
                            rel_diff: hit_window,
                        };

                        // todo: add a circular queue
                    } else if *state == LongNoteState::NotHit {
                        *state = LongNoteState::Missed {
                            held: None,
                            press_diff: None,
                        };
                    } else {
                        unreachable!()
                    }
                }

                continue;
            }

            if object.start_timestamp() + map_hit_window < map_timestamp {
                if let ObjectState::LongNote(state) = state {
                    if *state == LongNoteState::NotHit {
                        *state = LongNoteState::Missed {
                            held: None,
                            press_diff: None,
                        };
                        continue;
                    }

                    if let LongNoteState::Held { .. } = state {

                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
            }

            lane_state.first_object -= 1;
            break;
        }
    }

    pub fn key_press(&mut self, lane: usize, timestamp: GameTimestamp) {
        self.update(lane, timestamp);
        if !self.has_active_objects(lane) {
            return;
        }

        let hit_window = GameTimestampDifference::from_millis(76);

        let map_timestamp = timestamp.to_map(&self.timestamp_converter);
        let map_hit_window = hit_window.to_map(&self.timestamp_converter);

        let lane_state = &mut self.lane_states[lane];
        let object = &self.map.lanes[lane].objects[lane_state.first_object];
        let state = &mut lane_state.object_states[lane_state.first_object];

        if map_timestamp >= object.start_timestamp() - map_hit_window {
            let diff = (map_timestamp - object.start_timestamp()).to_game(&self.timestamp_converter);

            match state {
                ObjectState::HitObject(ref mut state) => {
                    *state = HitObjectState::Hit { diff };
                    lane_state.first_object += 1;
                }
                ObjectState::LongNote(ref mut state) => {
                    *state = LongNoteState::Held { diff };
                }
            }

            // todo:" cricular queue"
        }
    }

    pub fn key_release(&mut self, lane: usize, timestamp: GameTimestamp) {
        self.update(lane, timestamp);
        if !self.has_active_objects(lane) {
            return;
        }

        let hit_window = GameTimestampDifference::from_millis(76);

        let map_timestamp = timestamp.to_map(&self.timestamp_converter);
        let map_hit_window = hit_window.to_map(&self.timestamp_converter);

        let lane_state = &mut self.lane_states[lane];
        let object = &self.map.lanes[lane].objects[lane_state.first_object];
        let state = &mut lane_state.object_states[lane_state.first_object];

        if let ObjectState::LongNote(state) = state {
            if let LongNoteState::Held { diff } = *state {
                if map_timestamp >= object.start_timestamp() - map_hit_window {
                    let rel_diff = (map_timestamp - object.start_timestamp()).to_game(&self.timestamp_converter);
                        
                        *state = LongNoteState::Hit { press_diff: diff, rel_diff };
        
                    // todo:" cricular queue"
                } else {
                    *state = LongNoteState::Missed { held: Some(map_timestamp), press_diff: Some(diff) };
                }

                lane_state.first_object += 1;
            }
        }
    }
}

impl ObjectState {
    pub fn is_hit(&self) -> bool {
        match self {
            Self::HitObject(HitObjectState::Hit { .. })
            | Self::LongNote(LongNoteState::Hit { .. }) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::Lane;

    #[test]
    fn game_state_regular_hit() {

        let lanes = vec![Lane {
            objects: vec![
                Object::HitObject {
                    timestamp: MapTimestamp::from_millis(0),
                },
                Object::HitObject {
                    timestamp: MapTimestamp::from_millis(10_000),
                },
            ],
        }];

        let map = Map {
            song_artist: None,
            song_title: None,
            difficulty_name: None,
            mapper: None,
            audio_file: None,
            lanes: lanes,
        };

        let mut state = Game::new(map);
        state.key_press(0, GameTimestamp::from_millis(10_000));

        assert_eq!(
            &state.lane_states[0].object_states[..],
            &[
                ObjectState::HitObject(HitObjectState::NotHit),
                ObjectState::HitObject(HitObjectState::Hit {
                    diff: GameTimestampDifference::from_millis(0)
                }),
            ][..]
        );
    }
}