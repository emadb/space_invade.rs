# Space Invaders Emulator

A Space Invaders arcade game emulator written in Rust, featuring a complete Intel 8080 CPU emulation.

## Description

This project is a faithful emulation of the classic 1978 Space Invaders arcade game. It implements a complete Intel 8080 microprocessor emulator that runs the original Space Invaders ROM. The emulator includes:

- Full Intel 8080 CPU instruction set implementation
- Accurate cycle timing for authentic gameplay
- Graphics rendering using the ggez game framework
- Original arcade cabinet display rotation (90° counter-clockwise)
- Input handling for player controls

## How to Run

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- Space Invaders ROM file (place in `roms/invaders.rom`)

### Building and Running

1. Clone the repository:
```bash
git clone https://github.com/emadb/space_invade.rs.git
cd space_invade.rs
```

2. Ensure you have the Space Invaders ROM file in the `roms` directory:
```
roms/invaders.rom
```

3. Build and run the emulator:
```bash
cargo run --release
```

## Game Controls

| Key | Action |
|-----|--------|
| `0` | Insert Coin |
| `1` | Start 1 Player Game |
| `J` | Move Left |
| `L` | Move Right |
| `A` | Fire |

## Technical Details

The emulator implements the Intel 8080 microprocessor with:
- 8-bit data bus and 16-bit address bus
- Full instruction set including arithmetic, logical, and control flow operations
- Interrupt handling for display refresh timing
- Memory-mapped video RAM (0x2400-0x3FFF)
- I/O ports for input handling

## References

- [Intel 8080 CPU Documentation](https://en.wikipedia.org/wiki/Intel_8080)
- [Space Invaders Hardware](http://computerarcheology.com/Arcade/SpaceInvaders/)
- [ggez Game Framework](https://ggez.rs/)
- Additional technical documentation available in `.github/` directory

## License

This is an educational project for learning about CPU emulation and classic arcade hardware.
