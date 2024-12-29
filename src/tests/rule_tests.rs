use crate::components::{
    board::{Board, BoundaryCondition},
    rule::Rule,
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
    let result: GameOfLifeState = rule.apply((1, 1), &mut board).unwrap();

    // alive + 1 neighbour => death
    assert_eq!(result, GameOfLifeState::Dead);
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
    let result: GameOfLifeState = rule.apply((1, 1), &mut board).unwrap();
    // alive + 2 or 3 neighbours => survival
    assert_eq!(result, GameOfLifeState::Alive);
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
    let result: GameOfLifeState = rule.apply((1, 1), &mut board).unwrap();
    // alive + 4 neighbours => death
    assert_eq!(result, GameOfLifeState::Dead);
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
    let result: GameOfLifeState = rule.apply((1, 1), &mut board).unwrap();
    // dead + 3 neighbours => reproduction
    assert_eq!(result, GameOfLifeState::Alive);
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
    let result: LangtonsAntState = rule.apply((0, 0), &mut board).unwrap();

    // ant not present => no change
    assert_eq!(result, LangtonsAntState {
        colour: CellColour::Black,
        ant_direction: None,
    });
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
    let result: LangtonsAntState = rule.apply((1, 1), &mut board).unwrap();

    // Old cell: ant no longer present, white cell => black cell
    assert_eq!(result, LangtonsAntState {
        colour: CellColour::Black,
        ant_direction: None,
    });

    // New cell: ant facing right, black cell
    assert_eq!(board.get(2, 1).unwrap(), LangtonsAntState {
        colour: CellColour::Black,
        ant_direction: Some(AntDirection::Right),
    });
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
    let result: LangtonsAntState = rule.apply((1, 1), &mut board).unwrap();

    // Old cell: ant no longer present, white cell => black cell
    assert_eq!(result, LangtonsAntState {
        colour: CellColour::Black,
        ant_direction: None,
    });

    // New cell: ant facing left, black cell
    assert_eq!(board.get(0, 1).unwrap(), LangtonsAntState {
        colour: CellColour::Black,
        ant_direction: Some(AntDirection::Left),
    });
}

#[test]
fn test_rule_langtons_ant_rule_twice() {
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
    let result_1: LangtonsAntState = rule.apply((1, 1), &mut board).unwrap();
    let result_2: LangtonsAntState = rule.apply((2, 1), &mut board).unwrap();

    // Old cell: ant no longer present, white cell => black cell
    assert_eq!(result_1, LangtonsAntState {
        colour: CellColour::Black,
        ant_direction: None,
    });

    // 2nd cell: ant not present, black cell => white cell
    assert_eq!(result_2, LangtonsAntState {
        colour: CellColour::White,
        ant_direction: None,
    });

    // Last cell: ant facing up, black cell
    assert_eq!(board.get(2, 0).unwrap(), LangtonsAntState {
        colour: CellColour::Black,
        ant_direction: Some(AntDirection::Up),
    });
}