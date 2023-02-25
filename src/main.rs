use anyhow::Error;
use log::{debug, info};
use sdl2::{event::Event, keyboard::{Keycode}, pixels::{Color, PixelFormatEnum}, surface::Surface};
use simple_logger::SimpleLogger;
use std::{time::Duration, io::{BufReader, Read}, fs::File};
use chip_8_emu::{cpu::Cpu, memory::Memory, timer::Timer};

fn find_sdl_gl_driver() -> Result<u32, Error> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            debug!(target: "SDL2", "OpenGL driver found: {}", index);
            return Ok(index as u32);
        }
    }
    Err(Error::msg("Could not find OpenGL driver!"))
}

fn read_rom_from_file(file_path: &str) -> Result<Vec<u8>, Error> {
    let file_handle = File::open(file_path)?;
    let mut reader = BufReader::new(file_handle);
    let mut buffer = Vec::new();
    
    reader.read_to_end(&mut buffer)?;
    
    Ok(buffer)
}

fn main() -> Result<(), Error> {
    SimpleLogger::new().init()?;

    info!("Chip 8 Emulator is starting...");

    let sdl_context = sdl2::init().map_err(Error::msg)?;
    let window = sdl_context
        .video()
        .map_err(Error::msg)?
        .window("Chip-8 Emulator", 640, 320)
        .opengl()
        .build()?;
    let mut canvas = window
        .into_canvas()
        .index(find_sdl_gl_driver()?)
        .build()?;
    
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().map_err(Error::msg)?;

    let mut memory = Memory::new();
    memory.load_font_data();
    memory.load_rom_data(&(read_rom_from_file("roms/BC_test.ch8")?));

    let mut timer = Timer::new();

    let mut cpu = Cpu::new(&mut memory, &mut timer);

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => break 'main_loop,
                _ => {}
            }
        }

        cpu.tick();
        let mut pixels: Vec<u8> = cpu.get_display()
            .iter()
            .flat_map(|array| array.iter())
            .cloned()
            .collect();
        let mut surface = Surface::from_data(&mut pixels, 64, 32, 64, PixelFormatEnum::Index8).map_err(Error::msg)?;
        surface.set_color_key(true, Color::BLACK).map_err(Error::msg)?;
        let texture = texture_creator.create_texture_from_surface(surface)?;
        canvas.copy(&texture, None, None).map_err(Error::msg)?;
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
