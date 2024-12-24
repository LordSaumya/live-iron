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

    /// State representation for the Langton's Ant cellular automaton.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum LangtonsAntState {
        Empty,
        Ant,
    }

    impl State for LangtonsAntState {}
}