use crate::components::state::State;
use crate::components::board::Board;

/// The type of neighbourhood to use for a cellular automaton, which determines the cells to consider when updating a cell.
/// 
/// The neighbourhood types are:
/// - VonNeumann: The four cells directly adjacent to the cell.
/// - Moore: The eight cells directly adjacent to the cell.
pub enum NeighbourhoodType {
    VonNeumann,
    Moore,
}

/// A struct that defines the neighbourhood of a cell in a cellular automaton.
/// The struct contains the type of neighbourhood to use and the radius of the neighbourhood.
/// 
/// # Fields
/// 
/// - `neighbourhood_type`: The type of neighbourhood to use.
/// - `radius`: The radius of the neighbourhood.
pub struct Neighbourhood {
    pub neighbourhood_type: NeighbourhoodType,
    pub radius: usize,
}

impl Neighbourhood {
    /// Create a new `Neighbourhood` with the given type and radius.
    pub fn new(neighbourhood_type: NeighbourhoodType, radius: usize) -> Self {
        Self {
            neighbourhood_type,
            radius,
        }
    }

    /// Get the neighbourhood of a cell on a board.
    /// 
    /// # Arguments
    /// 
    /// - `board`: The board to get the neighbourhood from.
    /// - `x`: The x-coordinate of the cell.
    /// - `y`: The y-coordinate of the cell.
    /// 
    /// # Type Parameters
    /// 
    /// - `S`: The type of state that each cell in the board can have.
    /// 
    /// # Returns
    /// 
    /// A vector of the coordinates of the cells in the neighbourhood.
    pub fn get_neighbourhood<S: State>(&self, board: &Board<S>, x: usize, y: usize) -> Vec<Option<(usize, usize)>> {
        let mut neighbourhood: Vec<Option<(usize, usize)>> = Vec::with_capacity((2 * self.radius + 1) * (2 * self.radius + 1) - 1);
        let (width, height) = (board.width(), board.height());

        match self.neighbourhood_type {
            NeighbourhoodType::VonNeumann => {
                for i in (x as isize - self.radius as isize)..=(x as isize + self.radius as isize) {
                    for j in (y as isize - self.radius as isize)..=(y as isize + self.radius as isize) {
                        if (i == x as isize && j == y as isize) || i < 0 || j < 0 || i >= width as isize || j >= height as isize {
                            continue;
                        }

                        if (i - x as isize).abs() + (j - y as isize).abs() <= self.radius as isize {
                            let nx: usize = i.rem_euclid(width as isize) as usize;
                            let ny: usize = j.rem_euclid(height as isize) as usize;
                            match board.get(nx, ny) {
                                Some(_cell) => {
                                    neighbourhood.push(Some((nx, ny)));
                                }
                                None => {
                                    neighbourhood.push(None);
                                }
                            }
                        }
                    }
                }
            }
            NeighbourhoodType::Moore => {
                for i in (x as isize - self.radius as isize)..=(x as isize + self.radius as isize) {
                    for j in (y as isize - self.radius as isize)..=(y as isize + self.radius as isize) {
                        if i < 0 || j < 0 || i >= width as isize || j >= height as isize {
                            continue;
                        }

                        let nx: usize = i.rem_euclid(width as isize) as usize;
                        let ny: usize = j.rem_euclid(height as isize) as usize;

                        match board.get(nx, ny) {
                            Some(_cell) => {
                                neighbourhood.push(Some((nx, ny)));
                            }
                            None => {
                                neighbourhood.push(None);
                            }
                        }
                    }
                }
            }
        }
        neighbourhood
    }

    /// Get the states of the cells in the neighbourhood of a cell on a board.
    /// 
    /// # Arguments
    /// 
    /// - `board`: The board to get the neighbourhood states from.
    /// - `x`: The x-coordinate of the cell.
    /// - `y`: The y-coordinate of the cell.
    /// 
    /// # Type Parameters
    /// 
    /// - `S`: The type of state that each cell in the board can have.
    /// 
    /// # Returns
    /// 
    /// A vector of the states of the cells in the neighbourhood.
    /// If a cell is out of bounds, the state will be `None`.
    /// The order of the states is the same as the order of the cells in the neighbourhood.
    pub fn get_neighbourhood_states<S: State>(&self, board: &Board<S>, x: usize, y: usize) -> Vec<Option<S>> {
        let neighbours: Vec<Option<(usize, usize)>> = self.get_neighbourhood(board, x, y);
        let mut neighbourhood_states: Vec<Option<S>> = Vec::with_capacity(neighbours.len());
        
        neighbours.iter().map(|neighbour| neighbourhood_states.push(board.get(neighbour.unwrap().0, neighbour.unwrap().1))).for_each(drop);
        neighbourhood_states
    }
}