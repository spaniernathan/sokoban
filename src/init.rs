use sokoban::{
  Context,
  Player,
  Scene,
  clear_terminal,
  hide_terminal_cursor,
  get_terminal_size,
};
use core::panic;
use crossterm::{
    terminal::enable_raw_mode,
};
use std::io;
use tui::{
    backend::CrosstermBackend,
    Terminal,
};

fn check_map(map: &Vec<char>, offset: usize) -> bool {
    let mut boxes = 0;
    let mut goals = 0;
    let mut players = 0;
    // Not sure the loop is right
    for y in 0..offset {
      for x in 0..map.len() / offset {
        let index = y * offset + x;
        if map[index] == 'B' {
          boxes += 1;
        } else if map[index] == 'X' {
          goals += 1;
        } else if map[index] == 'P' {
          players += 1;
        }
      }
    }
    players == 1 && boxes > 1 && goals > 1 && boxes == goals
}

fn load_maps() -> (Vec<char>, usize) {
    let map_string = match std::fs::read_to_string(
        "/Users/spaniernathan/Work/Perso/sokoban/src/maps/level1.map",
    ) {
        Ok(s) => s,
        Err(e) => panic!("Failed to read map file: {}", e),
    };
    let offset = match map_string.find("\n") {
        Some(offset) => offset,
        None => panic!("Invalid map file"),
    };
    let mut map: Vec<char> = Vec::new();
    map_string.chars().for_each(|c| if c != '\n' { map.push(c) });
    if !check_map(&map, offset) {
        panic!("Invalid map file");
    }
    (map, offset)
}

pub fn init() -> Context {
  let stdout = io::stdout();
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = match Terminal::new(backend) {
      Ok(t) => t,
      Err(e) => panic!("Failed to create terminal: {}", e),
  };
  match enable_raw_mode() {
      Ok(_) => (),
      Err(e) => panic!("Failed to enable raw mode: {}", e),
  }
  terminal = hide_terminal_cursor(terminal);
  let size = get_terminal_size(&terminal);
  terminal = clear_terminal(terminal);

  let (map, map_offset) = load_maps();

  let mut player_x = 0;
  let mut player_y = 0;

  for y in 0..map_offset {
    for x in 0..(map.len() / map_offset) {
      if map[y * map_offset + x] == 'P' {
        player_x = x;
        player_y = y;
      }
    }
  }

  let context = Context {
      terminal,
      terminal_size: size,
      events: Vec::new(),
      error: None,
      map_offset,
      map,
      player: Player::new(player_y, player_x),
      current_scene: Scene::Game,
  };
  context
}
