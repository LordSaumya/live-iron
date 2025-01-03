use crate::components::{board::Board, neighbourhood::Neighbourhood, neighbourhood::NeighbourhoodType, state::common_states::GameOfLifeState, board::BoundaryCondition};

#[test]
fn test_neighbourhood_new_no_panic() {
    let _neighbourhood_vn: Neighbourhood = Neighbourhood::new(NeighbourhoodType::VonNeumann, 1);
    let _neighbourhood_m: Neighbourhood = Neighbourhood::new(NeighbourhoodType::Moore, 1);
}

#[test]
fn test_neighbourhood_get_neighbourhood_coords_von_neumann_non_edge_rad_1() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let board_bc_p: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);
    let board_bc_f: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Fixed(GameOfLifeState::Dead));
    let mut neighbourhood: Neighbourhood = Neighbourhood::new(NeighbourhoodType::VonNeumann, 1);

    let expected_neighbourhood_p: Vec<Option<(usize, usize)>> = vec![
        Some((0, 1)),
        Some((1, 0)),
        Some((1, 1)),
        Some((1, 2)),
        Some((2, 1)),
    ];

    let expected_neighbourhood_f: Vec<Option<(usize, usize)>> = vec![
        Some((0, 1)),
        Some((1, 0)),
        Some((1, 1)),
        Some((1, 2)),
        Some((2, 1)),
    ];

    assert_eq!(neighbourhood.get_neighbourhood_coords(&board_bc_p, 1, 1), expected_neighbourhood_p);
    assert_eq!(neighbourhood.get_neighbourhood_coords(&board_bc_f, 1, 1), expected_neighbourhood_f);
}

#[test]
fn test_neighbourhood_get_neighbourhood_coords_von_neumann_edge_rad_1() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let board_bc_p: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);
    let board_bc_f: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Fixed(GameOfLifeState::Dead));
    let mut neighbourhood: Neighbourhood = Neighbourhood::new(NeighbourhoodType::VonNeumann, 1);

    let expected_neighbourhood_p: Vec<Option<(usize, usize)>> = vec![
        Some((4, 0)),
        Some((0, 4)),
        Some((0, 0)),
        Some((0, 1)),
        Some((1, 0)),
    ];

    let expected_neighbourhood_f: Vec<Option<(usize, usize)>> = vec![
        None,
        None,
        Some((0, 0)),
        Some((0, 1)),
        Some((1, 0)),
    ];


    assert_eq!(neighbourhood.get_neighbourhood_coords(&board_bc_p, 0, 0), expected_neighbourhood_p);
    assert_eq!(neighbourhood.get_neighbourhood_coords(&board_bc_f, 0, 0), expected_neighbourhood_f);
}

#[test]
fn test_neighbourhood_get_neighbourhood_coords_von_neumann_non_edge_rad_2() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let board_bc_p: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);
    let board_bc_f: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Fixed(GameOfLifeState::Dead));
    let mut neighbourhood: Neighbourhood = Neighbourhood::new(NeighbourhoodType::VonNeumann, 2);

    let expected_neighbourhood_bc_p: Vec<Option<(usize, usize)>> = vec![
        Some((4, 1)),
        Some((0, 0)),
        Some((0, 1)),
        Some((0, 2)),
        Some((1, 4)),
        Some((1, 0)),
        Some((1, 1)),
        Some((1, 2)),
        Some((1, 3)),
        Some((2, 0)),
        Some((2, 1)),
        Some((2, 2)),
        Some((3, 1))
    ];

    let expected_neighbourhood_bc_f: Vec<Option<(usize, usize)>> = vec![
        None,
        Some((0, 0)),
        Some((0, 1)),
        Some((0, 2)),
        None,
        Some((1, 0)),
        Some((1, 1)),
        Some((1, 2)),
        Some((1, 3)),
        Some((2, 0)),
        Some((2, 1)),
        Some((2, 2)),
        Some((3, 1))
    ];

    assert_eq!(neighbourhood.get_neighbourhood_coords(&board_bc_p, 1, 1), expected_neighbourhood_bc_p);
    assert_eq!(neighbourhood.get_neighbourhood_coords(&board_bc_f, 1, 1), expected_neighbourhood_bc_f);
}

