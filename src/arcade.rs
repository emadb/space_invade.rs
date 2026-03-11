use crate::{bus::Bus, cpu::cpu::Cpu, memory::Memory};
use ggez::{
    Context, GameResult,
    graphics::{self, Color, DrawParam, InstanceArray},
    input::keyboard::{KeyCode, KeyInput},
};

pub struct Arcade {
    memory: Memory,
    bus: Bus,
    cpu: Cpu,
    pixels: InstanceArray,
}

impl Arcade {
    pub fn new(ctx: &Context) -> Self {
        let image = graphics::Image::from_color(&ctx.gfx, 1, 1, Some(Color::WHITE));
        let pixels = graphics::InstanceArray::new(ctx, image);

        Self {
            memory: Memory::new(),
            bus: Bus::init(),
            cpu: Cpu::new(),
            pixels: pixels,
        }
    }

    pub fn load_rom(&mut self, rom_file: &str) {
        let rom_data = std::fs::read(rom_file).unwrap();
        self.memory.init_rom(rom_data);
    }

    pub fn update(&mut self) {

        let mut cycles: u64 = 0;
        // CPU 2MHz
        // Video 60Hz
        // 2M / 60 = 33k
        while cycles < 16_768 {
            cycles += self.cpu.run_step(&mut self.memory, &mut self.bus) as u64;
        }
        self.cpu.send_interrupt(0xCF); // Updates the score/lives display area

        while cycles < 33_536 {
            cycles += self.cpu.run_step(&mut self.memory, &mut self.bus) as u64;
        }
        self.cpu.send_interrupt(0xD7); // Updates game state and draws sprites

        // TODO: is there a better way?
        std::thread::sleep(std::time::Duration::from_millis(25));
    }

}

impl ggez::event::EventHandler for Arcade {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.update();

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> ggez::GameResult {
        match input.keycode {
            Some(KeyCode::Key0) => { self.bus.set_port_1_bit(0); }  // insert coin
            Some(KeyCode::Key1) => { self.bus.set_port_1_bit(2); }  // player 1
            Some(KeyCode::Key2) => { self.bus.set_port_1_bit(1); }  // player 2

            Some(KeyCode::J) => { self.bus.set_port_1_bit(5); } // P1 Left
            Some(KeyCode::L) => { self.bus.set_port_1_bit(6); } // P1 right
            Some(KeyCode::X) => { self.bus.set_port_1_bit(4); } // P1 Fire

            Some(KeyCode::A) => { self.bus.set_port_2_bit(5); } // P2 Left
            Some(KeyCode::D) => { self.bus.set_port_2_bit(6); } // P2 Right
            Some(KeyCode::V) => { self.bus.set_port_2_bit(4); } // P2 Fire
            None => {}
            _ => {}
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> ggez::GameResult {
        match input.keycode {
            Some(KeyCode::Key0) => { self.bus.unset_port_1_bit(0); }
            Some(KeyCode::Key1) => { self.bus.unset_port_1_bit(2); }
            Some(KeyCode::Key2) => { self.bus.unset_port_1_bit(1); }

            Some(KeyCode::J) => { self.bus.unset_port_1_bit(5); }
            Some(KeyCode::L) => { self.bus.unset_port_1_bit(6); }
            Some(KeyCode::X) => { self.bus.unset_port_1_bit(4); }

            Some(KeyCode::A) => { self.bus.unset_port_2_bit(5); }
            Some(KeyCode::D) => { self.bus.unset_port_2_bit(6); }
            Some(KeyCode::V) => { self.bus.unset_port_2_bit(4); }

            None => {}
            _ => {}
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        self.pixels.clear();

        for addr in 0x2400..0x3FFF {
            let byte = self.memory.read_byte(addr);
            if byte == 0 {
                continue;
            }

            for bit in 0..8 {
                if byte & (1 << bit) != 0 {
                    let row = (addr - 0x2400) / 32;
                    let col = (addr - 0x2400) % 32;
                    let original_x = col * 8 + bit;
                    let original_y = row;

                    let screen_x = original_y;
                    let screen_y = 255 - original_x;

                    let x = screen_x as f32 * crate::PIXEL_FACTOR;
                    let y = screen_y as f32 * crate::PIXEL_FACTOR;

                    self.pixels.push(
                        DrawParam::default()
                            .scale([crate::PIXEL_FACTOR, crate::PIXEL_FACTOR])
                            .dest([x, y])
                            .color(Color::WHITE),
                    );
                }
            }
        }
        canvas.draw(&self.pixels, DrawParam::default());
        canvas.finish(ctx)
    }
}
