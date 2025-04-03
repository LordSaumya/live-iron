use crate::components::{
    board::Board,
    rule::Rule,
    state::State,
};
use std::fmt::Debug;

/// A trait that defines a rule for a cellular automaton that can be evolved using genetic algorithms.
/// 
/// This trait extends `Rule` to add genetic algorithm operations,
/// allowing rules to be evolved over time using selection, crossover,
/// and mutation.
/// 
/// # Type Parameters
/// 
/// - `S`: The type of state that each cell in the board can have.
pub trait Genotype<S: State>: Rule<S> + Clone + Debug + Send + Sync {
    /// Perform crossover with another genotype to produce offspring.
    /// 
    /// Combines genetic material from self and other to create new rule according to the crossover strategy.
    /// 
    /// # Arguments
    /// - `other`: The other genotype to crossover with.
    /// 
    /// # Returns
    /// 
    /// A new genotype that is a combination of self and other.
    fn crossover(&self, other: &Self) -> Self;
    
    /// Mutate this genotype with the given mutation rate.
    /// 
    /// Changes the rule's parameters randomly with probability determined by `mutation_rate`.
    /// 
    /// # Arguments
    /// - `mutation_rate`: The probability of mutation for each parameter.
    fn mutate(&mut self, mutation_rate: f64);
    
    /// Calculate the fitness of this genotype.
    /// 
    /// Evaluates how well this rule performs against a target pattern or other criteria.
    /// 
    /// # Arguments
    /// - `board`: The board to evaluate the rule against.
    /// 
    /// # Returns
    /// 
    /// A fitness score as a floating-point number.
    fn fitness(&self, board: &Board<S>) -> f64;
}