#[test]
fn test_neighbourhood_get_neighbourhood_coords_von_neumann_edge_rad_2() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let board_bc_p: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);
    let board_bc_f: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Fixed(GameOfLifeState::Dead));
    let mut neighbourhood: Neighbourhood = Neighbourhood::new(NeighbourhoodType::VonNeumann, 2);

    let expected_neighbourhood_p: Vec<Option<(usize, usize)>> = vec![
        Some((3, 0)),
        Some((4, 4)),
        Some((4, 0)),
        Some((4, 1)),
        Some((0, 3)),
        Some((0, 4)),
        Some((0, 0)),
        Some((0, 1)),
        Some((0, 2)),
        Some((1, 4)),
        Some((1, 0)),
        Some((1, 1)),
        Some((2, 0))
    ];

    let expected_neighbourhood_f: Vec<Option<(usize, usize)>> = vec![
        None,
        None,
        None,
        None,
        None,
        None,
        Some((0, 0)),
        Some((0, 1)),
        Some((0, 2)),
        None,
        Some((1, 0)),
        Some((1, 1)),
        Some((2, 0))
    ];

    assert_eq!(neighbourhood.get_neighbourhood_coords(&board_bc_p, 0, 0), expected_neighbourhood_p);
    assert_eq!(neighbourhood.get_neighbourhood_coords(&board_bc_f, 0, 0), expected_neighbourhood_f);
}

#[test]
fn test_neighbourhood_get_neighbourhood_coords_moore_non_edge_rad_1() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let mut neighbourhood: Neighbourhood = Neighbourhood::new(NeighbourhoodType::Moore, 1);
    let board_bc_p: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);
    let board_bc_f: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Fixed(GameOfLifeState::Dead));

    let expected_neighbourhood_p: Vec<Option<(usize, usize)>> = vec![
        Some((0, 0)),
        Some((0, 1)),
        Some((0, 2)),
        Some((1, 0)),
        Some((1, 1)),
        Some((1, 2)),
        Some((2, 0)),
        Some((2, 1)),
        Some((2, 2)),
    ];

    let expected_neighbourhood_f: Vec<Option<(usize, usize)>> = vec![
        Some((0, 0)),
        Some((0, 1)),
        Some((0, 2)),
        Some((1, 0)),
        Some((1, 1)),
        Some((1, 2)),
        Some((2, 0)),
        Some((2, 1)),
        Some((2, 2)),
    ];

    assert_eq!(neighbourhood.get_neighbourhood_coords(&board_bc_p, 1, 1), expected_neighbourhood_p);
    assert_eq!(neighbourhood.get_neighbourhood_coords(&board_bc_f, 1, 1), expected_neighbourhood_f);
}

#[test]
fn test_neighbourhood_get_neighbourhood_coords_moore_edge_rad_1() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let mut neighbourhood: Neighbourhood = Neighbourhood::new(NeighbourhoodType::Moore, 1);
    let board_bc_p: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);
    let board_bc_f: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Fixed(GameOfLifeState::Dead));

    let expected_neighbourhood_p: Vec<Option<(usize, usize)>> = vec![
        Some((4, 4)),
        Some((4, 0)),
        Some((4, 1)),
        Some((0, 4)),
        Some((0, 0)),
        Some((0, 1)),
        Some((1, 4)),
        Some((1, 0)),
        Some((1, 1)),
    ];

    let expected_neighbourhood_f: Vec<Option<(usize, usize)>> = vec![
        None,
        None,
        None,
        None,
        Some((0, 0)),
        Some((0, 1)),
        None,
        Some((1, 0)),
        Some((1, 1)),
    ];

    assert_eq!(neighbourhood.get_neighbourhood_coords(&board_bc_p, 0, 0), expected_neighbourhood_p);
    assert_eq!(neighbourhood.get_neighbourhood_coords(&board_bc_f, 0, 0), expected_neighbourhood_f);
}

