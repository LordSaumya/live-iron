use crate::{automaton::Automaton, components::board::{BoardRepresentation, Colour}};
use crate::components::state::State;
use dioxus::prelude::*;
use tokio::time::Interval;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

/// A private struct that represents the render context for the simulation.
#[derive(Debug, Clone)]
struct BoardSimulationRender {
    board_states: Vec<BoardRepresentation>,
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

    let mut board_representations: Vec<BoardRepresentation> = Vec::new();

    for _ in 0..steps {
        automaton.evolve(1).unwrap();
        board_representations.push(automaton.board().to_representation());
    }

    let render: BoardSimulationRender = BoardSimulationRender {
        board_states: board_representations,
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
            for row in board_state {
                tr {
                    for cell in row {
                        board_cell { colour: cell }
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

    let mut step: Signal<usize, SyncStorage> = use_signal_sync(|| 1);
    let mut board_state: Signal<Vec<Vec<Colour>>, SyncStorage> = use_signal_sync(|| render.board_states[0].clone());

    let interval: u64 = render.interval;
    let steps: usize = render.steps;

    let _update_task: Coroutine<()> = use_coroutine(move |_rx: UnboundedReceiver<()>| {
    let board_state_list: Vec<Vec<Vec<Colour>>> = render.board_states.clone();
    async move {
        let mut interval: Interval = tokio::time::interval(std::time::Duration::from_millis(interval));
        interval.tick().await;
        for i in 1..steps {
            step.set(i + 1);
            board_state.set(board_state_list[i].clone());
            interval.tick().await;
        }
    }});

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        style { {include_str!("../assets/main.css")} }
        h1 {"LiveIron Simulation"}
        board_table { board_state: board_state.cloned() }
        p { "Step {step}" }
    }
}
