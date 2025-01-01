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
    println!("{:?}", automaton.rules().len());

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