#[test]
fn test_neighbourhood_get_neighbourhood_coords_moore_non_edge_rad_2() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let board_bc_p: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);
    let board_bc_f: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Fixed(GameOfLifeState::Dead));
    let mut neighbourhood: Neighbourhood = Neighbourhood::new(NeighbourhoodType::Moore, 2);

    let expected_neighbourhood_bc_p: Vec<Option<(usize, usize)>> = vec![
        Some((4, 4)),
        Some((4, 0)),
        Some((4, 1)),
        Some((4, 2)),
        Some((4, 3)),
        Some((0, 4)),
        Some((0, 0)),
        Some((0, 1)),
        Some((0, 2)),
        Some((0, 3)),
        Some((1, 4)),
        Some((1, 0)),
        Some((1, 1)),
        Some((1, 2)),
        Some((1, 3)),
        Some((2, 4)),
        Some((2, 0)),
        Some((2, 1)),
        Some((2, 2)),
        Some((2, 3)),
        Some((3, 4)),
        Some((3, 0)),
        Some((3, 1)),
        Some((3, 2)),
        Some((3, 3)),
    ];

    let expected_neighbourhood_bc_f: Vec<Option<(usize, usize)>> = vec![
        None,
        None,
        None,
        None,
        None,
        None,
        Some((0, 0)),
        Some((0, 1)),
        Some((0, 2)),
        Some((0, 3)),
        None,
        Some((1, 0)),
        Some((1, 1)),
        Some((1, 2)),
        Some((1, 3)),
        None,
        Some((2, 0)),
        Some((2, 1)),
        Some((2, 2)),
        Some((2, 3)),
        None,
        Some((3, 0)),
        Some((3, 1)),
        Some((3, 2)),
        Some((3, 3)),
    ];

    assert_eq!(neighbourhood.get_neighbourhood_coords(&board_bc_p, 1, 1), expected_neighbourhood_bc_p);
    assert_eq!(neighbourhood.get_neighbourhood_coords(&board_bc_f, 1, 1), expected_neighbourhood_bc_f);
}

#[test]
fn test_neighbourhood_get_neighbourhood_coords_moore_edge_rad_2() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let board_bc_p: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);
    let board_bc_f: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Fixed(GameOfLifeState::Dead));
    let mut neighbourhood: Neighbourhood = Neighbourhood::new(NeighbourhoodType::Moore, 2);

    let expected_neighbourhood_p: Vec<Option<(usize, usize)>> = vec![
        Some((3, 3)),
        Some((3, 4)),
        Some((3, 0)),
        Some((3, 1)),
        Some((3, 2)),
        Some((4, 3)),
        Some((4, 4)),
        Some((4, 0)),
        Some((4, 1)),
        Some((4, 2)),
        Some((0, 3)),
        Some((0, 4)),
        Some((0, 0)),
        Some((0, 1)),
        Some((0, 2)),
        Some((1, 3)),
        Some((1, 4)),
        Some((1, 0)),
        Some((1, 1)),
        Some((1, 2)),
        Some((2, 3)),
        Some((2, 4)),
        Some((2, 0)),
        Some((2, 1)),
        Some((2, 2)),
    ];

    let expected_neighbourhood_f: Vec<Option<(usize, usize)>> = vec![
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some((0, 0)),
        Some((0, 1)),
        Some((0, 2)),
        None,
        None,
        Some((1, 0)),
        Some((1, 1)),
        Some((1, 2)),
        None,
        None,
        Some((2, 0)),
        Some((2, 1)),
        Some((2, 2)),
    ];

    assert_eq!(neighbourhood.get_neighbourhood_coords(&board_bc_p, 0, 0), expected_neighbourhood_p);
    assert_eq!(neighbourhood.get_neighbourhood_coords(&board_bc_f, 0, 0), expected_neighbourhood_f);
}

