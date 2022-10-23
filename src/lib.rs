use std::io;
use crossterm::{
    event::{Event},
};
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    Terminal,
};

pub enum Error {
  TerminalSizeError,
}

pub struct Context {
  pub terminal: Terminal<CrosstermBackend<io::Stdout>>,
  pub terminal_size: Rect,
  pub events: Vec<Event>,
  pub error: Option<Error>,
  pub map: Vec<Vec<char>>,
  pub char_map: Vec<char>,
  pub map_offset: usize,
  pub player: Player,
}

#[derive(Debug)]
pub struct Player {
  x: u16,
  y: u16,
  is_on_receptacle: bool,
  was_on_receptacle: bool,
}

impl Player {
  pub fn new(x: u16, y: u16) -> Player {
      Player {
          x,
          y,
          is_on_receptacle: false,
          was_on_receptacle: false,
      }
  }

  fn can_move(&mut self, next_tile: char, next_next_tile: char) -> (bool, bool) {
      let mut player_moved = false;
      let mut box_moved = false;
      if self.is_on_receptacle {
          self.was_on_receptacle = true;
          self.is_on_receptacle = false;
      }
      if next_tile == '#' {
          return (player_moved, box_moved);
      }
      if next_tile == ' ' {
          player_moved = true;
      } else if next_tile == 'X' {
          player_moved = true;
          self.is_on_receptacle = true;
      } else if next_tile == 'B' && (next_next_tile == 'X' || next_next_tile == ' ') {
          player_moved = true;
          box_moved = true;
      }
      // TODO: handle when box is on receptacle
      (player_moved, box_moved)
  }

  pub fn move_up(&mut self, mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
      let next_tile = map[self.y as usize - 1][self.x as usize];
      let next_next_tile = match map.get(self.y as usize - 2) {
          Some(row) => row[self.x as usize],
          None => '#',
      };
      if self.y > 0 {
          let (player_moved, box_moved) = self.can_move(next_tile, next_next_tile);
          if player_moved {
              map[self.y as usize - 1][self.x as usize] = 'P';
              if self.was_on_receptacle {
                  map[self.y as usize][self.x as usize] = 'X';
                  self.was_on_receptacle = false;
              } else {
                  map[self.y as usize][self.x as usize] = ' ';
              }
              if box_moved {
                  map[self.y as usize - 2][self.x as usize] = 'B';
              }
              self.y -= 1;
          }
      }
      map
  }

  pub fn move_down(&mut self, mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
      let next_tile = map[self.y as usize + 1][self.x as usize];
      let next_next_tile = match map.get(self.y as usize + 2) {
          Some(row) => row[self.x as usize],
          None => '#',
      };
      if self.y < map.len() as u16 - 1 {
          let (player_moved, box_moved) = self.can_move(next_tile, next_next_tile);
          if player_moved {
              map[self.y as usize + 1][self.x as usize] = 'P';
              if self.was_on_receptacle {
                  map[self.y as usize][self.x as usize] = 'X';
                  self.was_on_receptacle = false;
              } else {
                  map[self.y as usize][self.x as usize] = ' ';
              }
              if box_moved {
                  map[self.y as usize + 2][self.x as usize] = 'B';
              }
              self.y += 1;
          }
      }
      map
  }

  pub fn move_left(&mut self, mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
      let next_tile = map[self.y as usize][self.x as usize - 1];
      let next_next_tile = match map[self.y as usize].get(self.x as usize - 2) {
          Some(tile) => *tile,
          None => '#',
      };
      if self.x > 0 {
          let (player_moved, box_moved) = self.can_move(next_tile, next_next_tile);
          if player_moved {
              map[self.y as usize][self.x as usize - 1] = 'P';
              if self.was_on_receptacle {
                  map[self.y as usize][self.x as usize] = 'X';
                  self.was_on_receptacle = false;
              } else {
                  map[self.y as usize][self.x as usize] = ' ';
              }
              if box_moved {
                  map[self.y as usize][self.x as usize - 2] = 'B';
              }
              self.x -= 1;
          }
      }
      map
  }

  pub fn move_right(&mut self, mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
      let next_tile = map[self.y as usize][self.x as usize + 1];
      let next_next_tile = match map[self.y as usize].get(self.x as usize + 2) {
          Some(tile) => *tile,
          None => '#',
      };
      if self.x < map[self.y as usize].len() as u16 - 1 {
          let (player_moved, box_moved) = self.can_move(next_tile, next_next_tile);
          if player_moved {
              map[self.y as usize][self.x as usize + 1] = 'P';
              if self.was_on_receptacle {
                  map[self.y as usize][self.x as usize] = 'X';
                  self.was_on_receptacle = false;
              } else {
                  map[self.y as usize][self.x as usize] = ' ';
              }
              if box_moved {
                  map[self.y as usize][self.x as usize + 2] = 'B';
              }
              self.x += 1;
          }
      }
      map
  }
}

pub fn noop() {}

pub fn clear_terminal(mut terminal: Terminal<CrosstermBackend<io::Stdout>>) -> Terminal<CrosstermBackend<io::Stdout>> {
    terminal = match terminal.clear() {
        Ok(_) => terminal,
        Err(e) => panic!("Failed to clear terminal: {}", e),
    };
    terminal
}

pub fn hide_terminal_cursor(mut terminal: Terminal<CrosstermBackend<io::Stdout>>) -> Terminal<CrosstermBackend<io::Stdout>> {
    terminal = match terminal.hide_cursor() {
        Ok(_) => terminal,
        Err(e) => panic!("Failed to hide cursor: {}", e),
    };
    terminal
}

pub fn get_terminal_size(terminal: &Terminal<CrosstermBackend<io::Stdout>>) -> Rect {
  let size = match terminal.size() {
    Ok(s) => s,
    Err(e) => panic!("Failed to get terminal size: {}", e),
  };
  size
}
