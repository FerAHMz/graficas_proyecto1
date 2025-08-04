use raylib::prelude::*;
use std::collections::HashMap;

pub struct TextureManager {
    pub textures: HashMap<char, Texture2D>,
}

impl TextureManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut textures = HashMap::new();
        
        // Cargar texturas Pokémon (por ahora usaremos colores, luego las cambiaremos por texturas reales)
        // Estas serán las texturas que corresponden a diferentes tipos de pared
        textures.insert('+', Self::create_color_texture(rl, thread, Color::BROWN, 64, 64)); // Paredes esquinas - Madera
        textures.insert('-', Self::create_color_texture(rl, thread, Color::DARKBROWN, 64, 64)); // Paredes horizontales - Piedra
        textures.insert('|', Self::create_color_texture(rl, thread, Color::GRAY, 64, 64)); // Paredes verticales - Roca
        
        TextureManager { textures }
    }
    
    fn create_color_texture(rl: &mut RaylibHandle, thread: &RaylibThread, color: Color, width: i32, height: i32) -> Texture2D {
        let image = Image::gen_image_color(width, height, color);
        rl.load_texture_from_image(thread, &image).unwrap()
    }
    
    pub fn get_texture(&self, wall_type: char) -> Option<&Texture2D> {
        self.textures.get(&wall_type)
    }
}
