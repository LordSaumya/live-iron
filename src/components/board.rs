use super::error::OutOfBoundsSetError;
use super::state::State;
use std::fmt::Debug;

/// The type of boundary condition to use for the board, which determines how to handle cells at the edges of the board.
///
/// The boundary conditions are:
/// - Periodic: The board wraps around at the edges.
/// - Fixed: The cells at the edges are fixed with a given state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BoundaryCondition<S: State> {
    Periodic,
    Fixed(S),
}

impl<S: State> std::fmt::Display for BoundaryCondition<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoundaryCondition::Periodic => write!(f, "Periodic"),
            BoundaryCondition::Fixed(s) => write!(f, "Fixed({:?})", s),
        }
    }
}

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
/// - `boundary_condition`: The boundary condition of the board.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Board<S: State> {
    cells: Vec<S>,
    dim: (usize, usize),
    boundary_condition: BoundaryCondition<S>,
}

impl<S: State> Board<S> {
    /// Create a new `Board` with the given width, height, and initial state.
    ///
    /// # Arguments
    /// - `initial_state`: The initial state of the cells in the board as a 2D vector.
    pub fn new(initial_state: Vec<Vec<S>>, boundary_condition: BoundaryCondition<S>) -> Self {
        Self {
            dim: (initial_state[0].len(), initial_state.len()),
            cells: initial_state.into_iter().flatten().collect(),
            boundary_condition,
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

    /// Get the boundary condition of the board.
    pub fn boundary_condition(&self) -> BoundaryCondition<S> {
        self.boundary_condition.clone()
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

    /// Set the state of a cell on the board. Wraps around the edges if the boundary condition is periodic.
    ///
    /// # Arguments
    ///
    /// - `x`: The x-coordinate of the cell.
    /// - `y`: The y-coordinate of the cell.
    /// - `state`: The new state of the cell.
    ///
    /// # Returns
    ///
    /// An error if the coordinates are out of bounds for a fixed boundary condition.
    #[inline(always)]
    pub fn set(&mut self, x: usize, y: usize, state: S) -> Result<(), OutOfBoundsSetError> {
        match self.boundary_condition {
            BoundaryCondition::Periodic => {
                let x: usize = x % self.dim.0;
                let y: usize = y % self.dim.1;
                self.cells[y * self.dim.0 + x] = state;
            }
            BoundaryCondition::Fixed(_fixed_state) => {
                if x < self.dim.0 && y < self.dim.1 {
                    self.cells[y * self.dim.0 + x] = state;
                } else {
                    return Err(OutOfBoundsSetError {
                        x,
                        y,
                        width: self.dim.0,
                        height: self.dim.1,
                    });
                }
            }
        }
        Ok(())
    }

    /// Get an iterator over the coordinates of the board.
    ///
    /// # Returns
    ///
    /// An iterator over the cell coordinates of the board in row-major order.
    ///
    /// The iterator yields tuples of the form `(x, y)`.
    pub fn iter_coords(&self) -> IterCoords {
        IterCoords {
            x: 0,
            y: 0,
            width: self.dim.0,
            height: self.dim.1,
        }
    }

    /// Get a representation of the board as a 2D vector of colours.
    ///
    /// The colours are determined by the `State` trait implementation for the cell states.
    /// The state must implement the `Into<Colour>` trait to convert the state to a colour.
    ///
    /// # Returns
    ///
    /// A 2D vector of colours representing the board.
    pub fn to_representation(&self) -> BoardRepresentation
    where
        S: Into<Colour>,
    {
        let mut representation: BoardRepresentation = Vec::new();
        for y in 0..self.dim.1 {
            let mut row: Vec<Colour> = Vec::new();
            for x in 0..self.dim.0 {
                row.push(self.get(x, y).unwrap().into());
            }
            representation.push(row);
        }
        representation
    }
}

impl<S: State> std::fmt::Display for Board<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Collect all cells into a 2D array of strings
        let mut lines: Vec<Vec<String>> = Vec::new();
        for y in 0..self.dim.1 {
            let mut row: Vec<String> = Vec::new();
            for x in 0..self.dim.0 {
                row.push(format!("{:?}", self.get(x, y).unwrap()));
            }
            lines.push(row);
        }

        // Find the maximum width
        let max_width = lines
            .iter()
            .flat_map(|row| row.iter())
            .map(|s| s.len())
            .max()
            .unwrap_or(1);

        // Print rows with borders
        for row in &lines {
            // Top border
            write!(f, "+")?;
            for _ in row {
                write!(f, "{:-<width$}+", "", width = max_width + 2)?;
            }
            writeln!(f)?;

            // Cell contents
            write!(f, "|")?;
            for cell in row {
                write!(f, " {:>width$} |", cell, width = max_width)?;
            }
            writeln!(f)?;
        }

        // Bottom border
        if let Some(row) = lines.first() {
            write!(f, "+")?;
            for _ in row {
                write!(f, "{:-<width$}+", "", width = max_width + 2)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

/// A type alias for a 2D vector of colours representing a board, used for rendering.
pub type BoardRepresentation = Vec<Vec<Colour>>;

/// A struct representing an RGB colour, used for rendering.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Colour {
    /// The red component of the colour.
    pub r: u8,
    /// The green component of the colour.
    pub g: u8,
    /// The blue component of the colour.
    pub b: u8,
}

impl Colour {
    /// Create a new `Colour` with the given red, green, and blue components.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Create a new `Colour` representing white.
    pub fn white() -> Self {
        Self {r: 255, g: 255, b: 255 }
    }

    /// Create a new `Colour` representing black.
    pub fn black() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    /// Create a new `Colour` representing red.
    pub fn red() -> Self {
        Self { r: 255, g: 0, b: 0 }
    }

    /// Create a new `Colour` representing green.
    pub fn green() -> Self {
        Self { r: 0, g: 255, b: 0 }
    }

    /// Create a new `Colour` representing blue.
    pub fn blue() -> Self {
        Self { r: 0, g: 0, b: 255 }
    }
}

impl From<Colour> for String {
    fn from(colour: Colour) -> String {
        format!("rgb({}, {}, {})", colour.r, colour.g, colour.b)
    }
}

/// An iterator over the coordinates of a board.
///
/// The iterator yields tuples of the form `(x, y)`.
///
/// # Fields
///
/// - `x`: The current x-coordinate.
/// - `y`: The current y-coordinate.
/// - `width`: The width of the board.
/// - `height`: The height of the board.
pub struct IterCoords {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Iterator for IterCoords {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.height {
            let coord: (usize, usize) = (self.x, self.y);
            self.x += 1;
            if self.x == self.width {
                self.x = 0;
                self.y += 1;
            }
            Some(coord)
        } else {
            None
        }
    }
}
