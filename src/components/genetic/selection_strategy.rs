use rand::{Rng, thread_rng};
use std::fmt::Debug;

/// Methods for selecting parents from a population
#[derive(Clone, Debug)]
pub enum SelectionStrategy {
    /// Tournament selection with the given tournament size
    Tournament(usize),
    /// Fitness proportionate (roulette wheel) selection
    RouletteWheel,
    /// Rank-based selection
    Rank(f64),
    /// Truncation selection (selecting from top percentage)
    Truncation(f64),
}

impl SelectionStrategy {
    /// Select two parent indices based on fitness scores
    pub fn select_parents(&self, fitness_scores: &[f64]) -> (usize, usize) {
        match self {
            Self::Tournament(size) => self.tournament_selection(fitness_scores, *size),
            Self::RouletteWheel => self.roulette_wheel_selection(fitness_scores),
            Self::Rank(pressure) => self.rank_selection(fitness_scores, *pressure),
            Self::Truncation(percentage) => self.truncation_selection(fitness_scores, *percentage),
        }
    }

    fn tournament_selection(&self, fitness_scores: &[f64], tournament_size: usize) -> (usize, usize) {
        let mut rng: rand::prelude::ThreadRng = thread_rng();
        let population_size: usize = fitness_scores.len();
        
        // First parent
        let mut best_idx1: usize = rng.gen_range(0..population_size);
        let mut best_fitness1: f64 = fitness_scores[best_idx1];
        
        for _ in 1..tournament_size {
            let idx: usize = rng.gen_range(0..population_size);
            if fitness_scores[idx] > best_fitness1 {
                best_idx1 = idx;
                best_fitness1 = fitness_scores[idx];
            }
        }
        
        // Second parent (ensure different from first)
        let mut best_idx2: usize = rng.gen_range(0..population_size);
        while best_idx2 == best_idx1 && population_size > 1 {
            best_idx2 = rng.gen_range(0..population_size);
        }
        
        let mut best_fitness2: f64 = fitness_scores[best_idx2];
        
        for _ in 1..tournament_size {
            let idx: usize = rng.gen_range(0..population_size);
            if idx != best_idx1 && fitness_scores[idx] > best_fitness2 {
                best_idx2 = idx;
                best_fitness2 = fitness_scores[idx];
            }
        }
        
        (best_idx1, best_idx2)
    }

    fn roulette_wheel_selection(&self, fitness_scores: &[f64]) -> (usize, usize) {
        let mut rng = thread_rng();
        let total_fitness: f64 = fitness_scores.iter().sum();
        
        // Handle edge case of zero total fitness
        if total_fitness <= 0.0 {
            let n: usize = fitness_scores.len();
            return (rng.gen_range(0..n), rng.gen_range(0..n));
        }
        
        // Select first parent
        let mut spin: f64 = rng.gen_range(0.0..total_fitness);
        let mut parent1: usize = 0;
        
        for (i, fitness) in fitness_scores.iter().enumerate() {
            spin -= fitness;
            if spin <= 0.0 {
                parent1 = i;
                break;
            }
        }
        
        // Select second parent (ensure different from first)
        let mut parent2: usize = parent1;
        if fitness_scores.len() > 1 {
            while parent2 == parent1 {
                spin = rng.gen_range(0.0..total_fitness);
                for (i, fitness) in fitness_scores.iter().enumerate() {
                    spin -= fitness;
                    if spin <= 0.0 {
                        parent2 = i;
                        break;
                    }
                }
            }
        }
        
        (parent1, parent2)
    }
    
    fn rank_selection(&self, fitness_scores: &[f64], selection_pressure: f64) -> (usize, usize) {
        let mut rng: rand::prelude::ThreadRng = thread_rng();
        let n: usize = fitness_scores.len();
        
        // Rank individuals by fitness scores
        let mut ranked_indices: Vec<usize> = (0..n).collect();
        ranked_indices.sort_by(|&a, &b| fitness_scores[b].partial_cmp(&fitness_scores[a]).unwrap());
        
        // Calculate selection probabilities based on ranks
        let total_rank: f64 = (1..=n).map(|i| i as f64).sum();
        let probabilities: Vec<f64> = ranked_indices.iter()
            .map(|&idx| (n - idx) as f64 / total_rank * selection_pressure)
            .collect();
        
        // Select first parent
        let mut parent1: usize = 0;
        let mut spin: f64 = rng.gen_range(0.0..1.0);
        
        for (i, prob) in probabilities.iter().enumerate() {
            spin -= prob;
            if spin <= 0.0 {
                parent1 = i;
                break;
            }
        }
        
        // Select second parent (ensure different from first)
        let mut parent2: usize = parent1;
        while parent2 == parent1 && n > 1 {
            spin = rng.gen_range(0.0..1.0);
            for (i, prob) in probabilities.iter().enumerate() {
                spin -= prob;
                if spin <= 0.0 {
                    parent2 = i;
                    break;
                }
            }
        }
        
        (parent1, parent2)
    }
    
    fn truncation_selection(&self, fitness_scores: &[f64], percentage: f64) -> (usize, usize) {
        let mut rng: rand::prelude::ThreadRng = thread_rng();
        let n: usize = fitness_scores.len();
        
        // Sort indices by fitness scores
        let mut indices: Vec<usize> = (0..n).collect();
        indices.sort_by(|&a, &b| fitness_scores[b].partial_cmp(&fitness_scores[a]).unwrap());
        
        // Select top percentage of individuals
        let cutoff: usize = (n as f64 * percentage).round() as usize;
        let selected_indices: Vec<usize> = indices[..cutoff].to_vec();
        
        // Select two parents from the top individuals
        let parent1: usize = selected_indices[rng.gen_range(0..cutoff)];
        let parent2: usize = selected_indices[rng.gen_range(0..cutoff)];
        
        (parent1, parent2)
    }

    /// Select indices for death based on fitness scores
    /// 
    /// # Arguments
    /// 
    /// - `fitness_scores`: A slice of fitness scores for the population.
    /// - `percentage`: The percentage of individuals remaining after selection.
    /// 
    /// # Returns
    /// 
    /// A vector of indices representing the individuals selected for death.
    pub fn select_deaths(&self, fitness_scores: &[f64], percentage: f64) -> Vec<usize> {
        match self {
            Self::Tournament(size) => self.tournament_selection_death(fitness_scores, *size),
            Self::RouletteWheel => self.roulette_wheel_selection_death(fitness_scores),
            Self::Rank(pressure) => self.rank_selection_death(fitness_scores, *pressure),
            Self::Truncation(percentage) => self.truncation_selection_death(fitness_scores, *percentage),
        }
    }

    fn tournament_selection_death(&self, fitness_scores: &[f64], tournament_size: usize) -> Vec<usize> {
        let mut rng: rand::prelude::ThreadRng = thread_rng();
        let population_size: usize = fitness_scores.len();
        
        // Select individuals for the tournament
        let mut selected_indices: Vec<usize> = (0..population_size).collect();
        selected_indices.shuffle(&mut rng);
        
        // Select the best individuals from the tournament
        let mut best_indices: Vec<usize> = Vec::new();
        for i in 0..tournament_size {
            best_indices.push(selected_indices[i]);
        }
        
        // Sort the best indices by fitness scores
        best_indices.sort_by(|&a, &b| fitness_scores[b].partial_cmp(&fitness_scores[a]).unwrap());
        
        // Select the worst individuals for death
        let num_deaths: usize = (population_size as f64 * (1.0 - percentage)).round() as usize;
        best_indices[num_deaths..].to_vec()
    }
}
