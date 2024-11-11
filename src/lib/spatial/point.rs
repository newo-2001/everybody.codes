use derive_more::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Add, Sub)]
pub struct Point<T = u32> {
    pub x: T,
    pub y: T
}