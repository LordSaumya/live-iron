use crate::components::{
    board::{Board, BoundaryCondition},
    rule::{Rule, Delta},
    rule::common_rules::{GameOfLifeRule, LangtonsAntRule},
    state::common_states::{AntDirection, CellColour, GameOfLifeState, LangtonsAntState},
};

#[test]
fn test_rule_game_of_life_underpopulation() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead],
        vec![GameOfLifeState::Dead, GameOfLifeState::Alive, GameOfLifeState::Alive],
        vec![GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead],
    ];

    let mut board: Board<GameOfLifeState> = Board::new(initial_state, BoundaryCondition::Periodic);

    let rule: GameOfLifeRule = GameOfLifeRule;
    let result: Vec<Delta<GameOfLifeState>> = rule.delta((1, 1), &mut board).unwrap();

    // alive + 1 neighbour => death
    let expected_delta: Delta<GameOfLifeState> = Delta::new(1, 1, GameOfLifeState::Dead);
    assert_eq!(result, vec![expected_delta]);
}

#[test]
fn test_rule_game_of_life_survival() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead],
        vec![GameOfLifeState::Dead, GameOfLifeState::Alive, GameOfLifeState::Alive],
        vec![GameOfLifeState::Dead, GameOfLifeState::Alive, GameOfLifeState::Dead],
    ];

    let mut board: Board<GameOfLifeState> = Board::new(initial_state, BoundaryCondition::Periodic);

    let rule: GameOfLifeRule = GameOfLifeRule;
    let result:Vec<Delta<GameOfLifeState>> = rule.delta((1, 1), &mut board).unwrap();

    // alive + 2 or 3 neighbours => survival
    let expected_delta: Delta<GameOfLifeState> = Delta::new(1, 1, GameOfLifeState::Alive);
    assert_eq!(result, vec![expected_delta]);
}

#[test]
fn test_rule_game_of_life_overpopulation() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![GameOfLifeState::Dead, GameOfLifeState::Alive, GameOfLifeState::Dead],
        vec![GameOfLifeState::Alive, GameOfLifeState::Alive, GameOfLifeState::Alive],
        vec![GameOfLifeState::Dead, GameOfLifeState::Alive, GameOfLifeState::Dead],
    ];

    let mut board: Board<GameOfLifeState> = Board::new(initial_state, BoundaryCondition::Periodic);

    let rule: GameOfLifeRule = GameOfLifeRule;
    let result:Vec<Delta<GameOfLifeState>> = rule.delta((1, 1), &mut board).unwrap();

    // alive + 4 neighbours => death
    let expected_delta: Delta<GameOfLifeState> = Delta::new(1, 1, GameOfLifeState::Dead);
    assert_eq!(result, vec![expected_delta]);
}

#[test]
fn test_rule_game_of_life_reproduction() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![GameOfLifeState::Dead, GameOfLifeState::Dead, GameOfLifeState::Dead],
        vec![GameOfLifeState::Alive, GameOfLifeState::Dead, GameOfLifeState::Alive],
        vec![GameOfLifeState::Dead, GameOfLifeState::Alive, GameOfLifeState::Dead],
    ];

    let mut board: Board<GameOfLifeState> = Board::new(initial_state, BoundaryCondition::Periodic);

    let rule: GameOfLifeRule = GameOfLifeRule;
    let result:Vec<Delta<GameOfLifeState>> = rule.delta((1, 1), &mut board).unwrap();

    // dead + 3 neighbours => reproduction
    let expected_delta: Delta<GameOfLifeState> = Delta::new(1, 1, GameOfLifeState::Alive);
    assert_eq!(result, vec![expected_delta]);
}

