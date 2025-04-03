use crate::components::{
    board::Board,
    genetic::{
        genotype::Genotype,
        selection_strategy::SelectionStrategy,
    },
    state::State,
};
use std::fmt::Debug;
use rayon::prelude::*;
use std::marker::PhantomData;

/// A struct that represents a population of genotypes in a genetic algorithm.
///
/// The population contains a vector of genotypes, a selection strategy, and a mutation rate for the population. It implements methods for creating a new population, calculating fitness scores, and adding or removing genotypes.
///
/// # Type Parameters
/// - `S`: The type of state that each cell in the board can have.
/// - `G`: The type of genotype that represents a rule for the cellular automaton.
///
/// # Fields
///
/// - `genotypes`: A vector of genotypes in the population.
/// - `selection_strategy`: The strategy to use for selection (e.g., tournament, roulette, etc.).
/// - `mutation_rate`: The rate of mutation for the population. Between 0.0 and 1.0.
#[derive(Clone, Debug)]
pub struct Population<S: State, G: Genotype<S>> {
    /// A vector of genotypes in the population.
    genotypes: Vec<G>,
    /// The strategy to use for selection (e.g., tournament, roulette, etc.).
    selection_strategy: SelectionStrategy,
    /// The rate of mutation for the population. Between 0.0 and 1.0.
    mutation_rate: f64,
    _phantom: PhantomData<S>,
}

impl<S: State, G: Genotype<S>> Population<S, G> {
    /// Create a new `Population` with the given genotypes, selection strategy, and mutation rate.
    ///
    /// # Arguments
    /// - `genotypes`: A vector of genotypes in the population.
    /// - `selection_strategy`: The strategy to use for selection (e.g., tournament, roulette, etc.).
    /// - `mutation_rate`: The rate of mutation for the population. Between 0.0 and 1.0.
    /// 
    /// # Returns
    /// 
    /// A new `Population` with the given genotypes, mutation strategy, crossover strategy, and selection strategy.
    /// 
    /// # Panics
    /// 
    /// Panics if the mutation rate is not between 0.0 and 1.0.
    pub fn new(
        genotypes: Vec<G>,
        selection_strategy: SelectionStrategy,
        mutation_rate: f64,
    ) -> Self {
        // Ensure the mutation rate is between 0.0 and 1.0
        if mutation_rate < 0.0 || mutation_rate > 1.0 {
            panic!("Mutation rate must be between 0.0 and 1.0");
        }
        Self {
            genotypes,
            selection_strategy,
            mutation_rate,
            _phantom: PhantomData,
        }
    }

    /// Get the genotypes in the population.
    /// 
    /// # Returns
    /// 
    /// A reference to the vector of genotypes in the population.
    pub fn genotypes(&self) -> &Vec<G> {
        &self.genotypes
    }

    /// Calculate the fitness scores of all genotypes in the population.
    /// 
    /// # Arguments
    /// 
    /// - `board`: A reference to the board of cells to evaluate the genotypes against.
    /// 
    /// # Returns
    /// 
    /// A vector of fitness scores for each genotype in the population.
    pub fn fitness_scores(&self, board: &Board<S>) -> Vec<f64> {
        self.genotypes
            .par_iter()
            .map(|genotype| genotype.fitness(board))
            .collect()
    }

    /// Remove a genotype from the population at the given index.
    /// 
    /// # Arguments
    /// 
    /// - `index`: The index of the genotype to remove.
    /// 
    /// # Returns
    /// 
    /// The removed genotype.
    /// 
    /// # Panics
    /// 
    /// Panics if the index is out of bounds.
    pub fn remove_genotype(&mut self, index: usize) -> Result<G, String> {
        if index >= self.genotypes.len() {
            return Err(format!("Index out of bounds: {}", index));
        }
        Ok(self.genotypes.remove(index))
    }

    pub fn add_genotype(&mut self, genotype: G) {
        self.genotypes.push(genotype)
    }

    /// Get the number of genotypes in the population.
    /// 
    /// # Returns
    /// 
    /// The number of genotypes in the population.
    pub fn len(&self) -> usize {
        self.genotypes.len()
    }

