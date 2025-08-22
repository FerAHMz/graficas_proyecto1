use raylib::prelude::*;
use std::collections::HashMap;

pub struct TextureManager {
    pub textures: HashMap<char, Texture2D>,
    pub sky_texture: Option<Texture2D>,
    pub floor_texture: Option<Texture2D>,
    // Cache de píxeles para acceso rápido
    pub wall_texture_cache: Option<Vec<Vec<Color>>>,
    pub floor_texture_cache: Option<Vec<Vec<Color>>>,
    pub sky_texture_cache: Option<Vec<Vec<Color>>>,
}

impl TextureManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut textures = HashMap::new();
        
        // Cargar imagen de pared y crear texturas
        let wall_texture = Self::load_texture_from_path(rl, thread, "assets/img/wall2.jpg");
        
        // Asignar la misma textura a todos los tipos de pared
        if let Some(texture) = wall_texture {
            textures.insert('+', texture);
        } else {
            // Fallback a textura de color si no se puede cargar la imagen
            textures.insert('+', Self::create_color_texture(rl, thread, Color::BROWN, 64, 64));
        }
        
        // Para las otras paredes, clonamos la misma textura o usamos fallbacks
        if let Some(wall_tex) = textures.get(&'+') {
            // Como no podemos clonar Texture2D directamente, usamos la misma textura para todas
            // o cargamos la misma imagen múltiples veces
            let wall_texture2 = Self::load_texture_from_path(rl, thread, "assets/img/wall2.jpg");
            let wall_texture3 = Self::load_texture_from_path(rl, thread, "assets/img/wall2.jpg");
            
            textures.insert('-', wall_texture2.unwrap_or_else(|| Self::create_color_texture(rl, thread, Color::DARKBROWN, 64, 64)));
            textures.insert('|', wall_texture3.unwrap_or_else(|| Self::create_color_texture(rl, thread, Color::GRAY, 64, 64)));
        }
        
        // Cargar texturas para cielo y piso
        let sky_texture = Self::load_texture_from_path(rl, thread, "assets/img/wall1.jpg");
        let floor_texture = Self::load_texture_from_path(rl, thread, "assets/img/wall3.jpg");
        
        let mut manager = TextureManager { 
            textures,
            sky_texture,
            floor_texture,
            wall_texture_cache: None,
            floor_texture_cache: None,
            sky_texture_cache: None,
        };
        
        // Crear cachés de píxeles para acceso rápido desde las imágenes reales
        manager.create_texture_caches_from_images(rl, thread);
        
        manager
    }
    
    fn create_color_texture(rl: &mut RaylibHandle, thread: &RaylibThread, color: Color, width: i32, height: i32) -> Texture2D {
        let image = Image::gen_image_color(width, height, color);
        rl.load_texture_from_image(thread, &image).unwrap()
    }
    
    fn load_texture_from_path(rl: &mut RaylibHandle, thread: &RaylibThread, path: &str) -> Option<Texture2D> {
        match rl.load_texture(thread, path) {
            Ok(texture) => {
                println!("Textura cargada exitosamente: {}", path);
                Some(texture)
            },
            Err(e) => {
                println!("Warning: Failed to load texture {}: {}. Using fallback color.", path, e);
                None
            }
        }
    }
    
    fn create_texture_caches_from_images(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        // Crear cache para textura de pared desde la imagen real
        match Image::load_image("assets/img/wall2.jpg") {
            Ok(wall_image) => {
                println!("Creando cache de textura de pared desde wall2.jpg");
                self.wall_texture_cache = Some(Self::create_pixel_cache(&wall_image));
            },
            Err(e) => {
                println!("No se pudo cargar wall2.jpg: {}, usando textura procedural para paredes", e);
                // Fallback a textura procedural
                let mut cache = Vec::new();
                for y in 0..64 {
                    let mut row = Vec::new();
                    for x in 0..64 {
                        let intensity = ((x + y) % 16) as f32 / 16.0;
                        let color = Color::new(
                            (120.0 + intensity * 50.0) as u8,
                            (80.0 + intensity * 30.0) as u8,
                            (40.0 + intensity * 20.0) as u8,
                            255
                        );
                        row.push(color);
                    }
                    cache.push(row);
                }
                self.wall_texture_cache = Some(cache);
            }
        }
        
        // Crear cache para textura de piso desde la imagen real
        match Image::load_image("assets/img/wall3.jpg") {
            Ok(floor_image) => {
                println!("Creando cache de textura de piso desde wall3.jpg");
                self.floor_texture_cache = Some(Self::create_pixel_cache(&floor_image));
            },
            Err(e) => {
                println!("No se pudo cargar wall3.jpg: {}, usando textura procedural para piso", e);
                // Fallback a textura procedural
                let mut cache = Vec::new();
                for y in 0..64 {
                    let mut row = Vec::new();
                    for x in 0..64 {
                        let intensity = ((x * 2 + y) % 12) as f32 / 12.0;
                        let color = Color::new(
                            (60.0 + intensity * 40.0) as u8,
                            (100.0 + intensity * 50.0) as u8,
                            (30.0 + intensity * 25.0) as u8,
                            255
                        );
                        row.push(color);
                    }
                    cache.push(row);
                }
                self.floor_texture_cache = Some(cache);
            }
        }
        
        // Crear cache para textura de cielo desde la imagen real
        match Image::load_image("assets/img/wall1.jpg") {
            Ok(sky_image) => {
                println!("Creando cache de textura de cielo desde wall1.jpg");
                self.sky_texture_cache = Some(Self::create_pixel_cache(&sky_image));
            },
            Err(e) => {
                println!("No se pudo cargar wall1.jpg: {}, usando textura procedural para cielo", e);
                // Fallback a textura procedural
                let mut cache = Vec::new();
                for y in 0..64 {
                    let mut row = Vec::new();
                    for x in 0..64 {
                        let intensity = ((x + y * 2) % 20) as f32 / 20.0;
                        let color = Color::new(
                            (135.0 + intensity * 40.0) as u8,
                            (180.0 + intensity * 40.0) as u8,
                            (220.0 + intensity * 35.0) as u8,
                            255
                        );
                        row.push(color);
                    }
                    cache.push(row);
                }
                self.sky_texture_cache = Some(cache);
            }
        }
    }
    
    fn create_pixel_cache(image: &Image) -> Vec<Vec<Color>> {
        let mut cache = Vec::new();
        let mut temp_image = image.clone();
        
        for y in 0..image.height {
            let mut row = Vec::new();
            for x in 0..image.width {
                let color = temp_image.get_color(x, y);
                row.push(color);
            }
            cache.push(row);
        }
        cache
    }
    
    pub fn get_wall_texture_pixel(&self, tx: f32, ty: f32) -> Color {
        if let Some(ref cache) = self.wall_texture_cache {
            let tex_x = ((tx * cache[0].len() as f32) as usize).min(cache[0].len() - 1);
            let tex_y = ((ty * cache.len() as f32) as usize).min(cache.len() - 1);
            cache[tex_y][tex_x]
        } else {
            // Fallback a textura procedural si no hay cache
            let r = (tx * 255.0) as u8;
            let g = (ty * 255.0) as u8;
            let b = ((tx + ty) * 127.5) as u8;
            Color::new(r.max(100), g.max(80), b.max(60), 255)
        }
    }
    
    pub fn get_floor_texture_pixel(&self, tx: f32, ty: f32) -> Color {
        if let Some(ref cache) = self.floor_texture_cache {
            let tex_x = ((tx * cache[0].len() as f32) as usize).min(cache[0].len() - 1);
            let tex_y = ((ty * cache.len() as f32) as usize).min(cache.len() - 1);
            cache[tex_y][tex_x]
        } else {
            // Fallback a color sólido si no hay cache
            Color::new(139, 90, 43, 255) // Color marrón para el piso
        }
    }
    
    pub fn get_sky_texture_pixel(&self, tx: f32, ty: f32) -> Color {
        if let Some(ref cache) = self.sky_texture_cache {
            let tex_x = ((tx * cache[0].len() as f32) as usize).min(cache[0].len() - 1);
            let tex_y = ((ty * cache.len() as f32) as usize).min(cache.len() - 1);
            cache[tex_y][tex_x]
        } else {
            // Fallback a color de cielo si no hay cache
            Color::new(135, 206, 235, 255) // Color azul cielo
        }
    }
    
    pub fn get_texture(&self, wall_type: char) -> Option<&Texture2D> {
        self.textures.get(&wall_type)
    }
}
