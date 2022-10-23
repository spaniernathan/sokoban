# Sokoban

This game uses [tui](https://crates.io/crates/tui) and [crossterm](https://crates.io/crates/crossterm) to render the game.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

## How to run

```sh
$ cargo run --release -- --tick-rate 200
```

## How to play

- Use the arrow keys to move the player (the Green tile)
- Push the boxes (the yellow tiles) into the goal tiles (the red tiles)

## Create your own level

- Create a new file in the `levels` directory
- Use the following characters:
  - `#` for walls
  - `X` for goals
  - `P` for the player
  - `B` for boxes
  - ` ` for empty tiles
- The level must be sourrounded by walls
- The player must be in the level
- The level must have at least one box and one goal
- The level must have an equal number of boxes and goals
- The level must be rectangular

## TODOs / Fixes

- [ ] Use an int array + offset instead of a `Vec<Vec<char>>` to represent the level
- [ ] Handle when box is on receptacle
- [ ] Map selection
- [ ] Victory screen
- [ ] Check: The level must be rectangular
- [ ] Check: The level must be sourrounded by walls
