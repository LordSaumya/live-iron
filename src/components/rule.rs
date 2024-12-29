use super::{board::Board, error::OutOfBoundsSetError, state::State};

/// A trait that defines a rule for updating the state of a cell in a cellular automaton.
///
/// The rule takes the coordinates of the cell and the board of cells as input and returns the new state of the cell.
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
    /// The new state of the cell at the given coordinates, or an error if the coordinates are out of bounds.
    fn apply(&self, coord: (usize, usize), board: &mut Board<S>) -> Result<S, OutOfBoundsSetError>;
}

/// Rules for common cellular automata.
pub mod common_rules {
    use super::Rule;
    use crate::components::board::Board;
    use crate::components::error::OutOfBoundsSetError;
    use crate::components::neighbourhood::{Neighbourhood, NeighbourhoodType};
    use crate::components::state::common_states::{
        AntDirection, CellColour, GameOfLifeState, LangtonsAntState,
    };
    pub struct GameOfLifeRule;

    impl Rule<GameOfLifeState> for GameOfLifeRule {
        fn apply(
            &self,
            coord: (usize, usize),
            board: &mut Board<GameOfLifeState>,
        ) -> Result<GameOfLifeState, OutOfBoundsSetError> {
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

            match curr_state {
                GameOfLifeState::Alive => {
                    num_alive -= 1; //subtract cell from neighbourhood
                    if num_alive < 2 {
                        Ok(GameOfLifeState::Dead)
                    } else if num_alive == 2 || num_alive == 3 {
                        Ok(GameOfLifeState::Alive)
                    } else {
                        Ok(GameOfLifeState::Dead)
                    }
                }
                GameOfLifeState::Dead => {
                    if num_alive == 3 {
                        Ok(GameOfLifeState::Alive)
                    } else {
                        Ok(GameOfLifeState::Dead)
                    }
                }
            }
        }
    }

    pub struct LangtonsAntRule;

    impl Rule<LangtonsAntState> for LangtonsAntRule {
        fn apply(
            &self,
            coord: (usize, usize),
            board: &mut Board<LangtonsAntState>,
        ) -> Result<LangtonsAntState, OutOfBoundsSetError> {
            // Get the current state of the cell.
            let old_state: LangtonsAntState = board.get(coord.0, coord.1).ok_or(OutOfBoundsSetError {
                x: coord.0,
                y: coord.1,
                width: board.width(),
                height: board.height(),
            })?;

            // Get the ant's direction. If the ant is not present, return the old state (no change).
            let Some(direction) = old_state.ant_direction else {
                return Ok(old_state);
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
            let updated_old_cell = LangtonsAntState {
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

            // Update the new cell with the new direction of the ant, and set the ant's direction.
            next_cell.ant_direction = Some(new_direction);
            board.set(nx, ny, next_cell)?;

            // Return the updated state of the old cell.
            Ok(updated_old_cell)
        }
    }
}
