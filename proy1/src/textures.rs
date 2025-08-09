use raylib::prelude::*;
use std::collections::HashMap;

pub struct TextureManager {
    pub textures: HashMap<char, Texture2D>,
    pub sky_texture: Option<Texture2D>,
    pub floor_texture: Option<Texture2D>,
}

impl TextureManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut textures = HashMap::new();
        
        // Cargar texturas para las paredes
        textures.insert('+', Self::create_color_texture(rl, thread, Color::BROWN, 64, 64)); // Paredes esquinas - Madera
        textures.insert('-', Self::create_color_texture(rl, thread, Color::DARKBROWN, 64, 64)); // Paredes horizontales - Piedra
        textures.insert('|', Self::create_color_texture(rl, thread, Color::GRAY, 64, 64)); // Paredes verticales - Roca
        
        // Cargar texturas de Pokémon para cielo y piso
        let sky_texture = Self::load_pokemon_texture(rl, thread, "assets/wall1.png");
        let floor_texture = Self::load_pokemon_texture(rl, thread, "assets/wall3.png");
        
        TextureManager { 
            textures,
            sky_texture,
            floor_texture,
        }
    }
    
    fn create_color_texture(rl: &mut RaylibHandle, thread: &RaylibThread, color: Color, width: i32, height: i32) -> Texture2D {
        let image = Image::gen_image_color(width, height, color);
        rl.load_texture_from_image(thread, &image).unwrap()
    }
    
    fn load_pokemon_texture(rl: &mut RaylibHandle, thread: &RaylibThread, path: &str) -> Option<Texture2D> {
        match rl.load_texture(thread, path) {
            Ok(texture) => Some(texture),
            Err(e) => {
                println!("Warning: Failed to load texture {}: {}. Using fallback color.", path, e);
                None
            }
        }
    }
    
    pub fn get_texture(&self, wall_type: char) -> Option<&Texture2D> {
        self.textures.get(&wall_type)
    }
}
