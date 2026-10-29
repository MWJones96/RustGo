# RustGo

A rules engine for the board game Go, written in Rust and organised along Clean Architecture lines.

## Overview

RustGo models a Go board and the rules for placing stones. The code is split into a **domain** layer (the board, the game and the logic for finding groups and liberties) and a **use cases** layer, which exposes "place a piece" through input, output, requester and interactor types in the style of Clean Architecture.

It is a library-style core with no user interface: `main` is empty, and the behaviour is exercised through unit tests.

## Features

- Square boards of any size (`GoBoard::new(size)`), with black and white stones
- Turn order: black moves first, and a move by the wrong player is rejected
- Rejection of moves onto an occupied point
- Group and liberty detection: connected stones are flood-filled into groups, and each group's empty neighbouring points are collected as its liberties
- Capture: after a move, any opposing group left with no liberties is removed
- Ko rule: a move is rejected if it would repeat the previous position by replaying the same point, using snapshots of the last two board states
- Each move returns a result with a success flag, the resulting board, the next player and, on failure, an error message such as `The player WHITE has violated the Ko rule.`

## Tech stack

- Rust, 2021 edition
- Standard library only in the code; `either` and `lazy_static` are listed in `Cargo.toml` but not used

## Getting started

Requires a Rust toolchain (stable).

```sh
cargo build
cargo test
```

`cargo run` builds and runs the binary, but it does nothing yet.

## Project structure

```
src/
├── main.rs                         # empty entry point
├── domain/
│   ├── go_board.rs                 # GoBoard, GoPlayer and board state; place, remove, clear
│   ├── go_game.rs                  # GoGame: current player, board, two previous snapshots
│   └── util/
│       └── group_liberties_aggregator.rs   # finds groups and their liberties
└── use_cases/
    └── place_piece/
        ├── place_piece_input.rs      # row, column and player
        ├── place_piece_output.rs     # success, board, next player, error message
        ├── place_piece_requester.rs  # PlacePieceRequester trait
        └── place_piece_interactor.rs # implementation for GoGame: turns, capture, ko
```

## Tests

`cargo test` runs unit tests for:

- board creation, placing, removing and clearing stones
- grouping and liberty counting on small boards
- the place-piece use case: a valid move, the wrong player moving, an occupied point, capturing a group and a ko violation

## Status and limitations

This is an early-stage engine, last worked on in 2022. Not yet implemented:

- any interface (command line, graphical or network) for playing a game
- suicide prevention: a move that leaves the player's own group with no liberties is allowed
- passing, resigning, end-of-game detection and scoring
- bounds checking: a move outside the board panics rather than returning an error
