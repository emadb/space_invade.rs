pub struct Bus {
    pub shift_reg: u16,
    pub shift_offset: u8,
    pub port_1: u8,
    pub port_2: u8
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
    pub fn init() -> Self {
        Bus{
            port_1: 0,
            port_2: 0,
            shift_reg: 0,
            shift_offset: 0
        }
    }

    pub fn write_port(&mut self, port: u8, data: u8) {
        match port {
            2 => { self.shift_offset = data & 0x07 },
            3 => { /* sound */},
            4 => {
                // Writing to port 4 shifts MSB into LSB, and the new value into MSB
                self.shift_reg >>= 8;
                self.shift_reg |= (data as u16) << 8;
            }
            5 => { /* sound */},
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
}