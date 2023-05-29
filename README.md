# Chip8 Emulator

A emulator (actually interpreter) of the chip-8 programming language for the terminal and written in Rust.
This project uses [crossterm](https://github.com/crossterm-rs/crossterm) for the screen and for the events.

---

## Caveats

- The input system relies on the [kitty's keyboard protocol](https://sw.kovidgoyal.net/kitty/keyboard-protocol/), if you terminal does not support it, you may experience issues with the input.
- Currently it doesn't have a functional sound system, so you will hear nothing.

---

## Getting Started

In order to build the emulator, you will need to have installed [git](https://git-scm.com/) and [rust](https://www.rust-lang.org/).

---

## Building

To clone the project and build it, use the following commands:
```bash
git clone https://github.com/seg-mx/chip8
cd chip8
cargo build --release
```

After doing that, the binary will be in `./target/release/chip8`.

---

## Executing A Rom

You need to execute chip8 command as:
```bash
chip8 path/to/your/rom.ch8
```

The rom is running too fast or too slow? Change the tick rate by adding a extra parameter (Default is 250, use 0 for fast as possible and 2000 for a 2 seconds delay).
```bash
chip8 path/to/your/rom.ch8 15
```

---

## Using The Keypad

The chip-8 keypad layout looks like this:

|   |   |   |   |
| - | - | - | - |
| 1 | 2 | 3 | c |
| 4 | 5 | 6 | d |
| 7 | 8 | 9 | e |
| a | 0 | b | f |

However, for modern computers the most common layout (and the one this emulator uses) is:

|   |   |   |   |
| - | - | - | - |
| 1 | 2 | 3 | 4 |
| q | w | e | r |
| a | s | d | f |
| z | x | c | v |

So, if a game asks you to press key `d` you should press `r` instead.

---

## Quitting

Like in most terminal applications, to exit the emulator, you have to press `Ctrl + C`.

---

## Troubleshooting

If you experience any problems with the emulator, [open an issue](https://github.com/seg-mx/chip8/issues/new) explaining the situation and I'll try my best to help.
