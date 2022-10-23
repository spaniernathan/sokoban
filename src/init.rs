use sokoban::{Context, Player, clear_terminal, hide_terminal_cursor, get_terminal_size};
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
    for row in 0..map.len() {
      for col in 0..offset {
        let index = row * offset + col;
        if map[index] == 'B' {
          boxes += 1;
        } else if map[index] == 'X' {
          goals += 1;
        } else if map[index] == 'P' {
          players += 1;
        }
      }
    }
    players != 1 && boxes < 1 && goals < 1 && boxes != goals
}

fn load_maps() -> Vec<char> {
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
    map
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

  let map_string = match std::fs::read_to_string(
      "/Users/spaniernathan/Work/Perso/sokoban/src/maps/level1.map",
  ) {
      Ok(s) => s,
      Err(e) => panic!("Failed to read map file: {}", e),
  };
  let mut map_vec: Vec<Vec<char>> = Vec::new();
  for line in map_string.lines() {
      let mut line_vec: Vec<char> = Vec::new();
      for c in line.chars() {
          line_vec.push(c);
      }
      map_vec.push(line_vec);
  }

  let player_x = map_vec.iter().position(|x| x.contains(&'P')).unwrap() as u16;
  let player_y = map_vec[player_x as usize]
      .iter()
      .position(|&x| x == 'P')
      .unwrap() as u16;

  let context = Context {
      terminal,
      terminal_size: size,
      events: Vec::new(),
      error: None,
      map: map_vec,
      map_offset: 0,
      char_map: Vec::new(),
      player: Player::new(player_y, player_x),
  };
  context
}
