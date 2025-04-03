# LiveIron

## Description

A performant cellular automata simulator library with visualisation capabilities built in Rust. The library is designed to be flexible and easy to use, with a focus on performance. It provides a simple API for creating and running cellular automata simulations, as well as visualising them using the `dioxus` library. In addition, the library also supports evolving cellular automata using genetic algorithms, as conceptualised by [Mitchell et al](https://www.sciencedirect.com/science/article/abs/pii/0167278994902933) using the `GeneticAutomaton` struct and its associated components.

## Components

### Cellular Automata

- `state`: One of the two atomic elements of a cellular automaton, the state module contains the `state` struct, which is used to represent the state of a cell in the simulation. It also contains `state` implementations for Conway's Game of Life and Langton's Ant.

- `rule`: One of the two atomic elements of a cellular automaton, the rule module contains the `rule` trait that defines how the state of a cell changes. It also contains `rule` implementations for Conway's Game of Life and Langton's Ant.

- `board`: The board module contains the `board` struct, which is the main data structure used to represent the board of cells in the simulation. It contains methods and structs to create the board with a specified `BoundaryCondition`, read and write cell states, and convert the board to its representation for visualisation.

- `neighbourhood`: The neighbourhood module contains the `neighbourhood` struct and `neighbourhood` implementations for the `Moore` and `VonNeumann` neighbourhoods. The `neighbourhood` struct is used to define the neighbourhood of a cell in the simulation, and implements methods to efficiently calculate the neighbours of a cell and return their states and coordinates.

- `automaton`: The most important module, the automaton module contains the `automaton` struct, which is used to represent the cellular automaton simulation. It contains methods to create an automaton with a specified board and a set of rules, evolve the automaton by applying the rules to the board, and visualise the automaton using the `ui` module.

- `ui`: The `ui` module uses the `dioxus` library to create a window and render the automaton to the screen. The module contains main simulation function as well as several Dioxus components used to visualise the automaton.

### Genetic Automata

- `genotype`: The atomic element of a genetic automaton, the `genotype` module contains the `genotype` trait, which is used to represent the genotype in the population. It replaces the `rule` trait in the `cellular automata` module as a genetic rule. The `genotype` trait is used to define how the genotype is represented, how it is mutated, how it is evaluated, and how it is used to create offspring.

- `population`: The population module contains the `population` struct, which is used to represent the population of genotypes in the simulation. It contains methods to create a population with a specified size and genotype, evaluate the population, select the best genotypes for reproduction, cull the population, and create offspring.

- `selection_strategy`: The selection strategy module contains the `SelectionStrategy` enum, which provides methods to select parents for reproduction from the population and select genotypes to cull from the population. It contains implementations for several selection strategies, including `Tournament`, `RouletteWheel`, `Rank`, and `Truncation`. The selection strategy is used to select the best genotypes for reproduction based on their fitness scores.

- `genetic_automaton`: The `genetic_automaton` module contains the `GeneticAutomaton` struct, which is used to represent the genetic automaton simulation. It contains methods to create a genetic automaton with a specified population and board, evolve the genetic automaton by applying the genotypes to the board, and visualise the genetic automaton using the `ui` module.

## Usage

To use the library, add the following to your `Cargo.toml`:

```toml
[dependencies]
live-iron = "0.1.2"
```

Using the library is both simple and flexible. Here's an example of Conway's Game of Life:

```rust
// Create a 20x20 board with a glider pattern
let mut initial_state: Vec<Vec<GameOfLifeState>> = vec![vec![GameOfLifeState::Dead; 20]; 20];

initial_state[0][0] = GameOfLifeState::Alive;
initial_state[0][2] = GameOfLifeState::Alive;
initial_state[1][1] = GameOfLifeState::Alive;
initial_state[1][2] = GameOfLifeState::Alive;
initial_state[2][1] = GameOfLifeState::Alive;

// Create a board with the initial state and periodic boundary conditions
let mut board: Board<GameOfLifeState> = Board::new(initial_state, BoundaryCondition::Periodic);

// Create a Game of Life rule and an automaton with the board and rule
let rule: GameOfLifeRule = GameOfLifeRule;
let mut automaton: automaton::Automaton<'_, GameOfLifeState> = automaton::Automaton::new(&mut board, vec![Box::new(rule)]);

// Visualise the automaton for 100 steps with an interval of 500ms.
let _ = automaton.visualise(100, 500);
```
