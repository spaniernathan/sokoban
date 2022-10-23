use sokoban::{Context};
use core::panic;
use tui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
};

pub fn render(mut context: Context) -> Context {
    // check for errors first then display accordingly
    context.terminal = match context.terminal.draw(|f| {
      let mut xx = 1;
      for x in &context.map {
          let mut yy = 1;
          for y in x {
              let rect = Rect::new(yy, xx, 2, 1);
              let color = match y {
                  ' ' => Color::White,
                  'P' => Color::Green,
                  'B' => Color::Yellow,
                  'X' => Color::Red,
                  '#' => Color::Black,
                  _ => Color::White,
              };
              let block = Block::default().style(Style::default().bg(color).fg(color));
              f.render_widget(block, rect);
              yy += 2;
          }
          xx += 1;
      }
      let block = Block::default().title("Sokoban").borders(Borders::ALL);
      f.render_widget(block, context.terminal_size);
  }) {
      Ok(_) => context.terminal,
      Err(e) => panic!("{}", e),
  };
  context
}
