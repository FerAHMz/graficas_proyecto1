use raylib::prelude::*;
use std::time::{Duration, Instant};

pub struct AnimatedSprite {
    pub position: Vector2,
    pub frames: Vec<Rectangle>,
    pub current_frame: usize,
    pub frame_duration: Duration,
    pub last_frame_time: Instant,
    pub texture: Option<Texture2D>,
    pub scale: f32,
    pub color: Color,
}

impl AnimatedSprite {
    pub fn new(position: Vector2, frame_duration: Duration) -> Self {
        AnimatedSprite {
            position,
            frames: vec![
                Rectangle::new(0.0, 0.0, 32.0, 32.0),
                Rectangle::new(32.0, 0.0, 32.0, 32.0),
                Rectangle::new(64.0, 0.0, 32.0, 32.0),
                Rectangle::new(96.0, 0.0, 32.0, 32.0),
            ],
            current_frame: 0,
            frame_duration,
            last_frame_time: Instant::now(),
            texture: None,
            scale: 2.0,
            color: Color::WHITE,
        }
    }
    
    pub fn update(&mut self) {
        if self.last_frame_time.elapsed() >= self.frame_duration {
            self.current_frame = (self.current_frame + 1) % self.frames.len();
            self.last_frame_time = Instant::now();
        }
    }
    
    pub fn draw_as_pixels(&self, framebuffer: &mut crate::framebuffer::Framebuffer) {
        // Dibujar sprite como píxeles en el framebuffer
        let sprite_size = 8;
        let colors = [
            Color::YELLOW,  // Pikachu amarillo
            Color::ORANGE,  // Naranja
            Color::RED,     // Rojo
            Color::YELLOW,  // Amarillo de nuevo
        ];
        
        let color = colors[self.current_frame];
        
        // Dibujar un sprite simple de 8x8 píxeles
        for y in 0..sprite_size {
            for x in 0..sprite_size {
                let pixel_x = (self.position.x as i32 + x - sprite_size / 2) as u32;
                let pixel_y = (self.position.y as i32 + y - sprite_size / 2) as u32;
                
                if pixel_x < framebuffer.width && pixel_y < framebuffer.height {
                    framebuffer.set_pixel(pixel_x, pixel_y, color);
                }
            }
        }
    }
}

pub struct SpriteManager {
    pub sprites: Vec<AnimatedSprite>,
}

impl SpriteManager {
    pub fn new() -> Self {
        let mut sprites = Vec::new();
        
        // Crear algunos sprites Pokémon animados en posiciones aleatorias
        sprites.push(AnimatedSprite::new(
            Vector2::new(200.0, 300.0),
            Duration::from_millis(500)
        ));
        
        sprites.push(AnimatedSprite::new(
            Vector2::new(600.0, 200.0),
            Duration::from_millis(400)
        ));
        
        sprites.push(AnimatedSprite::new(
            Vector2::new(400.0, 450.0),
            Duration::from_millis(600)
        ));
        
        SpriteManager { sprites }
    }
    
    pub fn update(&mut self) {
        for sprite in &mut self.sprites {
            sprite.update();
        }
    }
    
    pub fn draw(&self, framebuffer: &mut crate::framebuffer::Framebuffer) {
        for sprite in &self.sprites {
            sprite.draw_as_pixels(framebuffer);
        }
    }
}