#[test]
fn test_neighbourhood_get_neighbourhood_states_von_neumann_non_edge_rad_1() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let board_bc_p: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);
    let board_bc_f: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Fixed(GameOfLifeState::Dead));
    let mut neighbourhood: Neighbourhood = Neighbourhood::new(NeighbourhoodType::VonNeumann, 1);

    let expected_neighbourhood_states_p: Vec<Option<GameOfLifeState>> = vec![
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Alive),
    ];

    let expected_neighbourhood_states_f: Vec<Option<GameOfLifeState>> = vec![
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Alive),
    ];

    assert_eq!(neighbourhood.get_neighbourhood_states(&board_bc_p, 1, 1), expected_neighbourhood_states_p);
    assert_eq!(neighbourhood.get_neighbourhood_states(&board_bc_f, 1, 1), expected_neighbourhood_states_f);
}

#[test]
fn test_neighbourhood_get_neighbourhood_states_von_neumann_edge_rad_1() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let board_bc_p: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);
    let board_bc_f: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Fixed(GameOfLifeState::Dead));
    let mut neighbourhood: Neighbourhood = Neighbourhood::new(NeighbourhoodType::VonNeumann, 1);

    let expected_neighbourhood_states_p: Vec<Option<GameOfLifeState>> = vec![
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Alive),
    ];

    let expected_neighbourhood_states_f: Vec<Option<GameOfLifeState>> = vec![
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Alive),
    ];

    assert_eq!(neighbourhood.get_neighbourhood_states(&board_bc_p, 0, 0), expected_neighbourhood_states_p);
    assert_eq!(neighbourhood.get_neighbourhood_states(&board_bc_f, 0, 0), expected_neighbourhood_states_f);
}

#[test]
fn test_neighbourhood_get_neighbourhood_states_von_neumann_non_edge_rad_2() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let board_bc_p: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);
    let board_bc_f: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Fixed(GameOfLifeState::Dead));
    let mut neighbourhood: Neighbourhood = Neighbourhood::new(NeighbourhoodType::VonNeumann, 2);

    let expected_neighbourhood_states_bc_p: Vec<Option<GameOfLifeState>> = vec![
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead)
    ];

    let expected_neighbourhood_states_bc_f: Vec<Option<GameOfLifeState>> = vec![
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead)
    ];

    assert_eq!(neighbourhood.get_neighbourhood_states(&board_bc_p, 1, 1), expected_neighbourhood_states_bc_p);
    assert_eq!(neighbourhood.get_neighbourhood_states(&board_bc_f, 1, 1), expected_neighbourhood_states_bc_f);
}

#[test]
fn test_neighbourhood_get_neighbourhood_states_von_neumann_edge_rad_2() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let board_bc_p: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);
    let board_bc_f: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Fixed(GameOfLifeState::Dead));
    let mut neighbourhood: Neighbourhood = Neighbourhood::new(NeighbourhoodType::VonNeumann, 2);

    let expected_neighbourhood_states_p: Vec<Option<GameOfLifeState>> = vec![
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
    ];

    let expected_neighbourhood_states_f: Vec<Option<GameOfLifeState>> = vec![
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
    ];

    assert_eq!(neighbourhood.get_neighbourhood_states(&board_bc_p, 0, 0), expected_neighbourhood_states_p);
    assert_eq!(neighbourhood.get_neighbourhood_states(&board_bc_f, 0, 0), expected_neighbourhood_states_f);
}

#[test]
fn test_neighbourhood_get_neighbourhood_states_moore_non_edge_rad_1() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let mut neighbourhood: Neighbourhood = Neighbourhood::new(NeighbourhoodType::Moore, 1);
    let board_bc_p: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);
    let board_bc_f: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Fixed(GameOfLifeState::Dead));

    let expected_neighbourhood_states_p: Vec<Option<GameOfLifeState>> = vec![
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
    ];

    let expected_neighbourhood_states_f: Vec<Option<GameOfLifeState>> = vec![
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
    ];

    assert_eq!(neighbourhood.get_neighbourhood_states(&board_bc_p, 1, 1), expected_neighbourhood_states_p);
    assert_eq!(neighbourhood.get_neighbourhood_states(&board_bc_f, 1, 1), expected_neighbourhood_states_f);
}

