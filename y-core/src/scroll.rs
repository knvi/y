use std::ops::{Div, Mul};

use crate::timing::GameTimestampDifference;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Position(pub i64);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct ScrollSpeed(pub u8);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct ScrollSpeedMultiplier(i32);

impl ScrollSpeedMultiplier {
    pub fn new(value: i32) -> Self {
        assert!(value < 2i32.pow(24));
        assert!(value >= -(2i32.pow(24)));

        Self(value)
    }

    pub fn from_f32(value: f32) -> Self {
        Self::new((value * 1000.) as i32)
    }

    pub fn as_f32(self) -> f32 {
        (self.0 as f32) / 1000.
    }
}

impl Default for ScrollSpeedMultiplier {
    fn default() -> Self {
        Self(1000)
    }
}

impl Mul<GameTimestampDifference> for ScrollSpeed {
    type Output = Position;

    fn mul(self, rhs: GameTimestampDifference) -> Self::Output {
        Position(
            i64::from(self.0)
                * i64::from(rhs.into_milli_hundreds())
                * i64::from(ScrollSpeedMultiplier::default().0),
        )
    }
}

impl Mul<ScrollSpeed> for GameTimestampDifference {
    type Output = Position;

    fn mul(self, rhs: ScrollSpeed) -> Self::Output {
        rhs * self
    }
}

impl Div<ScrollSpeed> for Position {
    type Output = GameTimestampDifference;

    fn div(self, rhs: ScrollSpeed) -> Self::Output {
        let value = self.0 / i64::from(rhs.0) / i64::from(ScrollSpeedMultiplier::default().0);
        GameTimestampDifference::from_milli_hundreds(value.try_into().unwrap())
    }
}