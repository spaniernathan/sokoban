use core::panic;
use sokoban::{Context, Scene};
use tui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
};

fn render_victory_scene(mut context: Context) -> Context {
    context
}

fn render_map_selection(mut context: Context) -> Context {
    context
}

fn render_game(mut context: Context) -> Context {
    context.terminal = match context.terminal.draw(|f| {
        for y in 0..context.map_offset {
            for x in 0..(context.map.len() / context.map_offset) {
                let rect = Rect::new((x + 1) as u16, (y + 1) as u16, 2, 1);
                let color = match context.map[y * context.map_offset + x] {
                    ' ' => Color::White,
                    'P' => Color::Green,
                    'B' => Color::Yellow,
                    'X' => Color::Red,
                    '#' => Color::Black,
                    _ => Color::White,
                };
                let block = Block::default().style(Style::default().bg(color).fg(color));
                f.render_widget(block, rect);
            }
        }
        let block = Block::default().title("Sokoban").borders(Borders::ALL);
        f.render_widget(block, context.terminal_size);
    }) {
        Ok(_) => context.terminal,
        Err(e) => panic!("{}", e),
    };
    context
}

pub fn render(mut context: Context) -> Context {
    // check for errors first then display accordingly
    context = match &context.current_scene {
        Scene::MapSelection => render_map_selection(context),
        Scene::Game => render_game(context),
        Scene::Victory => render_victory_scene(context),
    };
    context
}