#[test]
fn test_neighbourhood_get_neighbourhood_states_moore_edge_rad_1() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let mut neighbourhood: Neighbourhood = Neighbourhood::new(NeighbourhoodType::Moore, 1);
    let board_bc_p: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);
    let board_bc_f: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Fixed(GameOfLifeState::Dead));

    let expected_neighbourhood_states_p: Vec<Option<GameOfLifeState>> = vec![
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
    ];

    let expected_neighbourhood_states_f: Vec<Option<GameOfLifeState>> = vec![
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
    ];

    assert_eq!(neighbourhood.get_neighbourhood_states(&board_bc_p, 0, 0), expected_neighbourhood_states_p);
    assert_eq!(neighbourhood.get_neighbourhood_states(&board_bc_f, 0, 0), expected_neighbourhood_states_f);
}

#[test]
fn test_neighbourhood_get_neighbourhood_states_moore_non_edge_rad_2() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let board_bc_p: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);
    let board_bc_f: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Fixed(GameOfLifeState::Dead));
    let mut neighbourhood: Neighbourhood = Neighbourhood::new(NeighbourhoodType::Moore, 2);

    let expected_neighbourhood_states_bc_p: Vec<Option<GameOfLifeState>> = vec![
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead)
    ];

    let expected_neighbourhood_states_bc_f: Vec<Option<GameOfLifeState>> = vec![
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead)
    ];

    assert_eq!(neighbourhood.get_neighbourhood_states(&board_bc_p, 1, 1), expected_neighbourhood_states_bc_p);
    assert_eq!(neighbourhood.get_neighbourhood_states(&board_bc_f, 1, 1), expected_neighbourhood_states_bc_f);
}

#[test]
fn test_neighbourhood_get_neighbourhood_states_moore_edge_rad_2() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let board_bc_p: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);
    let board_bc_f: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Fixed(GameOfLifeState::Dead));
    let mut neighbourhood: Neighbourhood = Neighbourhood::new(NeighbourhoodType::Moore, 2);

    let expected_neighbourhood_states_p: Vec<Option<GameOfLifeState>> = vec![
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead)
    ];

    let expected_neighbourhood_states_f: Vec<Option<GameOfLifeState>> = vec![
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Dead),
        Some(GameOfLifeState::Alive),
        Some(GameOfLifeState::Dead)
    ];

    assert_eq!(neighbourhood.get_neighbourhood_states(&board_bc_p, 0, 0), expected_neighbourhood_states_p);
    assert_eq!(neighbourhood.get_neighbourhood_states(&board_bc_f, 0, 0), expected_neighbourhood_states_f);
}

#[test]
fn test_neighbourhood_get_neighbourhood_states_coords() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let board_bc_p: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);

    let mut neighbourhood: Neighbourhood = Neighbourhood::new(NeighbourhoodType::VonNeumann, 2);
    let neighbourhood_coords: Vec<Option<(usize, usize)>> = neighbourhood.get_neighbourhood_coords(&board_bc_p, 1, 1);
    let neighbourhood_states: Vec<Option<GameOfLifeState>> = neighbourhood.get_neighbourhood_states(&board_bc_p, 1, 1);

    let relative_coords: Vec<(isize, isize)> = neighbourhood_coords.iter().map(|x| match x {
        Some((x, y)) => (*x as isize - 1, *y as isize - 1),
        None => panic!("Invalid state"),
    }).collect();

    let neighbourhood_states_coords: Vec<(Option<GameOfLifeState>, (isize, isize))> = neighbourhood.get_neighbourhood_states_coords(&board_bc_p, 1, 1);
    let expected_neighbourhood_states_coords: Vec<(Option<GameOfLifeState>, (isize, isize))> = neighbourhood_states.iter().zip(relative_coords.iter()).map(|(state, coords)| (*state, *coords)).collect();

    assert_eq!(neighbourhood_states_coords, expected_neighbourhood_states_coords);
}
