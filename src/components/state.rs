use std::fmt::Debug;

pub trait State: Clone + Copy + PartialEq + Eq + Debug + Send + Sync + 'static {}

pub mod common_states {
    use super::State;

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum GameOfLifeState {
        Dead,
        Alive,
    }

    impl State for GameOfLifeState {}

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum LangtonsAntState {
        Empty,
        Ant,
    }

    impl State for LangtonsAntState {}
}