use log::{debug, info};
use sdl2::{event::Event, keyboard::{Keycode}, pixels::{Color, PixelFormatEnum}, surface::Surface};
use simple_logger::SimpleLogger;
use std::{time::Duration, io::{BufReader, Read, Result as IOResult}, fs::File};
use chip_8_emu::{cpu::Cpu, memory::Memory, timer::Timer};

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            debug!(target: "SDL2", "OpenGL driver found: {}", index);
            return Some(index as u32);
        }
    }
    None
}

fn read_rom_from_file(file_path: &str) -> IOResult<Vec<u8>> {
    let file_handle = File::open(file_path)?;
    let mut reader = BufReader::new(file_handle);
    let mut buffer = Vec::new();
    
    reader.read_to_end(&mut buffer)?;
    
    Ok(buffer)
}

fn main() -> Result<(), String> {
    SimpleLogger::new().init().map_err(|e| e.to_string())?;

    info!("Chip 8 Emulator is starting...");

    let sdl_context = sdl2::init()?;
    let window = sdl_context
        .video()?
        .window("Chip-8 Emulator", 640, 320)
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .map_err(|e| e.to_string())?;
    
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump()?;

    let mut memory = Memory::new();
    memory.load_font_data();
    memory.load_rom_data(
        &(read_rom_from_file("roms/BC_test.ch8").map_err(|e| e.to_string())?)
    );

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
        let mut surface = Surface::from_data(&mut pixels, 64, 32, 64, PixelFormatEnum::Index8)?;
        surface.set_color_key(true, Color::BLACK)?;
        let texture = texture_creator.create_texture_from_surface(surface).unwrap();
        canvas.copy(&texture, None, None)?;
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
