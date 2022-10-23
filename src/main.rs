use std::io;
mod init;
mod input;
mod update;
mod render;

fn main() -> Result<(), io::Error> {
    let mut context = crate::init::init();
    loop {
        context = crate::input::input(context);
        context = crate::update::update(context);
        context = crate::render::render(context);
    }
}
