
use std::fmt::{Debug, Formatter};

/// Error type for when a cell is accessed out of bounds on a board.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct OutOfBoundsSetError {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}
impl Debug for OutOfBoundsSetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Out of bounds error: ({}, {}) accessed a board of size ({}, {})", self.x, self.y, self.width, self.height)
    }
}