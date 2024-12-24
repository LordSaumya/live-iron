use std::fmt::Debug;
use std::vec;
use crate::components::state::State;
use crate::components::error::OutOfBoundsSetError;


/// A struct that represents a board of cells in a cellular automaton.
/// 
/// The board contains a vector of cells and the dimensions of the board.
/// The cells are stored in row-major order.
/// 
/// # Type Parameters
/// 
/// - `S`: The type of state that each cell in the board can have.
/// 
/// # Fields
/// 
/// - `cells`: A vector of the cells in the board.
/// - `dim`: A tuple containing the width and height of the board.
pub struct Board<S: State> {
    cells: Vec<S>,
    dim: (usize, usize),
}

impl<S: State> Board<S> {
    /// Create a new `Board` with the given width, height, and initial state.
    /// 
    /// # Arguments
    /// - `initial_state`: The initial state of the cells in the board as a 2D vector.
    pub fn new(initial_state: Vec<Vec<S>>) -> Self {
        Self {
            dim: (initial_state[0].len(), initial_state.len()),
            cells: initial_state.into_iter().flatten().collect(),
        }
    }

    /// Get the width of the board.
    pub fn width(&self) -> usize {
        self.dim.0
    }

    /// Get the height of the board.
    pub fn height(&self) -> usize {
        self.dim.1
    }

    /// Get the state of a cell on the board.
    /// 
    /// # Arguments
    /// 
    /// - `x`: The x-coordinate of the cell.
    /// 
    /// - `y`: The y-coordinate of the cell.
    /// 
    /// # Returns
    /// 
    /// The state of the cell at the given coordinates, or None if the coordinates are out of bounds.
    #[inline(always)]
    pub fn get(&self, x: usize, y: usize) -> Option<S> {
        if x < self.dim.0 && y < self.dim.1 {
            Some(self.cells[y * self.dim.0 + x])
        } else {
            None
        }
    }

    /// Set the state of a cell on the board.
    /// 
    /// # Arguments
    /// 
    /// - `x`: The x-coordinate of the cell.
    /// - `y`: The y-coordinate of the cell.
    /// - `state`: The new state of the cell.
    /// 
    /// # Returns
    /// 
    /// An error if the coordinates are out of bounds.
    #[inline(always)]
    pub fn set(&mut self, x: usize, y: usize, state: S) -> Result<(), OutOfBoundsSetError> {
        if x < self.dim.0 && y < self.dim.1 {
            self.cells[y * self.dim.0 + x] = state;
        } else {
            return Err(OutOfBoundsSetError { x, y, width: self.dim.0, height: self.dim.1 });
        }
        Ok(())
    }
}