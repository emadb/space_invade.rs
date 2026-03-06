pub mod bus;
pub mod cpu;
pub mod i8080;
pub mod memory;
pub mod stack;

use i8080::I8080;

pub const PIXEL_FACTOR: f32 = 5.0;

pub fn run(rom_file: &str) {
    let (ctx, event_loop) = ggez::ContextBuilder::new("Space Invade.rs", "ema")

        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(256.0 * PIXEL_FACTOR, 224.0 * PIXEL_FACTOR),
        )
        .build()
        .expect("could not create ggez context!");

    let mut i8080 = I8080::new(&ctx);
    i8080.load_rom(rom_file);

    ggez::event::run(ctx, event_loop, i8080);
}
