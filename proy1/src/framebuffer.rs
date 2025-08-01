use raylib::prelude::*;
use std::convert::TryInto;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            background_color,
        );
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(
            self.width.try_into().unwrap(),
            self.height.try_into().unwrap(),
            self.background_color,
        );
    }

    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            Image::draw_pixel(&mut self.color_buffer, x as i32, y as i32, self.current_color);
        }
    }

    pub fn get_color(&self, x: u32, y: u32) -> Color {
        // Verificar que las coordenadas estén dentro de los límites
        if x < self.width && y < self.height {
            let index = ((y * self.width + x) * 4) as usize;
            let data = unsafe { std::slice::from_raw_parts(self.color_buffer.data as *const u8, self.color_buffer.width as usize * self.color_buffer.height as usize * 4) };

            // Devolver el color si el índice es válido
            if index + 3 < data.len() {
                return Color::new(data[index], data[index + 1], data[index + 2], data[index + 3]);
            }
        }

        self.background_color // Devolver el color de fondo si fuera una coordenada fuera de límites
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }
}
