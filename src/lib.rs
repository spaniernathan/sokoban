use crossterm::event::Event;
use std::io;
use tui::{backend::CrosstermBackend, layout::Rect, Terminal};

pub enum Error {
    TerminalSizeError,
}

#[derive(Debug, PartialEq)]
pub enum Scene {
    MapSelection,
    Game,
    Victory,
}

pub struct Context {
    pub terminal: Terminal<CrosstermBackend<io::Stdout>>,
    pub terminal_size: Rect,
    pub events: Vec<Event>,
    pub error: Option<Error>,
    pub map: Vec<char>,
    pub map_offset: usize,
    pub player: Player,
    pub current_scene: Scene,
    pub victory : bool,
}

#[derive(Debug)]
pub struct Player {
    x: usize,
    y: usize,
    is_on_receptacle: bool,
    was_on_receptacle: bool,
}

impl Player {
    pub fn new(x: usize, y: usize) -> Player {
        Player {
            x,
            y,
            is_on_receptacle: false,
            was_on_receptacle: false,
        }
    }

    fn can_move(&mut self, next_tile: char, next_next_tile: char) -> (bool, bool, bool) {
        let mut player_moved = false;
        let mut box_moved = false;
        let mut box_on_goal = false;
        if self.is_on_receptacle {
            self.was_on_receptacle = true;
            self.is_on_receptacle = false;
        }
        if next_tile == '#' {
            return (player_moved, box_moved, box_on_goal);
        }
        if next_tile == ' ' {
            player_moved = true;
        } else if next_tile == 'X' {
            player_moved = true;
            self.is_on_receptacle = true;
        } else if next_tile == 'B' && (next_next_tile == 'X' || next_next_tile == ' ') {
          if next_next_tile == ' ' {
            player_moved = true;
            box_moved = true;
          } else if next_next_tile == 'X' {
            player_moved = true;
            box_moved = true;
            box_on_goal = true
          }
        }
        (player_moved, box_moved, box_on_goal)
    }

    pub fn move_up_char(&mut self, mut map: Vec<char>, offset: usize) -> Vec<char> {
        let next_tile = match map.get((self.y - 1) * offset + self.x) {
            Some(tile) => *tile,
            None => '#',
        };
        let next_next_tile = match map.get((self.y - 2) * offset + self.x) {
            Some(tile) => *tile,
            None => '#',
        };
        if self.y > 0 {
            let (player_moved, box_moved, box_on_goal) = self.can_move(next_tile, next_next_tile);
            if player_moved {
                map[(self.y - 1) * offset + self.x] = 'P';
                if self.was_on_receptacle {
                    map[self.y * offset + self.x] = 'X';
                    self.was_on_receptacle = false;
                } else {
                    map[self.y * offset + self.x] = ' ';
                }
                if box_moved {
                    map[(self.y - 2) * offset + self.x] = 'B';
                    if box_on_goal {
                      map[(self.y - 2) * offset + self.x] = 'O';
                    }
                }
                self.y -= 1;
            }
        }
        map
    }

    pub fn move_down_char(&mut self, mut map: Vec<char>, offset: usize) -> Vec<char> {
        let next_tile = match map.get((self.y + 1) * offset + self.x) {
            Some(tile) => *tile,
            None => '#',
        };
        let next_next_tile = match map.get((self.y + 2) * offset + self.x) {
            Some(tile) => *tile,
            None => '#',
        };
        if self.y < map.len() - 1 {
            let (player_moved, box_moved, box_on_goal) = self.can_move(next_tile, next_next_tile);
            if player_moved {
                map[(self.y + 1) * offset + self.x] = 'P';
                if self.was_on_receptacle {
                    map[self.y * offset + self.x] = 'X';
                    self.was_on_receptacle = false;
                } else {
                    map[self.y * offset + self.x] = ' ';
                }
                if box_moved {
                    map[(self.y + 2) * offset + self.x] = 'B';
                    if box_on_goal {
                      map[(self.y + 2) * offset + self.x] = 'O';
                    }
                }
                self.y += 1;
            }
        }
        map
    }

    pub fn move_left_char(&mut self, mut map: Vec<char>, offset: usize) -> Vec<char> {
        let next_tile = match map.get(self.y * offset + self.x - 1) {
            Some(tile) => *tile,
            None => '#',
        };
        let next_next_tile = match map.get(self.y * offset + self.x - 2) {
            Some(tile) => *tile,
            None => '#',
        };
        if self.x > 0 {
            let (player_moved, box_moved, box_on_goal) = self.can_move(next_tile, next_next_tile);
            if player_moved {
                map[self.y * offset + self.x - 1] = 'P';
                if self.was_on_receptacle {
                    map[self.y * offset + self.x] = 'X';
                    self.was_on_receptacle = false;
                } else {
                    map[self.y * offset + self.x] = ' ';
                }
                if box_moved {
                    map[self.y * offset + self.x - 2] = 'B';
                    if box_on_goal {
                      map[self.y * offset + self.x - 2] = 'O';
                    }
                }
                self.x -= 1;
            }
        }
        map
    }

    pub fn move_right_char(&mut self, mut map: Vec<char>, offset: usize) -> Vec<char> {
        let next_tile = match map.get(self.y * offset + self.x + 1) {
            Some(tile) => *tile,
            None => '#',
        };
        let next_next_tile = match map.get(self.y * offset + self.x + 2) {
            Some(tile) => *tile,
            None => '#',
        };
        if self.x < offset - 1 {
            let (player_moved, box_moved, box_on_goal) = self.can_move(next_tile, next_next_tile);
            if player_moved {
                map[self.y * offset + self.x + 1] = 'P';
                if self.was_on_receptacle {
                    map[self.y * offset + self.x] = 'X';
                    self.was_on_receptacle = false;
                } else {
                    map[self.y * offset + self.x] = ' ';
                }
                if box_moved {
                    map[self.y * offset + self.x + 2] = 'B';
                    if box_on_goal {
                      map[self.y * offset + self.x + 2] = 'O';
                    }
                }
                self.x += 1;
            }
        }
        map
    }
}

pub fn noop() {}

pub fn clear_terminal(
    mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
) -> Terminal<CrosstermBackend<io::Stdout>> {
    terminal = match terminal.clear() {
        Ok(_) => terminal,
        Err(e) => panic!("Failed to clear terminal: {}", e),
    };
    terminal
}

pub fn hide_terminal_cursor(
    mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
) -> Terminal<CrosstermBackend<io::Stdout>> {
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
