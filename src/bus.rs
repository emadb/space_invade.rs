use rusty_audio::Audio;

pub struct Bus {
    pub shift_reg: u16,
    pub shift_offset: u8,
    pub port_1: u8,
    pub port_2: u8,
    audio: Audio,
    prev_port_3: u8,
    prev_port_5: u8,
}

pub enum InputPort {
    Port1, Port2
}

/*

Ports:
     Read 1
     BIT 0   coin (0 when active)
         1   P2 start button
         2   P1 start button
         3   ?
         4   P1 shoot button
         5   P1 joystick left
         6   P1 joystick right
         7   ?

     Read 2
     BIT 0,1 dipswitch number of lives (0:3,1:4,2:5,3:6)
         2   tilt 'button'
         3   dipswitch bonus life at 1:1000,0:1500
         4   P2 shoot button
         5   P2 joystick left
         6   P2 joystick right
         7   dipswitch coin info 1:off,0:on

     Read 3      shift register result

     Write 2     shift register result offset (bits 0,1,2)
     Write 3     sound related
     Write 4     fill shift register
     Write 5     sound related
     Write 6     strange 'debug' port? eg. it writes to this port when
             it writes text to the screen (0=a,1=b,2=c, etc)

     (write ports 3,5,6 can be left unemulated, read port 1=$01 and 2=$00
     will make the game run, but but only in attract mode)
 */

impl Bus {
    pub fn new() -> Self {
        let mut audio = Audio::new();
        audio.add("exp", "./roms/explosion.wav");
        audio.add("fi1", "./roms/fastinvader1.wav");
        audio.add("fi2", "./roms/fastinvader2.wav");
        audio.add("fi3", "./roms/fastinvader3.wav");
        audio.add("fi4", "./roms/fastinvader4.wav");
        audio.add("kill", "./roms/invaderkilled.wav");
        audio.add("shoot", "./roms/shoot.wav");
        audio.add("ufohi", "./roms/ufo_highpitch.wav");
        audio.add("ufolow", "./roms/ufo_lowpitch.wav");
        Bus{
            port_1: 0,
            port_2: 0,
            shift_reg: 0,
            shift_offset: 0,
            audio,
            prev_port_3: 0,
            prev_port_5: 0,
        }
    }

    pub fn write_port(&mut self, port: u8, data: u8) {
        match port {
            2 => { self.shift_offset = data & 0x07 },
            3 => {
                // Bit	Description
                // 0	Spaceship (looped) sound
                // 1	Shot sound
                // 2	Base (your ship) hit sound
                // 3	Invader hit sound
                // 4	Extended play sound

                let trigger_sound = (data ^ self.prev_port_3) & data;

                if trigger_sound & 0x01 != 0 { /* UFO sound */ }
                if trigger_sound & 0x02 != 0 { self.audio.play("shoot"); }
                if trigger_sound & 0x04 != 0 { self.audio.play("exp"); }
                if trigger_sound & 0x08 != 0 { self.audio.play("kill"); }
                if trigger_sound & 0x10 != 0 { /* Extended play sound */ }

                self.prev_port_3 = data;
            },
            4 => {
                self.shift_reg >>= 8;
                self.shift_reg |= (data as u16) << 8;
            }
            5 => {
                // 0	Invaders walk 1 sound
                // 1	Invaders walk 2 sound
                // 2	Invaders walk 3 sound
                // 3	Invaders walk 4 sound
                // 4	Spaceship hit sound
                // 5	Amplifier enabled/disabled

                let trigger_sound = (data ^ self.prev_port_5) & data;

                if trigger_sound & 0x01 != 0 { self.audio.play("fi1"); }
                if trigger_sound & 0x02 != 0 { self.audio.play("fi2"); }
                if trigger_sound & 0x04 != 0 { self.audio.play("fi3"); }
                if trigger_sound & 0x08 != 0 { self.audio.play("fi4"); }
                if trigger_sound & 0x10 != 0 { self.audio.play("exp"); }

                self.prev_port_5 = data;
            },
            6 => { /* debug */},
            _ => panic!("port does not exists: {}", port)
        }
    }
    pub fn read_port(&mut self, port: u8) -> u8 {
        match port {
            1 => { self.port_1 },
            2 => { self.port_2 },
            3 => { ((self.shift_reg << self.shift_offset) >> 8) as u8},
            _ => panic!("port does not exists: {}", port)
        }
    }

     pub fn set_bit(&mut self, port: InputPort, bit: u8) {
       let mask = 1 << bit;
        match port {
            InputPort::Port1 => self.port_1 |= mask,
            InputPort::Port2 => self.port_2 |= mask,
        }
    }

    pub fn unset_bit(&mut self, port: InputPort, bit: u8) {
        let mask = 1 << bit;
        match port {
            InputPort::Port1 => self.port_1 &= !mask,
            InputPort::Port2 => self.port_2 &= !mask,
        }
    }
}