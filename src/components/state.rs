use std::fmt::Debug;

/// The `State` trait is used to define the possible states of a cell in a cellular automaton.
pub trait State: Clone + Copy + PartialEq + Eq + Debug + Send + Sync + 'static {}

pub mod common_states {
    use super::State;
    use crate::components::board::Colour;

    /// State representation for the Game of Life cellular automaton.
    /// 
    /// Implements Into<Colour> for visualisation purposes.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum GameOfLifeState {
        Dead,
        Alive,
    }

    impl State for GameOfLifeState {}
    impl Into<Colour> for GameOfLifeState {
        fn into(self) -> Colour {
            match self {
                GameOfLifeState::Dead => Colour::black(),
                GameOfLifeState::Alive => Colour::white(),
            }
        }
    }

    /// State representation for Langton's Ant facing direction.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum AntDirection {
        Up,
        Right,
        Down,
        Left,
    }

    /// State representation for Langton's Ant cell colour.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum CellColour {
        White,
        Black,
    }

    /// Stores the cell colour and optionally an ant with a facing direction.
    ///
    /// The ant direction is optional because the ant may not be present on the cell.
    /// If the ant is not present, the ant direction is None.
    /// 
    /// Implements Into<Colour> for visualisation purposes.
    ///
    /// # Fields
    ///
    /// - `colour`: The colour of the cell.
    /// - `ant_direction`: The direction the ant is facing, if present.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct LangtonsAntState {
        pub colour: CellColour,
        pub ant_direction: Option<AntDirection>,
    }

    impl State for LangtonsAntState {}
    impl Into<Colour> for LangtonsAntState {
        fn into(self) -> Colour {
            if let Some(_ant_direction) = self.ant_direction {
                Colour::red()
            } else {
                match self.colour {
                    CellColour::White => Colour::white(),
                    CellColour::Black => Colour::black(),
                }
            }
        }
    }
}
