use super::components::error::OutOfBoundsSetError;
use super::components::{
    board::{Board, Colour},
    rule::{Delta, Rule},
    state::State,
};
use super::ui::simulate;
use rayon::prelude::*;

/// A struct that represents a cellular automaton.
///
/// The automaton contains a board of cells, a set of rules, and the current time step.
///
/// # Type Parameters
///
/// - `S`: The type of state that each cell in the board can have.
///
/// # Fields
///
/// - `board`: A reference to the board of cells.
/// - `rules`: A vector of rules to apply to the board. The rules are applied in the order they are stored in the vector.
/// - `curr_time`: The current time step of the automaton.
///
/// # Lifetime
///
/// - `'a`: The lifetime of the board.
pub struct Automaton<'a, S: State> {
    board: &'a mut Board<S>,
    rules: Vec<Box<dyn Rule<S>>>,
    curr_time: usize,
}

impl<'a, S: State> Automaton<'a, S> {
    /// Create a new `Automaton` with the given board, and rules.
    ///
    /// # Arguments
    ///
    /// - `board`: A reference to the board of cells.
    ///
    /// - `rules`: A vector of rules to apply to the board.
    ///
    /// # Returns
    ///
    /// A new `Automaton` with the given board, and rules.
    pub fn new(board: &'a mut Board<S>, rules: Vec<Box<dyn Rule<S>>>) -> Self {
        Self {
            board,
            rules,
            curr_time: 0,
        }
    }

    /// Get the current time step of the automaton.
    ///
    /// # Returns
    ///
    /// The current time step of the automaton.
    pub fn curr_time(&self) -> usize {
        self.curr_time
    }

    /// Get the board of the automaton.
    ///
    /// # Returns
    ///
    /// A reference to the board of the automaton.
    pub fn board(&self) -> &Board<S> {
        self.board
    }

    /// Get the rules of the automaton.
    ///
    /// # Returns
    ///
    /// A reference to the vector of rules of the automaton.
    pub fn rules(&self) -> &Vec<Box<dyn Rule<S>>> {
        &self.rules
    }

    /// Add a rule to the automaton.
    ///
    /// # Arguments
    ///
    /// - `rule`: The rule to add to the automaton.
    pub fn add_rule(&mut self, rule: Box<dyn Rule<S>>) {
        self.rules.push(rule);
    }

    /// Apply the rules of the automaton to the board.
    ///
    /// # Returns
    ///
    /// A `Result` containing an error if the rules could not be applied.
    fn apply_rules(&mut self) -> Result<(), OutOfBoundsSetError> {
        if self.rules.is_empty() {
            return Ok(());
        }

        let estimated_deltas: usize = self.board.width() * self.board.height() * self.rules.len();
        let mut deltas: Vec<Delta<S>> = Vec::with_capacity(estimated_deltas);

        let coords: Vec<(usize, usize)> = self.board.iter_coords().collect::<Vec<(usize, usize)>>();
        for rule in self.rules.iter() {
            let rule_deltas: Vec<Delta<S>> = coords
                .par_iter()
                .filter_map(|coord| {
                    match rule.delta(coord.clone(), self.board) {
                        Ok(deltas) => Some(deltas),
                        Err(_) => None,
                    }
                })
                .flatten()
                .collect();
            deltas.extend(rule_deltas);
        }

        deltas.iter().for_each(|delta| {
            let _ = delta.apply(self.board);
        });

        Ok(())
    }

    /// Advance the automaton by one time step.
    ///
    /// The automaton applies the rules to the board and increments the time step.
    ///
    /// # Returns
    ///
    /// A `Result` containing an error if the automaton could not be advanced.
    fn advance(&mut self) -> Result<(), OutOfBoundsSetError> {
        self.apply_rules()?;
        self.curr_time += 1;
        Ok(())
    }

    /// Advance the automaton by the given number of time steps.
    ///
    /// The automaton applies the rules to the board and increments the time step by the given number.
    /// Rules are applied in the order they are stored.
    ///
    /// # Arguments
    ///
    /// - `steps`: The number of time steps to advance the automaton.
    ///
    /// # Returns
    ///
    /// A `Result` containing an error if the automaton could not be advanced.
    pub fn evolve(&mut self, steps: usize) -> Result<(), OutOfBoundsSetError> {
        for _ in 0..steps {
            self.advance()?;
        }
        Ok(())
    }

    /// Advance the automaton by the given number of time steps and print the board at each time step.
    ///
    /// The automaton applies the rules to the board and increments the time step by the given number.
    /// Rules are applied in the order they are stored. The board is printed at each time step.
    ///
    /// # Arguments
    ///
    /// - `steps`: The number of time steps to advance the automaton.
    ///
    /// - `interval`: The number of milliseconds between each time step.
    ///
    /// # Returns
    ///
    /// A `Result` containing an error if the automaton could not be advanced.
    pub fn evolve_with_print(
        &mut self,
        steps: usize,
        interval: u64,
    ) -> Result<(), OutOfBoundsSetError> {
        for _ in 0..steps {
            self.advance()?;
            std::thread::sleep(std::time::Duration::from_millis(interval));
            println!("{}", self.board);
        }
        Ok(())
    }

    /// Visualise the automaton by running the simulation for the given number of steps and interval.
    /// 
    /// The automaton applies the rules to the board and increments the time step by the given number.
    /// Rules are applied in the order they are stored. The board is visualised at each time step.
    /// 
    /// # Arguments
    /// 
    /// - `steps`: The number of time steps to advance the automaton.
    /// 
    /// - `interval`: The number of milliseconds between each time step.
    /// 
    /// # Returns
    /// 
    /// A `Result` containing an error if the automaton could not be advanced.
    /// 
    /// # Type Parameters
    /// 
    /// - `S`: The type of state that each cell in the board can have. It must implement `Into<Colour>`.
    pub fn visualise(&mut self, steps: usize, interval: u64) -> Result<(), OutOfBoundsSetError>
    where
        S: Into<Colour>,
    {
        simulate(self, steps, interval);
        Ok(())
    }
}
