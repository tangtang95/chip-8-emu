use anyhow::Error;
use sdl2::{render::{Canvas, TextureCreator}, video::{Window, WindowContext}, surface::Surface, pixels::{PixelFormatEnum, Color}};

pub struct Renderer<const T: usize, const U: usize> {
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>
}

impl<const T: usize, const U: usize> Renderer<T, U> {
    pub fn new(canvas: Canvas<Window>) -> Self {
        let texture_creator = canvas.texture_creator();
        Self {
            canvas, 
            texture_creator: texture_creator
        }
    }

    pub fn render_bw_pixels(&mut self, pixels: &[[u8; T]; U]) -> Result<(), Error> {
        let mut pixels: Vec<u8> = pixels
            .iter()
            .flat_map(|array| array.iter())
            .cloned()
            .collect();
        let mut surface = Surface::from_data(&mut pixels, T as u32, U as u32, T as u32, PixelFormatEnum::Index8).map_err(Error::msg)?;
        surface.set_color_key(true, Color::BLACK).map_err(Error::msg)?;
        let texture = self.texture_creator.create_texture_from_surface(surface)?;
        self.canvas.copy(&texture, None, None).map_err(Error::msg)?;

        Ok(())
    }

    pub fn update(&mut self) {
        self.canvas.present();
    }

}