use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::textures::TextureManager;

pub fn render_2d(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>) {
    let scale = 8;
    
    // Dibujar el mapa
    for (y, row) in maze.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let color = match cell {
                '+' => Color::BROWN,        // Esquinas - marrón
                '-' => Color::GRAY,         // Paredes horizontales - gris
                '|' => Color::DARKGRAY,     // Paredes verticales - gris oscuro
                'p' => Color::BLUE,         // Jugador - azul
                'g' => Color::GOLD,         // Meta - dorado
                _ => Color::GREEN,          // Espacios libres - verde (césped)
            };
            
            for dy in 0..scale {
                for dx in 0..scale {
                    let pixel_x = (x * scale + dx) as u32;
                    let pixel_y = (y * scale + dy) as u32;
                    
                    if pixel_x < framebuffer.width && pixel_y < framebuffer.height {
                        framebuffer.set_pixel(pixel_x, pixel_y, color);
                    }
                }
            }
        }
    }
    
    // Dibujar jugador como un círculo rojo
    let player_x = (player.pos.x / 50.0 * scale as f32) as i32;
    let player_y = (player.pos.y / 50.0 * scale as f32) as i32;
    
    for dy in -2..=2 {
        for dx in -2..=2 {
            if dx*dx + dy*dy <= 4 { // Círculo aproximado
                let px = (player_x + dx) as u32;
                let py = (player_y + dy) as u32;
                if px < framebuffer.width && py < framebuffer.height {
                    framebuffer.set_pixel(px, py, Color::RED);
                }
            }
        }
    }
    
    // Dibujar dirección del jugador
    let end_x = player_x + (player.angle.cos() * 15.0) as i32;
    let end_y = player_y + (player.angle.sin() * 15.0) as i32;
    draw_line_in_framebuffer(framebuffer, player_x, player_y, end_x, end_y, Color::YELLOW);
}

pub fn render_3d(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>, _texture_manager: &TextureManager) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let num_rays = width as usize;
    
    for i in 0..num_rays {
        let current_ray = i as f32 / width;
        let ray_angle = player.angle - player.fov / 2.0 + player.fov * current_ray;
        
        let (distance, wall_type) = cast_ray_with_type(player.pos, ray_angle, maze);
        
        // Corregir distorsión de ojo de pez
        let corrected_distance = distance * (ray_angle - player.angle).cos();
        
        // Calcular altura de la pared
        let distance_to_plane = (width / 2.0) / (player.fov / 2.0).tan();
        let wall_height = (50.0 / corrected_distance.max(1.0)) * distance_to_plane;
        let wall_top = ((height / 2.0) - (wall_height / 2.0)).max(0.0);
        let wall_bottom = ((height / 2.0) + (wall_height / 2.0)).min(height);
        
        // Dibujar columna completa
        for y in 0..(height as u32) {
            let color = if (y as f32) < wall_top {
                // Cielo
                Color::new(135, 206, 235, 255) // Azul cielo
            } else if (y as f32) < wall_bottom {
                // Pared con colores según el tipo
                let base_color = match wall_type {
                    '+' => Color::new(139, 69, 19, 255),   // Marrón para esquinas
                    '-' => Color::new(105, 105, 105, 255), // Gris para horizontales
                    '|' => Color::new(169, 169, 169, 255), // Gris claro para verticales
                    _ => Color::GRAY,
                };
                
                // Aplicar sombreado basado en la distancia
                let intensity = 1.0 / (1.0 + corrected_distance * corrected_distance * 0.0005);
                let intensity = intensity.min(1.0).max(0.1);
                
                Color::new(
                    (base_color.r as f32 * intensity) as u8,
                    (base_color.g as f32 * intensity) as u8,
                    (base_color.b as f32 * intensity) as u8,
                    255
                )
            } else {
                // Suelo
                Color::new(34, 139, 34, 255) // Verde césped
            };
            
            framebuffer.set_pixel(i as u32, y, color);
        }
    }
}

fn cast_ray_with_type(start: Vector2, angle: f32, maze: &Vec<Vec<char>>) -> (f32, char) {
    let mut current_pos = start;
    let step_size = 0.5;
    let dx = angle.cos() * step_size;
    let dy = angle.sin() * step_size;
    
    for _ in 0..2000 { // Más pasos para mayor precisión
        current_pos.x += dx;
        current_pos.y += dy;
        
        // Convertir posición a coordenadas del mapa
        let map_x = (current_pos.x / 50.0) as usize;
        let map_y = (current_pos.y / 50.0) as usize;
        
        // Verificar límites
        if map_y >= maze.len() || map_x >= maze[0].len() {
            return (1000.0, '+');
        }
        
        // Verificar si golpeamos una pared
        let cell = maze[map_y][map_x];
        if cell == '+' || cell == '-' || cell == '|' {
            let distance = ((current_pos.x - start.x).powi(2) + (current_pos.y - start.y).powi(2)).sqrt();
            return (distance, cell);
        }
    }
    
    (1000.0, '+') // Si no golpeamos nada, devolver distancia máxima
}

fn draw_line_in_framebuffer(framebuffer: &mut Framebuffer, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;
    
    let mut x = x0;
    let mut y = y0;
    
    loop {
        if x >= 0 && x < framebuffer.width as i32 && y >= 0 && y < framebuffer.height as i32 {
            framebuffer.set_pixel(x as u32, y as u32, color);
        }
        
        if x == x1 && y == y1 {
            break;
        }
        
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}
