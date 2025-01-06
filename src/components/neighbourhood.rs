use super::board::{Board, BoundaryCondition};
use super::state::State;
use std::collections::HashMap;

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
/// 
/// # Warning
/// 
/// Sharing a neighbourhood instance between multiple boards can lead to unexpected behaviour due to caching.
pub struct Neighbourhood {
    /// The type of neighbourhood to use.
    pub neighbourhood_type: NeighbourhoodType,
    /// The radius of the neighbourhood.
    pub radius: usize,
    /// Cache of the dimensions and boundary type of the board
    board_cache: ((usize, usize), String),
    /// Cache of the neighbourhoods of each cell
    neighbour_cache: HashMap<(usize, usize), Vec<Option<(usize, usize)>>>,
}

impl Neighbourhood {
    /// Create a new `Neighbourhood` with the given type and radius.
    pub fn new(neighbourhood_type: NeighbourhoodType, radius: usize) -> Self {
        Self {
            neighbourhood_type,
            radius,
            board_cache: ((0, 0), String::new()),
            neighbour_cache: HashMap::new(),
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
    pub fn get_neighbourhood_coords<S: State>(
        &mut self,
        board: &Board<S>,
        x: usize,
        y: usize,
    ) -> Vec<Option<(usize, usize)>> {
        let boundary_condition: BoundaryCondition<S> = board.boundary_condition();
        let mut neighbourhood: Vec<Option<(usize, usize)>> =
            Vec::with_capacity((2 * self.radius + 1) * (2 * self.radius + 1) - 1);
        let (width, height) = (board.width(), board.height());

        // Clear the cache if the board dimensions have changed
        if self.board_cache != ((width, height), boundary_condition.to_string()) {
            self.neighbour_cache.clear();
            self.board_cache = ((width, height), boundary_condition.to_string());
        }

        // Check if the neighbourhood is in the cache. If it is, return the cached neighbourhood.
        if let Some(neighbours) = self.neighbour_cache.get(&(x, y)) {
            return neighbours.clone();
        }

        match self.neighbourhood_type {
            NeighbourhoodType::VonNeumann => {
                for i in (x as isize - self.radius as isize)..=(x as isize + self.radius as isize) {
                    for j in
                        (y as isize - self.radius as isize)..=(y as isize + self.radius as isize)
                    {
                        if (i - x as isize).abs() + (j - y as isize).abs() > self.radius as isize {
                            continue;
                        }
                        match boundary_condition {
                            BoundaryCondition::Periodic => {
                                let nx = i.rem_euclid(width as isize) as usize;
                                let ny = j.rem_euclid(height as isize) as usize;
                                neighbourhood.push(Some((nx, ny)));
                            }
                            BoundaryCondition::Fixed(_) => {
                                if i < 0 || j < 0 || i >= width as isize || j >= height as isize {
                                    neighbourhood.push(None);
                                } else {
                                    neighbourhood.push(Some((i as usize, j as usize)));
                                }
                            }
                        }
                    }
                }
            }
            NeighbourhoodType::Moore => {
                for i in (x as isize - self.radius as isize)..=(x as isize + self.radius as isize) {
                    for j in
                        (y as isize - self.radius as isize)..=(y as isize + self.radius as isize)
                    {
                        match boundary_condition {
                            BoundaryCondition::Periodic => {
                                let nx = i.rem_euclid(width as isize) as usize;
                                let ny = j.rem_euclid(height as isize) as usize;
                                neighbourhood.push(Some((nx, ny)));
                            }
                            BoundaryCondition::Fixed(_) => {
                                if i < 0 || j < 0 || i >= width as isize || j >= height as isize {
                                    neighbourhood.push(None);
                                } else {
                                    neighbourhood.push(Some((i as usize, j as usize)));
                                }
                            }
                        }
                    }
                }
            }
        }

        self.neighbour_cache.insert((x, y), neighbourhood.clone());
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
    pub fn get_neighbourhood_states<S: State>(
        &mut self,
        board: &Board<S>,
        x: usize,
        y: usize,
    ) -> Vec<Option<S>> {
        let neighbours: Vec<Option<(usize, usize)>> = self.get_neighbourhood_coords(board, x, y);
        let mut neighbourhood_states: Vec<Option<S>> = Vec::with_capacity(neighbours.len());

        neighbours.iter().for_each(|n| match n {
            Some((nx, ny)) => {
                neighbourhood_states.push(board.get(nx.to_owned(), ny.to_owned()));
            }
            None => match board.boundary_condition() {
                BoundaryCondition::Fixed(val) => neighbourhood_states.push(Some(val)),
                _ => neighbourhood_states.push(None),
            },
        });
        neighbourhood_states
    }

    /// Get the states and relative coordinates of the cells in the neighbourhood of a cell on a board.
    /// 
    /// # Arguments
    /// 
    /// - `board`: The board to get the neighbourhood states and coordinates from.
    /// - `x`: The x-coordinate of the cell.
    /// - `y`: The y-coordinate of the cell.
    /// 
    /// # Type Parameters
    /// 
    /// - `S`: The type of state that each cell in the board can have.
    /// 
    /// # Returns
    /// 
    /// A vector of tuples containing the states and relative coordinates of the cells in the neighbourhood.
    pub fn get_neighbourhood_states_coords<S: State>(
        &mut self,
        board: &Board<S>,
        x: usize,
        y: usize,
    ) -> Vec<(Option<S>, (isize, isize))> {
        let neighbours: Vec<Option<(usize, usize)>> = self.get_neighbourhood_coords(board, x, y);
        let mut neighbourhood_states_and_coords: Vec<(Option<S>, (isize, isize))> =
            Vec::with_capacity(neighbours.len());

        for (_i, n) in neighbours.iter().enumerate() {
            match n {
                Some((nx, ny)) => {
                    let dx = *nx as isize - x as isize;
                    let dy = *ny as isize - y as isize;
                    neighbourhood_states_and_coords.push((board.get(nx.to_owned(), ny.to_owned()), (dx, dy)));
                }
                None => match board.boundary_condition() {
                    BoundaryCondition::Fixed(val) => neighbourhood_states_and_coords.push((Some(val), (0, 0))),
                    _ => neighbourhood_states_and_coords.push((None, (0, 0))),
                },
            }
        }
        neighbourhood_states_and_coords
    }
}
