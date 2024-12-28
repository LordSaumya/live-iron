use crate::components::{state::State, board::Board, state::common_states::GameOfLifeState, error::OutOfBoundsSetError, board::BoundaryCondition};

#[test]
fn test_board_new_no_panic() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0],
        vec![1, 0, 1],
        vec![0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let _board: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);
}

#[test]
fn test_board_width() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0],
        vec![1, 0, 1],
        vec![0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let board: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);

    assert_eq!(board.width(), 3);
}

#[test]
fn test_board_height() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0],
        vec![1, 0, 1],
        vec![0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let board: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);

    assert_eq!(board.height(), 3);
}

#[test]
fn test_board_get_boundary_condition() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0],
        vec![1, 0, 1],
        vec![0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let board: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);

    assert_eq!(board.boundary_condition(), BoundaryCondition::Periodic);
}

#[test]
fn test_board_get_some() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0],
        vec![1, 0, 1],
        vec![0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let board: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);

    for i in 0..3 {
        for j in 0..3 {
            assert_eq!(board.get(i, j).unwrap(), initial_state[j][i]);
        }
    }
}

#[test]
fn test_board_get_none() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0],
        vec![1, 0, 1],
        vec![0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let board: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);

    assert!(board.get(3, 3).is_none());
}

#[test]
fn test_board_set_no_panic() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0],
        vec![1, 0, 1],
        vec![0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let mut board: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);

    for i in 0..3 {
        for j in 0..3 {
            board.set(i, j, initial_state[i][j]).unwrap();
        }
    }

    for i in 0..3 {
        for j in 0..3 {
            assert_eq!(board.get(i, j).unwrap(), initial_state[j][i]);
        }
    }
}

#[test]
fn test_board_set_out_of_bounds() {
    let initial_state: Vec<Vec<GameOfLifeState>> = vec![
        vec![0, 1, 0],
        vec![1, 0, 1],
        vec![0, 1, 0],
    ].iter().map(|x| x.iter().map(|&y| match y {
        0 => GameOfLifeState::Dead,
        1 => GameOfLifeState::Alive,
        _ => panic!("Invalid state"),
    }).collect()).collect();

    let mut board: Board<GameOfLifeState> = Board::new(initial_state.clone(), BoundaryCondition::Periodic);

    assert!(board.set(3, 3, GameOfLifeState::Dead).unwrap_err() == OutOfBoundsSetError { x: 3, y: 3, width: 3, height: 3 });
}