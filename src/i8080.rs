
use ggez::{Context, GameResult, graphics::{self, Color}, input::keyboard::KeyCode};
use crate::{bus::Bus, cpu::cpu::Cpu, memory::{Memory}};

pub struct I8080 {
    memory: Memory,
    bus: Bus,
    cpu: Cpu,
}

impl I8080 {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            bus: Bus::init(),
            cpu: Cpu::new(),
        }
    }
    pub fn load_rom(&mut self, rom_file: &str) {
        let rom_data = std::fs::read(rom_file).unwrap();
        self.memory.init(rom_data[0..0x2000].try_into().unwrap());
    }

    const CYCLES_PER_FRAME:u64 = 4_000_000 / 60;

    pub fn update(&mut self) {
        // Run ~33000 cycles per frame (2MHz CPU at 60fps)
        for _ in 0..16_667 {
            self.cpu.run_step(&mut self.memory, &mut self.bus);
        }
        self.cpu.send_interrupt(0xCF);

        for _ in 0..16_667 {
            self.cpu.run_step(&mut self.memory, &mut self.bus);
        }
        self.cpu.send_interrupt(0xD7);
    }

    fn insert_coin(&mut self) {
        self.bus.port_1 = self.bus.port_1 | 0x01;
    }

    fn player_1(&mut self) {
        self.bus.port_1 = self.bus.port_1 | 0x02;
    }

    fn left(&mut self) {
        self.bus.port_1 = self.bus.port_1 | 0x20;
    }

    fn right(&mut self) {
        self.bus.port_1 = self.bus.port_1 | 0x40;
    }
    fn fire(&mut self) {
        self.bus.port_1 = self.bus.port_1 | 0x10;
    }
}


impl ggez::event::EventHandler for I8080 {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let k_ctx = &ctx.keyboard;
        if k_ctx.is_key_pressed(KeyCode::Key0) { self.insert_coin(); }
        if k_ctx.is_key_pressed(KeyCode::Key1) { self.player_1(); }
        if k_ctx.is_key_pressed(KeyCode::Key2) { /* player 2 */ }
        if k_ctx.is_key_pressed(KeyCode::J) { self.left(); }
        if k_ctx.is_key_pressed(KeyCode::L) { self.right(); }
        if k_ctx.is_key_pressed(KeyCode::A) { self.fire(); }

        self.update();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Width = 224, Height = 256.
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        let pixel_size = 2.0;

        for addr in 0x2400..0x3FFF {
            let byte = self.memory.read_byte(addr);
            if byte == 0 {
                continue; // Skip empty bytes for performance
            }

            for bit in 0..8 {
                if byte & (1 << bit) != 0 {
                    // Calculate original position in the 256x224 frame buffer
                    // byte_idx / 32 = row (0..223), byte_idx % 32 = column of bytes (0..31)
                    let row = (addr - 0x2400) / 32; // 0..223 (this is the X in rotated view)
                    let col = (addr - 0x2400) % 32; // 0..31
                    let original_x = col * 8 + bit; // 0..255 (pixel X before rotation)
                    let original_y = row; // 0..223 (pixel Y before rotation)

                    // Rotate 90 degrees counter-clockwise for the cabinet display
                    // CCW rotation: (x, y) -> (y, maxX - x)
                    // maxX = 255, so new position is (original_y, 255 - original_x)
                    let screen_x = original_y; // 0..223
                    let screen_y = 255 - original_x; // 0..255

                    // Apply pixel scaling
                    let x = screen_x as f32 * pixel_size;
                    let y = screen_y as f32 * pixel_size;

                    let r1 = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(x, y, pixel_size, pixel_size),
                        Color::WHITE,
                    );

                    if let Ok(rect) = r1 {
                        canvas.draw(&rect, graphics::DrawParam::new());
                    }
                }
            }
        }

        canvas.finish(ctx)
    }
}
