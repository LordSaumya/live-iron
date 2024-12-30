use crate::automaton::Automaton;
use crate::components::board::Board;
use crate::components::state::common_states::GameOfLifeState;
use crate::components::board::BoundaryCondition;
use crate::components::rule::Rule;
use crate::components::rule::common_rules::GameOfLifeRule;

#[test]
fn test_automaton_new() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![vec![GameOfLifeState::Dead; 10]; 10];
    let mut board: Board<GameOfLifeState> = Board::new(initial_state, BoundaryCondition::Periodic);
    let rules: Vec<Box<dyn Rule<GameOfLifeState>>> = vec![Box::new(GameOfLifeRule {})];
    let _automaton: Automaton<'_, GameOfLifeState> = Automaton::new(&mut board, rules);
    }

#[test]
fn test_automaton_curr_time() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![vec![GameOfLifeState::Dead; 10]; 10];
    let mut board: Board<GameOfLifeState> = Board::new(initial_state, BoundaryCondition::Periodic);
    let rules: Vec<Box<dyn Rule<GameOfLifeState>>> = vec![Box::new(GameOfLifeRule {})];
    let automaton: Automaton<'_, GameOfLifeState> = Automaton::new(&mut board, rules);

    assert_eq!(automaton.curr_time(), 0);
}

#[test]
fn test_automaton_board() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![vec![GameOfLifeState::Dead; 10]; 10];
    let mut board: Board<GameOfLifeState> = Board::new(initial_state, BoundaryCondition::Periodic);
    let board_clone: Board<GameOfLifeState> = board.clone();
    let rules: Vec<Box<dyn Rule<GameOfLifeState>>> = vec![Box::new(GameOfLifeRule {})];
    let automaton: Automaton<'_, GameOfLifeState> = Automaton::new(&mut board, rules);

    assert_eq!(automaton.board(), &board_clone);
}

#[test]
fn test_automaton_rules() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![vec![GameOfLifeState::Dead; 10]; 10];
    let mut board: Board<GameOfLifeState> = Board::new(initial_state, BoundaryCondition::Periodic);
    let rules: Vec<Box<dyn Rule<GameOfLifeState>>> = vec![Box::new(GameOfLifeRule {}), Box::new(GameOfLifeRule {})];
    let automaton: Automaton<'_, GameOfLifeState> = Automaton::new(&mut board, rules);

    assert_eq!(automaton.rules().len(), 2);
}

#[test]
fn test_automaton_add_rule() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![vec![GameOfLifeState::Dead; 10]; 10];
    let mut board: Board<GameOfLifeState> = Board::new(initial_state, BoundaryCondition::Periodic);
    let rules: Vec<Box<dyn Rule<GameOfLifeState>>> = vec![Box::new(GameOfLifeRule {})];
    let mut automaton: Automaton<'_, GameOfLifeState> = Automaton::new(&mut board, rules);

    assert_eq!(automaton.rules().len(), 1);

    automaton.add_rule(Box::new(GameOfLifeRule {}));

    assert_eq!(automaton.rules().len(), 2);
}

#[test]
fn test_automaton_evolve_game_of_life_one_step() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![GameOfLifeState::Dead, GameOfLifeState::Alive, GameOfLifeState::Dead],
        vec![GameOfLifeState::Dead, GameOfLifeState::Alive, GameOfLifeState::Dead],
        vec![GameOfLifeState::Dead, GameOfLifeState::Alive, GameOfLifeState::Dead],
    ];
    let mut board: Board<GameOfLifeState> = Board::new(initial_state, BoundaryCondition::Fixed(GameOfLifeState::Dead));
    let rules: Vec<Box<dyn Rule<GameOfLifeState>>> = vec![Box::new(GameOfLifeRule {})];
    let mut automaton: Automaton<'_, GameOfLifeState> = Automaton::new(&mut board, rules);

    let _ = automaton.evolve(1).unwrap();

    let expected_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead],
        vec![GameOfLifeState::Alive, GameOfLifeState::Alive, GameOfLifeState::Alive],
        vec![GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead],
    ];
    let expected_board: Board<GameOfLifeState> = Board::new(expected_state, BoundaryCondition::Fixed(GameOfLifeState::Dead));

    assert_eq!(automaton.board(), &expected_board);
    assert_eq!(automaton.curr_time(), 1);
}

#[test]
fn test_automaton_evolve_game_of_life_two_steps() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead],
        vec![GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead],
        vec![GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Alive, GameOfLifeState::Dead, GameOfLifeState::Dead],
        vec![GameOfLifeState::Dead, GameOfLifeState::Alive, GameOfLifeState::Alive, GameOfLifeState::Alive, GameOfLifeState::Dead],
        vec![GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead],
    ];
    let mut board: Board<GameOfLifeState> = Board::new(initial_state, BoundaryCondition::Periodic);
    let rules: Vec<Box<dyn Rule<GameOfLifeState>>> = vec![Box::new(GameOfLifeRule {})];
    let mut automaton: Automaton<'_, GameOfLifeState> = Automaton::new(&mut board, rules);

    let _ = automaton.evolve(2).unwrap();

    let expected_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead],
        vec![GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Alive, GameOfLifeState::Dead, GameOfLifeState::Dead],
        vec![GameOfLifeState::Dead, GameOfLifeState::Alive, GameOfLifeState::Dead, GameOfLifeState::Alive, GameOfLifeState::Dead],
        vec![GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead],
        vec![GameOfLifeState::Dead, GameOfLifeState::Alive, GameOfLifeState::Alive, GameOfLifeState::Alive, GameOfLifeState::Dead],
    ];
    let expected_board: Board<GameOfLifeState> = Board::new(expected_state, BoundaryCondition::Periodic);

    assert_eq!(automaton.board(), &expected_board);
    assert_eq!(automaton.curr_time(), 2);
}

#[test]
fn test_automaton_evolve_game_of_life_ten_steps() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead],
        vec![GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead],
        vec![GameOfLifeState::Dead, GameOfLifeState::Alive, GameOfLifeState::Alive, GameOfLifeState::Dead, GameOfLifeState::Dead],
        vec![GameOfLifeState::Dead, GameOfLifeState::Alive, GameOfLifeState::Alive, GameOfLifeState::Dead, GameOfLifeState::Dead],
        vec![GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead],
    ];
    let mut board: Board<GameOfLifeState> = Board::new(initial_state, BoundaryCondition::Periodic);
    let rules: Vec<Box<dyn Rule<GameOfLifeState>>> = vec![Box::new(GameOfLifeRule {})];
    let mut automaton: Automaton<'_, GameOfLifeState> = Automaton::new(&mut board, rules);

    let _ = automaton.evolve(10).unwrap();

    let expected_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead],
        vec![GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead],
        vec![GameOfLifeState::Dead, GameOfLifeState::Alive, GameOfLifeState::Alive, GameOfLifeState::Dead, GameOfLifeState::Dead],
        vec![GameOfLifeState::Dead, GameOfLifeState::Alive, GameOfLifeState::Alive, GameOfLifeState::Dead, GameOfLifeState::Dead],
        vec![GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead],
    ];
    let expected_board: Board<GameOfLifeState> = Board::new(expected_state, BoundaryCondition::Periodic);

    assert_eq!(automaton.board(), &expected_board);
    assert_eq!(automaton.curr_time(), 10);
}