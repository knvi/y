use core::ops::{Add, Sub};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Timestamp(pub i32);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct TimestampDifference(pub i32);

pub struct TimestampConverter {
    /// Used to convert game-map and map-game times.
    /// 
    /// Global offset is used to adjust for the audio playback latency of the audio device
    pub global_offset: GameTimestampDifference,
}   

impl Timestamp {
    pub fn from_millis(millis: i32) -> Self {
        Self(
            millis.checked_mul(100).expect("overflow converting to timestamp")
        )
    }

    pub fn as_millis(self) -> i32 {
        self.0 / 100
    }

    pub fn from_milli_hundreds(milli_hundreds: i32) -> Self {
        Self(milli_hundreds)
    }

    pub fn into_milli_hundreds(self) -> i32 {
        self.0
    }
}

impl TimestampDifference {
    pub fn from_millis(millis: i32) -> Self {
        Self(
            millis.checked_mul(100).expect("overflow converting to timestamp")
        )
    }

    pub fn as_millis(self) -> i32 {
        self.0 / 100
    }

    pub fn from_milli_hundreds(milli_hundreds: i32) -> Self {
        Self(milli_hundreds)
    }

    pub fn into_milli_hundreds(self) -> i32 {
        self.0
    }
}

macro_rules! impl_timestamp {
    ($name:ident, $diff:ident) => {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
        pub struct $name(pub Timestamp);

        impl $name {
            pub fn from_millis(millis: i32) -> Self {
                Self(Timestamp::from_millis(millis))
            }

            pub fn as_millis(self) -> i32 {
                self.0.as_millis()
            }

            pub fn from_milli_hundreds(milli_hundreds: i32) -> Self {
                Self(Timestamp::from_milli_hundreds(milli_hundreds))
            }

            pub fn into_milli_hundreds(self) -> i32 {
                self.0.into_milli_hundreds()
            }
        }

        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
        pub struct $diff(pub TimestampDifference);

        impl $diff {
            pub fn from_millis(millis: i32) -> Self {
                Self(TimestampDifference::from_millis(millis))
            }

            pub fn as_millis(self) -> i32 {
                self.0.as_millis()
            }

            pub fn from_milli_hundreds(milli_hundreds: i32) -> Self {
                Self(TimestampDifference::from_milli_hundreds(milli_hundreds))
            }

            pub fn into_milli_hundreds(self) -> i32 {
                self.0.into_milli_hundreds()
            }
        }
    };
}

macro_rules! impl_ops {
    ($name:ty, $diff:ty) => {
        impl Sub<$name> for $name {
            type Output = $diff;

            #[inline]
            fn sub(self, other: $name) -> Self::Output {
                Self::Output {
                    0: self.0 - other.0,
                }
            }
        }

        impl Add<$diff> for $name {
            type Output = $name;

            #[inline]
            fn add(self, other: $diff) -> Self::Output {
                Self::Output {
                    0: self.0 + other.0,
                }
            }
        }

        impl Sub<$diff> for $name {
            type Output = $name;

            #[inline]
            fn sub(self, other: $diff) -> Self::Output {
                Self::Output {
                    0: self.0 - other.0,
                }
            }
        }

        impl Add<$diff> for $diff {
            type Output = $diff;

            #[inline]
            fn add(self, other: $diff) -> Self::Output {
                Self::Output {
                    0: self.0 + other.0,
                }
            }
        }

        impl Sub<$diff> for $diff {
            type Output = $diff;

            #[inline]
            fn sub(self, other: $diff) -> Self::Output {
                Self::Output {
                    0: self.0 - other.0,
                }
            }
        }
    };
}

impl_timestamp!(MapTimestamp, MapTimestampDifference);
impl_timestamp!(GameTimestamp, GameTimestampDifference);

impl_ops!(Timestamp, TimestampDifference);
impl_ops!(MapTimestamp, MapTimestampDifference);
impl_ops!(GameTimestamp, GameTimestampDifference);

impl MapTimestamp {
    pub fn to_game(self, converter: &TimestampConverter) -> GameTimestamp {
        converter.map_to_game(self)
    }
}

impl MapTimestampDifference {
    pub fn to_game(self, converter: &TimestampConverter) -> GameTimestampDifference {
        converter.map_to_game_difference(self)
    }
}

impl GameTimestamp {
    pub fn to_map(self, converter: &TimestampConverter) -> MapTimestamp {
        converter.game_to_map(self)
    }
}

impl GameTimestampDifference {
    pub fn to_map(self, converter: &TimestampConverter) -> MapTimestampDifference {
        converter.game_to_map_difference(self)
    }
}

impl TimestampConverter {
    pub fn game_to_map(&self, timestamp: GameTimestamp) -> MapTimestamp {
        MapTimestamp((timestamp + self.global_offset).0)
    }

    pub fn map_to_game(&self, timestamp: MapTimestamp) -> GameTimestamp {
        GameTimestamp(timestamp.0) - self.global_offset
    }

    pub fn game_to_map_difference(
        &self,
        difference: GameTimestampDifference,
    ) -> MapTimestampDifference {
        MapTimestampDifference(difference.0)
    }

    pub fn map_to_game_difference(
        &self,
        difference: MapTimestampDifference,
    ) -> GameTimestampDifference {
        GameTimestampDifference(difference.0)
    }
}