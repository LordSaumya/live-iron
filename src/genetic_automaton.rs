use super::components::error::OutOfBoundsSetError;
use super::components::{
    board::{Board, Colour},
    rule::Delta,
    genetic::{
        genotype::Genotype,
        population::Population,
    },
    state::State,
};
use super::ui::simulate_genetic;
use rayon::prelude::*;

/// A struct that represents a genetic cellular automaton.
///
/// The automaton contains a board of cells, a population of genotypes (genetic rules), and the current time step.
///
/// # Type Parameters
///
/// - `S`: The type of state that each cell in the board can have.
/// - `G`: The type of genotype that represents a rule for the cellular automaton.
///
/// # Fields
///
/// - `board`: A reference to the board of cells.
/// - `population`: A vector of genotypes (genetic rules) to apply to the board.
/// - `curr_time`: The current time step of the automaton.
///
/// # Lifetime
///
/// - `'a`: The lifetime of the board.
pub struct GeneticAutomaton<'a, S: State, G: Genotype<S>> {
    /// A reference to the board of cells.
    board: &'a mut Board<S>,
    /// A vector of genotypes (genetic rules) to apply to the board.
    population: Population<S, G>,
    /// The current time step of the automaton.
    curr_time: usize,
}

impl<'a, S: State, G: Genotype<S>> GeneticAutomaton<'a, S, G> {
    /// Create a new `GeneticAutomaton` with the given board and population.
    ///
    /// # Arguments
    ///
    /// - `board`: A reference to the board of cells.
    ///
    /// - `population`: A vector of genotypes (genetic rules) to apply to the board.
    ///
    /// # Returns
    ///
    /// A new `GeneticAutomaton` with the given board, rules, and neighbourhood.
    pub fn new(board: &'a mut Board<S>, population: Population<S, G>) -> Self {
        Self {
            board,
            population,
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

    /// Get the population of the automaton mutably.
    /// 
    /// # Returns
    /// 
    /// A mutable reference to the population of the automaton.
    pub fn population(&mut self) -> &mut Population<S, G> {
        &mut self.population
    }

    /// Apply the rules of the automaton to the board.
    /// 
    /// # Returns
    /// 
    /// A `Result` containing an error if the rules could not be applied.
    fn apply_rules(&mut self) -> Result<(), OutOfBoundsSetError> {
        if self.population.len() == 0 {
            return Ok(());
        }

        let estimated_deltas: usize = self.board.width() * self.board.height() * self.population.len();
        let mut deltas: Vec<Delta<S>> = Vec::with_capacity(estimated_deltas);

        let coords: Vec<(usize, usize)> = self.board.iter_coords().collect::<Vec<(usize, usize)>>();
        for rule in &self.population {
            let rule_deltas: Vec<Delta<S>> = coords
                .par_iter()
                .filter_map(|coord| {
                    match rule.delta(coord.clone(), &self.board) {
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
    /// # Arguments
    /// 
    /// - `growth_rate`: The growth rate of the population.
    /// - `death_rate`: The death rate of the population.
    /// 
    /// # Returns
    /// 
    /// A `Result` containing an error if the automaton could not be advanced.
    fn advance(
        &mut self,
        growth_rate: f64,
        death_rate: f64,
    ) -> Result<(), OutOfBoundsSetError> {
        self.apply_rules()?;
        let _ = self.population.advance_generation(death_rate, growth_rate, self.board);
        self.curr_time += 1;
        Ok(())
    }

    /// Advance the automaton by the given number of generations using tbe 
    /// 
    /// # Arguments
    /// 
    /// - `generations`: The number of generations to run the automaton for.
    /// - `growth_rate`: The growth rate of the population.
    /// - `death_rate`: The death rate of the population.
    /// 
    /// # Returns
    /// 
    /// A `Result` containing an error if the automaton could not be advanced.
    pub fn evolve(
        &mut self,
        generations: usize,
        growth_rate: f64,
        death_rate: f64,
    ) -> Result<(), OutOfBoundsSetError> {
        for _ in 0..generations {
            self.advance(growth_rate, death_rate)?;
        }
        Ok(())
    }

    /// Advance the automaton by the given number of generations and print the board at each generation using the given interval.
    /// 
    /// # Arguments
    /// 
    /// - `generations`: The number of generations to run the automaton for.
    /// - `growth_rate`: The growth rate of the population.
    /// - `death_rate`: The death rate of the population.
    /// - `interval`: The number of milliseconds between each generation.
    /// 
    /// # Returns
    /// 
    /// A `Result` containing an error if the automaton could not be advanced.
    pub fn evolve_with_print(
        &mut self,
        generations: usize,
        growth_rate: f64,
        death_rate: f64,
        interval: u64,
    ) -> Result<(), OutOfBoundsSetError> {
        for _ in 0..generations {
            self.advance(growth_rate, death_rate)?;
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
    /// - `growth_rate`: The growth rate of the population.
    /// 
    /// - `death_rate`: The death rate of the population.
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
    pub fn visualise(&mut self, 
        steps: usize,
        growth_rate: f64,
        death_rate: f64,
        interval: u64,
    ) -> Result<(), OutOfBoundsSetError>
    where
        S: Into<Colour>,
    {
        simulate_genetic(self, growth_rate, death_rate, steps, interval);
        Ok(())
    }
}
