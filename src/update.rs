use sokoban::{Context, noop, clear_terminal};
use crossterm::{
  event::{Event},
};

pub fn check_victory() {
  // check if all boxes are on receptacles
  // if so, clear terminal and display victory message
}

pub fn update(mut context: Context) -> Context {
  // Check if terminal has been resized under the map size + 10
  // check_victory();
  for event in &context.events {
      match event {
          Event::Key(event) => match event.code {
              crossterm::event::KeyCode::Char('q') => {
                  context.terminal = clear_terminal(context.terminal);
                  std::process::exit(0);
              }
              crossterm::event::KeyCode::Up => {
                  context.map = context.player.move_up_char(context.map, context.map_offset)
              }
              crossterm::event::KeyCode::Down => {
                  context.map = context.player.move_down_char(context.map, context.map_offset)
              }
              crossterm::event::KeyCode::Left => {
                  context.map = context.player.move_left_char(context.map, context.map_offset)
              }
              crossterm::event::KeyCode::Right => {
                  context.map = context.player.move_right_char(context.map, context.map_offset)
              }
              _ => noop(),
          },
          _ => noop(),
      }
  }
  context.events.clear();
  context
}
