use std::fmt::Debug;

/// The `State` trait is used to define the possible states of a cell in a cellular automaton.
/// 
/// The trait implements Clone, Copy, PartialEq, Eq, Debug, Send, Sync, and 'static for Bevy compatibility.
pub trait State: Clone + Copy + PartialEq + Eq + Debug + Send + Sync + 'static {}

pub mod common_states {
    use super::State;

    /// State representation for the Game of Life cellular automaton.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum GameOfLifeState {
        Dead,
        Alive,
    }

    impl State for GameOfLifeState {}

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
}