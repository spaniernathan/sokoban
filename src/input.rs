use sokoban::{Context, noop};
use std::time::Duration;
use crossterm::{
  event::{poll, read, Event},

};
use tui::{
  layout::Rect,
};

pub fn input(mut context: Context) -> Context {
  if poll(Duration::from_millis(200)).expect("Failed to poll") {
      match read() {
          Ok(Event::FocusGained) => noop(),
          Ok(Event::FocusLost) => noop(),
          Ok(Event::Key(event)) => context.events.push(Event::Key(event)),
          Ok(Event::Mouse(_)) => noop(),
          Ok(Event::Resize(width, height)) => {
              context.terminal_size = Rect::new(context.terminal_size.x, context.terminal_size.y, width, height)
          }
          Ok(Event::Paste(_)) => noop(),
          Err(err) => println!("Error: {:?}", err),
      }
  }
  context
}