    /// Add a child genotype to the population by selecting two parents using the selection strategy and performing crossover and mutation.
    /// 
    /// # Arguments
    /// 
    /// - `board`: A reference to the board of cells to evaluate the genotypes against.
    /// 
    /// # Returns
    /// 
    /// A result indicating success or failure.
    pub fn add_child(&mut self, board: &Board<S>) -> Result<(), String> {
        if self.genotypes.is_empty() {
            return Err("Population is empty".to_string());
        }

        // Select parents using the selection strategy
        let fitness_scores: Vec<f64> = self.fitness_scores(board);
        let (parent1_index, parent2_index) = self.selection_strategy.select_parents(&fitness_scores);

        let parent1: &G = &self.genotypes[parent1_index];
        let parent2: &G = &self.genotypes[parent2_index];

        // Perform crossover and mutation to create a child genotype
        let mut child: G = parent1.crossover(parent2);
        child.mutate(self.mutation_rate);

        // Add the child to the population
        Ok(self.genotypes.push(child))
    }

    /// Kill a percentage of the population based on fitness scores using the selection strategy.
    /// 
    /// # Arguments
    /// 
    /// - `percentage`: The percentage of the population to kill relative to the current population (0.0 to 1.0).
    /// - `board`: A reference to the board of cells to evaluate the genotypes against.
    /// 
    /// # Returns
    /// 
    /// A result indicating success or failure.
    pub fn shrink_population(&mut self, percentage: f64, board: &Board<S>) -> Result<(), String> {
        if percentage < 0.0 || percentage > 1.0 {
            return Err("Percentage must be between 0.0 and 1.0".to_string());
        }

        if self.genotypes.is_empty() {
            return Err("Population is empty".to_string());
        }

        let fitness_scores: Vec<f64> = self.fitness_scores(board);
        let selected_indices: Vec<usize> = self.selection_strategy.select_deaths(&fitness_scores, percentage);
        
        selected_indices.iter().for_each(|&index| {
            self.genotypes.remove(index);
        });

        Ok(())
    }

    /// Grow the population by adding a percentage of new genotypes based on fitness scores using the selection strategy.
    /// 
    /// # Arguments
    /// 
    /// - `percentage`: The percentage of the population to grow relative to the current population (0.0 to 1.0).
    /// - `board`: A reference to the board of cells to evaluate the genotypes against.
    /// 
    /// # Returns
    /// 
    /// A result indicating success or failure.
    pub fn grow_population(&mut self, percentage: f64, board: &Board<S>) -> Result<(), String> {
        if percentage < 0.0 || percentage > 1.0 {
            return Err("Percentage must be between 0.0 and 1.0".to_string());
        }

        if self.genotypes.is_empty() {
            return Err("Population is empty".to_string());
        }

        // Calculate the number of new genotypes to add
        let num_new_genotypes = (self.genotypes.len() as f64 * percentage).round() as usize;
        for _ in 0..num_new_genotypes {
            self.add_child(board)?;
        }

        Ok(())
    }

    /// Advance the population by one generation by first shrinking it (removing less fit individuals) and then growing it (adding new offspring).
    /// 
    /// # Arguments
    /// 
    /// - `death_percentage`: The percentage of the population to remove (0.0 to 1.0).
    /// - `growth_percentage`: The percentage of the population to add (0.0 to 1.0).
    /// - `board`: A reference to the board of cells to evaluate the genotypes against.
    /// 
    /// # Returns
    /// 
    /// A result indicating success or failure.
    pub fn advance_generation(&mut self, death_percentage: f64, growth_percentage: f64, board: &Board<S>) -> Result<(), String> {
        // First remove less fit individuals
        self.shrink_population(death_percentage, board)?;
        
        // Then add new offspring
        self.grow_population(growth_percentage, board)?;
        
        Ok(())
    }
}

// Implement IntoIterator for Population to allow consuming iteration
impl<S: State, G: Genotype<S>> IntoIterator for Population<S, G> {
    type Item = G;
    type IntoIter = std::vec::IntoIter<G>;

    fn into_iter(self) -> Self::IntoIter {
        self.genotypes.into_iter()
    }
}

// Implement IntoIterator for &Population to allow immutable iteration
impl<'a, S: State, G: Genotype<S>> IntoIterator for &'a Population<S, G> {
    type Item = &'a G;
    type IntoIter = std::slice::Iter<'a, G>;

    fn into_iter(self) -> Self::IntoIter {
        self.genotypes.iter()
    }
}

// Implement IntoIterator for &mut Population to allow mutable iteration
impl<'a, S: State, G: Genotype<S>> IntoIterator for &'a mut Population<S, G> {
    type Item = &'a mut G;
    type IntoIter = std::slice::IterMut<'a, G>;

    fn into_iter(self) -> Self::IntoIter {
        self.genotypes.iter_mut()
    }
}