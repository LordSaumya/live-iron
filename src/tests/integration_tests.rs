use std::collections::HashMap;
use crate::components::error::OutOfBoundsSetError;
use crate::components::rule::Delta;
use crate::components::{
    board::{Board, BoundaryCondition},
    rule::Rule,
    state::State,
};
use crate::automaton::Automaton;
use rand::Rng;

#[test]
fn test_forest_fire_ca() {
    // Define ForestFireState
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum ForestFireState {
        Empty,
        Tree,
        Burning,
    }
    impl State for ForestFireState {}

    // Define ForestFireRule
    struct ForestFireRule {
        burn_prob: f64,
        grow_prob: f64,
    }

    impl Rule<ForestFireState> for ForestFireRule {
        fn delta(&mut self, coords: (usize, usize), board: &Board<ForestFireState>) -> Result<Vec<Delta<ForestFireState>>, OutOfBoundsSetError> {
            let mut rng = rand::thread_rng();
            let mut deltas: Vec<Delta<ForestFireState>> = Vec::new();
            let state: ForestFireState = board.get(coords.0, coords.1).unwrap();

            match state {
                ForestFireState::Empty => {
                    if rng.gen::<f64>() < self.grow_prob {
                        deltas.push(Delta::new(coords.0,coords.1, ForestFireState::Tree));
                    }
                }
                ForestFireState::Tree => {
                    if rng.gen::<f64>() < self.burn_prob {
                        deltas.push(Delta::new(coords.0,coords.1, ForestFireState::Burning));
                    }
                }
                ForestFireState::Burning => {
                    deltas.push(Delta::new(coords.0,coords.1, ForestFireState::Empty));
                }
            }
            Ok(deltas)
        }
    }

    // Define initial state
    let initial_state: Vec<Vec<ForestFireState>> = vec![vec![ForestFireState::Empty; 10]; 10];
    let mut board: Board<ForestFireState> = Board::new(initial_state, BoundaryCondition::Fixed(ForestFireState::Empty));
    let _ = board.set(5, 5, ForestFireState::Burning).unwrap();

    // Define rules vector
    let rules: Vec<Box<dyn Rule<ForestFireState>>> = vec![];

    // Get stats of initial state
    let mut init_tree_count: i32 = 0;
    let mut init_burning_count: i32 = 0;
    let mut init_empty_count: i32 = 0;
    board.iter_coords().for_each(|coord| {
        match board.get(coord.0, coord.1).unwrap() {
            ForestFireState::Tree => init_tree_count += 1,
            ForestFireState::Burning => init_burning_count += 1,
            ForestFireState::Empty => init_empty_count += 1,
        }
    });

    // Create automaton
    let mut automaton: Automaton<ForestFireState> = Automaton::new(&mut board, rules);

     // Add rule to automaton (probabilities are set to 0.0 and 1.0 for testing predictability)
     let forest_fire_rule: ForestFireRule = ForestFireRule {
        burn_prob: 0.0,
        grow_prob: 1.0,
    };
    automaton.add_rule(Box::new(forest_fire_rule));

    // Evolve automaton
    let _ = automaton.evolve(10);

    // Get stats of final state
    let mut final_tree_count: i32 = 0;
    let mut final_burning_count: i32 = 0;
    let mut final_empty_count: i32 = 0;

    board.iter_coords().for_each(|coord| {
        match board.get(coord.0, coord.1).unwrap() {
            ForestFireState::Tree => final_tree_count += 1,
            ForestFireState::Burning => final_burning_count += 1,
            ForestFireState::Empty => final_empty_count += 1,
        }
    });
    
    // Check if final state is different from initial state
    assert_ne!(init_tree_count, final_tree_count);
    assert_ne!(init_burning_count, final_burning_count);
    assert_ne!(init_empty_count, final_empty_count);
}

#[test]
fn test_genetic_ca() {
    // Define GeneticState
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    enum GeneticState {
        A,
        B,
        Empty,
    }
    impl State for GeneticState {}

    // Define GeneticRule
    struct GeneticRule {
        weights: HashMap<GeneticState, Vec<f64>>,
    }

    impl Rule<GeneticState> for GeneticRule {
        fn delta(&mut self, coords: (usize, usize), board: &Board<GeneticState>) -> Result<Vec<Delta<GeneticState>>, OutOfBoundsSetError> {
            let state: GeneticState = board.get(coords.0, coords.1).unwrap();

            // Skip if cell is empty
            if state == GeneticState::Empty {
                return Ok(vec![]);
            }

            let weights: &Vec<f64> = &self.weights[&state];

            // Add weights to x and y coordinates to generate delta
            let delta_remove: Delta<GeneticState> = Delta::new(coords.0, coords.1, GeneticState::Empty);
            let delta_move: Delta<GeneticState> = Delta::new(coords.0 + weights[0] as usize, coords.1 + weights[1] as usize, state);

            // Evolve weights
            let _ = self.evolve(state);

            Ok(vec![delta_remove, delta_move])
        }
    }

    impl GeneticRule {
        fn evolve(&mut self, state: GeneticState) -> Result<(), OutOfBoundsSetError> {
            // For testing purposes, weights evolve according to the following rules:
            // - If the sum of the weights is less than 2, add 0.5 to each weight.
            // - If the sum of the weights is greater than 2, subtract 0.5 from each weight.
            // - If the sum of the weights is equal to 2, do nothing.

            let sum: f64 = self.weights[&state].iter().sum();
            if sum < 2.0 {
                self.weights.get_mut(&state).unwrap().iter_mut().for_each(|weight| *weight += 0.5);
            } else if sum > 2.0 {
                self.weights.get_mut(&state).unwrap().iter_mut().for_each(|weight| *weight -= 0.5);
            }

            Ok(())
        }
    }

    // Define initial state
    let initial_state: Vec<Vec<GeneticState>> = vec![vec![GeneticState::Empty; 10]; 10];
    let mut board: Board<GeneticState> = Board::new(initial_state, BoundaryCondition::Periodic);
    let _ = board.set(2, 2, GeneticState::A).unwrap();
    let _ = board.set(6, 1, GeneticState::B).unwrap();

    // Define rules vector
    let rules: Vec<Box<dyn Rule<GeneticState>>> = vec![];

    // Create automaton
    let mut automaton: Automaton<GeneticState> = Automaton::new(&mut board, rules);

    // Add rule to automaton
    let mut weights: HashMap<GeneticState, Vec<f64>> = HashMap::new();
    weights.insert(GeneticState::A, vec![2.0, 2.0]);
    weights.insert(GeneticState::B, vec![-0.5, -0.5]);

    let genetic_rule: GeneticRule = GeneticRule {
        weights
    };
    automaton.add_rule(Box::new(genetic_rule));

    // Evolve automaton
    let _ = automaton.evolve(1);

    // Check positions of A and B after one step
    assert_eq!(automaton.board().get(4, 4).unwrap(), GeneticState::A);
    assert_eq!(automaton.board().get(6, 1).unwrap(), GeneticState::B);
    
    // Evolve automaton
    let _ = automaton.evolve(1);

    // Check positions of A and B after two steps
    assert_eq!(automaton.board().get(5, 5).unwrap(), GeneticState::A);
    assert_eq!(automaton.board().get(6, 1).unwrap(), GeneticState::B);

    // Evolve automaton
    let _ = automaton.evolve(2);

    // Check positions of A and B after four steps
    assert_eq!(automaton.board().get(7, 7).unwrap(), GeneticState::A);
    assert_eq!(automaton.board().get(7, 2).unwrap(), GeneticState::B);
}
