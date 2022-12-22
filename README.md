# rust-boy

A Gameboy emulator written in Rust.

<img src="https://i.imgur.com/0GDI7HD.png" width="500">
<img src="https://i.imgur.com/sAcRA67.png" width="500" >

## Features
- Sound
- Game Saves
- Support for game cart types: MBC1, MBC2 and MBC3
- Configurable color palette


## Todo
- [ ] Gameboy Color support (MBC5)

## Project Outline (generated by ChatGPT)

Here is a simple outline of how you might go about writing a Game Boy emulator in Rust:

1. Start by familiarizing yourself with the Game Boy hardware and how it works. You can find information about the Game Boy's architecture, memory layout, and instruction set in various online resources, such as the Pan Docs.

2. Choose a graphics library to use for rendering the Game Boy's screen. Some popular options for Rust include SDL2 and glium.

3. Implement the Game Boy's CPU by writing a Rust struct that represents the state of the CPU, including the registers, flags, and program counter. You will also need to write functions that simulate the behavior of each instruction in the Game Boy's instruction set.

4. Implement the Game Boy's memory by writing a Rust struct that represents the state of the memory, including the ROM, RAM, and I/O registers. You will also need to write functions to read and write to different regions of memory, such as the ROM, RAM, and I/O registers.

5. Implement the Game Boy's graphics by writing a Rust struct that represents the state of the graphics hardware, including the framebuffer and the tile and sprite data. You will also need to write functions to render the graphics to the screen using the graphics library you chose in step 2.

6. Implement the Game Boy's input by writing a Rust struct that represents the state of the input devices, such as the buttons and the D-pad. You will also need to write functions to read input from the user and map it to the appropriate input registers.

7. Put it all together by writing a main function that creates instances of the CPU, memory, graphics, and input structs and connects them together. You will also need to write a loop that fetches and executes instructions from the ROM and updates the graphics and input state.

I hope this helps! This is just a high-level overview, and there will be many more details to consider when writing a fully-featured Game Boy emulator. Let me know if you have any questions or need further guidance

## Resources

https://github.com/gfx-rs/wgpu

https://github.com/RustAudio/cpal

https://github.com/emilk/egui

https://gbdev.io/pandocs/

https://www.youtube.com/watch?v=e87qKixKFME

https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html

https://rylev.github.io/DMG-01/public/book/introduction.html

http://bgb.bircd.org/pandocs.htm

https://github.com/alexcrichton/jba/tree/rust

https://github.com/p4ddy1/gbemulator

https://github.com/rylev/DMG-01

talk for the above repo

https://www.youtube.com/watch?v=B7seNuQncvU

https://github.com/Gekkio/mooneye-gb
