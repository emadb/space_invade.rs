pub mod bus;
pub mod cpu;
pub mod memory;
pub mod stack;
pub mod i8080;

use i8080::I8080;


pub fn run(rom_file: &str) {
    let (ctx, event_loop) = ggez::ContextBuilder::new("Space Invaders", "ema")
        .build()
        .expect("could not create ggez context!");

    let mut i8080 = I8080::new();
    i8080.load_rom(rom_file);

    ggez::event::run(ctx, event_loop, i8080);
}
