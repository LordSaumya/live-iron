use crate::{automaton::Automaton, components::board::{BoardRepresentation, Colour}};
use crate::components::state::State;
use dioxus::prelude::*;
use tokio::time::Interval;
use std::sync::Arc;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

/// A private struct that represents the render context for the simulation.
#[derive(Debug, Clone)]
struct BoardSimulationRender {
    states: Arc<Vec<BoardRepresentation>>,
    steps: usize,
    interval: u64,
}

/// The main function that runs the simulation.
/// 
/// This function takes an automaton and runs the simulation for the given number of steps with the given interval between each step.
/// To ensure that the interval is consistent, board states are precomputed before rendering the simulation. This may lead to a delay before the simulation starts, depending on the number of steps.
/// 
/// Parameters:
/// 
/// - `automaton`: The automaton to run the simulation on.
/// 
/// - `steps`: The number of steps to run the simulation for.
/// 
/// - `interval`: The interval between each step in milliseconds.
pub fn simulate<S: State + Into<Colour>>(automaton: &mut Automaton<S>, steps: usize, interval: u64) {
    // Create a vector to store all board states
    let mut state_vec: Vec<BoardRepresentation> = Vec::with_capacity(steps + 1);
    
    // Store the initial state
    state_vec.push(automaton.board().to_representation());
    
    // Precompute all states upfront
    for _ in 0..steps {
        // Evolve the automaton
        if let Ok(_) = automaton.evolve(1) {
            let new_state: BoardRepresentation = automaton.board().to_representation();
            state_vec.push(new_state);
        }
    }
    
    // Wrap in Arc for thread-safe sharing
    let states: Arc<Vec<BoardRepresentation>> = Arc::new(state_vec);
    
    // Prepare the render context
    let render: BoardSimulationRender = BoardSimulationRender {
        states,
        steps,
        interval,
    };
    
    dioxus::LaunchBuilder::new().with_context(render).launch(App);
}

/// A component that represents a cell in the board.
/// 
/// This component takes a `Colour` as a prop and renders a cell with the given colour.
/// 
/// Parameters:
/// 
/// - `colour`: The colour of the cell.
#[component]
fn board_cell(colour: Colour) -> Element {
    rsx! {
        td { class: "cell", style: format!("background-color: {}", String::from(colour)) }
    }
}

/// A component that represents the board as a table.
/// 
/// This component takes a `BoardRepresentation` as a prop and renders the board as a table.
/// 
/// Parameters:
/// 
/// - `board_state`: The state of the board represented as a 2D vector of `Colour`.
#[component]
pub fn board_table(board_state: BoardRepresentation) -> Element {
    rsx! {
        table { class: "board",
            for (row_idx, row) in board_state.iter().enumerate() {
                tr { key: "{row_idx}",
                    for (cell_idx, cell) in row.iter().enumerate() {
                        board_cell { key: "{cell_idx}", colour: *cell }
                    }
                }
            }
        }
    }
}

/// The main application component that renders the simulation.
#[component]
fn App() -> Element {
    let render: BoardSimulationRender = use_context::<BoardSimulationRender>();
    
    let step: Signal<usize> = use_signal(|| 0);
    
    let board_state: BoardRepresentation = {
        let current_index: usize = step.read().min(render.states.len().saturating_sub(1));
        render.states.get(current_index).cloned().unwrap_or_default()
    };
    
    let _update_task: Coroutine<()> = use_coroutine(move |_rx: UnboundedReceiver<()>| {
        let mut step_clone: Signal<usize> = step.clone();
        let steps: usize = render.steps;
        let interval_ms: u64 = render.interval;
        
        async move {
            let mut interval: Interval = tokio::time::interval(std::time::Duration::from_millis(interval_ms));
            interval.tick().await;
            
            for i in 1..steps {
                step_clone.set(i + 1);
                interval.tick().await;
            }
        }
    });
    
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        style { {include_str!("../assets/main.css")} }
        h1 {"LiveIron Simulation"}
        board_table { board_state: board_state }
        p { "Step {step}" }
    }
}
