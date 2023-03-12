# chip-8-emu
[![CI](https://github.com/tangtang95/chip-8-emu/actions/workflows/ci.yaml/badge.svg?branch=master&event=push)](https://github.com/tangtang95/chip-8-emu/actions/workflows/ci.yaml)

Rust implementation of Chip-8 emulator/interpreter using SDL2 (OpenGL)

## Usage

Run the emulator with a rom by passing its `<rom-path>` as argument:
```bash
chip-8-emu.exe <rom-path>
```

For the help info run the following command:
```bash
chip-8-emu.exe --help
```

## Build locally
To build the project locally, it is required to have rust toolchain (i.e. rustc compiler and cargo). Then it is required to install vcpkg through `cargo-vcpkg` tool in order to build `sdl2`.

### Requirements

- `rustc` compiler
- `cargo`

### Steps

- Install `cargo-vcpkg`
```bash
cargo install cargo-vcpkg
```
- Build `sdl2` via `cargo-vcpkg`
```bash
cargo vcpkg build
```
- Build the project
```bash
cargo build
```
- This will generate the executable in the folder `target/debug`