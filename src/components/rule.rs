use super::{board::Board, error::OutOfBoundsSetError, state::State};

/// A trait that defines a rule for updating the state of a cell in a cellular automaton.
///
/// # Type Parameters
///
/// - `S`: The type of state that each cell in the board can have.
pub trait Rule<S: State>: Send + Sync {
    /// Apply the rule to the cell at the given coordinates on the board.
    ///
    /// # Arguments
    ///
    /// - `coord`: A tuple containing the x and y coordinates of the cell.
    ///
    /// - `board`: A reference to the board of cells.
    ///
    /// # Returns
    ///
    /// A vector of deltas to the board, or an error if the coordinates are out of bounds.
    fn delta(&mut self, coord: (usize, usize), board: &Board<S>) -> Result<Vec<Delta<S>>, OutOfBoundsSetError>;
}

/// A struct that represents a change to the state of a cell in a cellular automaton.
/// 
/// The struct contains the x and y coordinates of the cell and the new state of the cell.
/// 
/// # Type Parameters
/// 
/// - `S`: The type of state that each cell in the board can have.
/// 
/// # Fields
/// 
/// - `x`: The x-coordinate of the cell.
/// - `y`: The y-coordinate of the cell.
/// - `state`: The new state of the cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Delta<S: State> {
    pub x: usize,
    pub y: usize,
    pub state: S,
}

impl<S: State> Delta<S> {
    /// Create a new `Delta` with the given x and y coordinates and state.
    /// 
    /// # Arguments
    /// 
    /// - `x`: The x-coordinate of the cell.
    /// 
    /// - `y`: The y-coordinate of the cell.
    /// 
    /// - `state`: The new state of the cell.
    /// 
    /// # Returns
    /// 
    /// A new `Delta` with the given x and y coordinates and state.
    pub fn new(x: usize, y: usize, state: S) -> Self {
        Self { x, y, state }
    }

    /// Apply the delta to the board.
    pub fn apply(&self, board: &mut Board<S>) -> Result<(), OutOfBoundsSetError> {
        board.set(self.x, self.y, self.state)
    }
}

/// Rules for common cellular automata.
pub mod common_rules {
    use std::vec;

    use super::{Rule, Delta};
    use crate::components::board::Board;
    use crate::components::error::OutOfBoundsSetError;
    use crate::components::neighbourhood::{Neighbourhood, NeighbourhoodType};
    use crate::components::state::common_states::{
        AntDirection, CellColour, GameOfLifeState, LangtonsAntState,
    };
    pub struct GameOfLifeRule;

    impl Rule<GameOfLifeState> for GameOfLifeRule {
        fn delta (
            &mut self,
            coord: (usize, usize),
            board: &Board<GameOfLifeState>,
        ) -> Result<Vec<Delta<GameOfLifeState>>, OutOfBoundsSetError> {
            let mut num_alive: u16 = 0;
            let neighbourhood: Neighbourhood = Neighbourhood::new(NeighbourhoodType::Moore, 1);

            let curr_state: GameOfLifeState = board
                .get(coord.0, coord.1)
                .expect("The rule should not be applied on cells outside the board");
            let neighbours: Vec<Option<GameOfLifeState>> =
                neighbourhood.get_neighbourhood_states(board, coord.0, coord.1);

            neighbours.iter().for_each(|x| match x {
                Some(GameOfLifeState::Alive) => num_alive += 1,
                _ => {}
            });

            let new_state: GameOfLifeState = match curr_state {
                GameOfLifeState::Alive => {
                    num_alive -= 1; //subtract cell from neighbourhood
                    if num_alive < 2 {
                        GameOfLifeState::Dead
                    } else if num_alive == 2 || num_alive == 3 {
                        GameOfLifeState::Alive
                    } else {
                        GameOfLifeState::Dead
                    }
                }
                GameOfLifeState::Dead => {
                    if num_alive == 3 {
                        GameOfLifeState::Alive
                    } else {
                        GameOfLifeState::Dead
                    }
                }
            };

            Ok(vec![Delta::new(coord.0, coord.1, new_state)])
        }
    }

    pub struct LangtonsAntRule;

    impl Rule<LangtonsAntState> for LangtonsAntRule {
        fn delta(
            &mut self,
            coord: (usize, usize),
            board: &Board<LangtonsAntState>,
        ) -> Result<Vec<Delta<LangtonsAntState>>, OutOfBoundsSetError> {
            // Get the current state of the cell.
            let old_state: LangtonsAntState = board.get(coord.0, coord.1).ok_or(OutOfBoundsSetError {
                x: coord.0,
                y: coord.1,
                width: board.width(),
                height: board.height(),
            })?;

            // Get the ant's direction. If the ant is not present, return the old state (no change).
            let Some(direction) = old_state.ant_direction else {
                return Ok(vec![Delta::new(coord.0, coord.1, old_state)]);
            };
            
            // Update the cell's state based on the ant's direction and the cell's colour.
            let new_direction: AntDirection = match old_state.colour {
                CellColour::White => match direction {
                    AntDirection::Up => AntDirection::Right,
                    AntDirection::Right => AntDirection::Down,
                    AntDirection::Down => AntDirection::Left,
                    AntDirection::Left => AntDirection::Up,
                },
                CellColour::Black => match direction {
                    AntDirection::Up => AntDirection::Left,
                    AntDirection::Left => AntDirection::Down,
                    AntDirection::Down => AntDirection::Right,
                    AntDirection::Right => AntDirection::Up,
                },
            };

            // Flip the colour of the old cell and remove the ant.
            let flipped_colour: CellColour = match old_state.colour {
                CellColour::White => CellColour::Black,
                CellColour::Black => CellColour::White,
            };

            let updated_old_cell: LangtonsAntState = LangtonsAntState {
                colour: flipped_colour,
                ant_direction: None,
            };

            let (nx, ny) = match new_direction {
                AntDirection::Up => (coord.0, coord.1.wrapping_sub(1)),
                AntDirection::Right => (coord.0 + 1, coord.1),
                AntDirection::Down => (coord.0, coord.1 + 1),
                AntDirection::Left => (coord.0.wrapping_sub(1), coord.1),
            };

            let mut next_cell: LangtonsAntState = board.get(nx, ny).ok_or(OutOfBoundsSetError {
                x: nx,
                y: ny,
                width: board.width(),
                height: board.height(),
            })?;

            next_cell = LangtonsAntState {
                colour: next_cell.colour,
                ant_direction: Some(new_direction),
            };

            // Return the deltas to the old and new cells
            Ok(vec![Delta::new(coord.0, coord.1, updated_old_cell), Delta::new(nx, ny, next_cell)])
        }
    }
}
