use crossterm::event::Event;
use sokoban::{clear_terminal, noop, Context, Scene};

pub fn check_victory(map: &Vec<char>, offset: usize) -> bool {
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
    players == 1 && boxes == 0 && goals == 0
}

pub fn update(mut context: Context) -> Context {
    // Check if terminal has been resized under the map size (+ 10 ?)
    for event in &context.events {
        match event {
            Event::Key(event) => match event.code {
                crossterm::event::KeyCode::Char('q') => {
                    context.terminal = clear_terminal(context.terminal);
                    std::process::exit(0);
                }
                crossterm::event::KeyCode::Up => {
                    if context.current_scene == Scene::Game {
                        context.map = context.player.move_up_char(context.map, context.map_offset)
                    }
                }
                crossterm::event::KeyCode::Down => {
                    if context.current_scene == Scene::Game {
                        context.map = context
                            .player
                            .move_down_char(context.map, context.map_offset)
                    }
                }
                crossterm::event::KeyCode::Left => {
                    if context.current_scene == Scene::Game {
                        context.map = context
                            .player
                            .move_left_char(context.map, context.map_offset)
                    }
                }
                crossterm::event::KeyCode::Right => {
                    if context.current_scene == Scene::Game {
                        context.map = context
                            .player
                            .move_right_char(context.map, context.map_offset)
                    }
                }
                _ => noop(),
            },
            _ => noop(),
        }
    }
    if check_victory(&context.map, context.map_offset) == true && context.victory != true {
        context.victory = true;
        context.events.clear();
        context.current_scene = Scene::Victory;
        return context;
    }
    context.events.clear();
    context
}
