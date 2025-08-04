use raylib::prelude::*;
use std::f32::consts::PI;

pub struct Player {
    pub pos: Vector2,
    pub angle: f32,
    pub fov: f32,
    pub speed: f32,
    pub rotation_speed: f32,
}

impl Player {
    pub fn new(pos: Vector2, angle: f32, fov: f32) -> Self {
        Player {
            pos,
            angle,
            fov,
            speed: 100.0,
            rotation_speed: 1.0,
        }
    }

    pub fn update_keyboard(&mut self, rl: &mut RaylibHandle) {
        let dt = rl.get_frame_time();
        
        // Movimiento hacia adelante/atrás
        if rl.is_key_down(KeyboardKey::KEY_W) || rl.is_key_down(KeyboardKey::KEY_UP) {
            self.pos.x += self.angle.cos() * self.speed * dt;
            self.pos.y += self.angle.sin() * self.speed * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) || rl.is_key_down(KeyboardKey::KEY_DOWN) {
            self.pos.x -= self.angle.cos() * self.speed * dt;
            self.pos.y -= self.angle.sin() * self.speed * dt;
        }
        
        // Rotación con teclado
        if rl.is_key_down(KeyboardKey::KEY_A) || rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.angle -= self.rotation_speed * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) || rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.angle += self.rotation_speed * dt;
        }
        
        // Normalizar ángulo
        if self.angle < 0.0 {
            self.angle += 2.0 * PI;
        }
        if self.angle >= 2.0 * PI {
            self.angle -= 2.0 * PI;
        }
    }
    
    pub fn update_mouse(&mut self, rl: &mut RaylibHandle) {
        let mouse_delta = rl.get_mouse_delta();
        self.angle += mouse_delta.x * 0.001; // Sensibilidad del mouse
        
        // Normalizar ángulo
        if self.angle < 0.0 {
            self.angle += 2.0 * PI;
        }
        if self.angle >= 2.0 * PI {
            self.angle -= 2.0 * PI;
        }
    }
}