#[test]
fn test_rule_langtons_ant_no_change() {
    let initial_state: Vec<Vec<LangtonsAntState>> = vec![
        vec!['B', 'B', 'B'],
        vec!['B', 'A', 'B'],
        vec!['B', 'B', 'B'],
    ].iter().map(|x| x.iter().map(|&y| match y {
        'W' => LangtonsAntState {
            colour: CellColour::White,
            ant_direction: None,
        },
        'B' => LangtonsAntState {
            colour: CellColour::Black,
            ant_direction: None,
        },
        'A' => LangtonsAntState {
            colour: CellColour::White,
            ant_direction: Some(AntDirection::Up),
        },
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let mut board: Board<LangtonsAntState> = Board::new(initial_state, BoundaryCondition::Periodic);

    let rule: LangtonsAntRule = LangtonsAntRule;
    let result:Vec<Delta<LangtonsAntState>> = rule.delta((0, 0), &mut board).unwrap();

    // ant not present => one delta with no change
    let expected_delta: Delta<LangtonsAntState> = Delta::new(0, 0, LangtonsAntState {
        colour: CellColour::Black,
        ant_direction: None,
    });
    assert_eq!(result, vec![expected_delta]);
}

#[test]
fn test_rule_langtons_ant_turn_right() {
    let initial_state: Vec<Vec<LangtonsAntState>> = vec![
        vec!['B', 'B', 'B'],
        vec!['B', 'A', 'B'],
        vec!['B', 'B', 'B'],
    ].iter().map(|x| x.iter().map(|&y| match y {
        'W' => LangtonsAntState {
            colour: CellColour::White,
            ant_direction: None,
        },
        'B' => LangtonsAntState {
            colour: CellColour::Black,
            ant_direction: None,
        },
        'A' => LangtonsAntState {
            colour: CellColour::White,
            ant_direction: Some(AntDirection::Up),
        },
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let mut board: Board<LangtonsAntState> = Board::new(initial_state, BoundaryCondition::Periodic);

    let rule: LangtonsAntRule = LangtonsAntRule;
    let result:Vec<Delta<LangtonsAntState>> = rule.delta((1, 1), &mut board).unwrap();

    // Old cell: ant no longer present, white cell => black cell
    let expected_delta_1: Delta<LangtonsAntState> = Delta::new(1, 1, LangtonsAntState {
        colour: CellColour::Black,
        ant_direction: None,
    });

    // New cell: ant facing right, black cell
    let expected_delta_2: Delta<LangtonsAntState> = Delta::new(2, 1, LangtonsAntState {
        colour: CellColour::Black,
        ant_direction: Some(AntDirection::Right),
    });

    assert_eq!(result, vec![expected_delta_1, expected_delta_2]);
}

#[test]
fn test_rule_langtons_ant_turn_left() {
    let initial_state: Vec<Vec<LangtonsAntState>> = vec![
        vec!['B', 'B', 'B'],
        vec!['B', 'A', 'B'],
        vec!['B', 'B', 'B'],
    ].iter().map(|x| x.iter().map(|&y| match y {
        'W' => LangtonsAntState {
            colour: CellColour::White,
            ant_direction: None,
        },
        'B' => LangtonsAntState {
            colour: CellColour::Black,
            ant_direction: None,
        },
        'A' => LangtonsAntState {
            colour: CellColour::White,
            ant_direction: Some(AntDirection::Down),
        },
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let mut board: Board<LangtonsAntState> = Board::new(initial_state, BoundaryCondition::Periodic);

    let rule: LangtonsAntRule = LangtonsAntRule;
    let result:Vec<Delta<LangtonsAntState>> = rule.delta((1, 1), &mut board).unwrap();

    // Old cell: ant no longer present, white cell => black cell
    let expected_delta_1: Delta<LangtonsAntState> = Delta::new(1, 1, LangtonsAntState {
        colour: CellColour::Black,
        ant_direction: None,
    });

    // New cell: ant facing left, black cell
    let expected_delta_2: Delta<LangtonsAntState> = Delta::new(0, 1, LangtonsAntState {
        colour: CellColour::Black,
        ant_direction: Some(AntDirection::Left),
    });

    assert_eq!(result, vec![expected_delta_1, expected_delta_2]);
}