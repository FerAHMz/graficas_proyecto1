use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::raycasting::{cast_ray, render_wall_slice};
use crate::textures::TextureManager;
use raylib::color::Color;

pub fn render_2d(fb: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>) {
    let block_size = 20; // Size of each maze cell in pixels
    
    // Draw maze
    for y in 0..maze.len() {
        for x in 0..maze[y].len() {
            let tile = maze[y][x];
            let color = match tile {
                '#' => Color::RED,
                '+' => Color::GREEN,
                '-' => Color::BLUE,
                '|' => Color::YELLOW,
                _ => Color::BLACK, // Empty space
            };
            
            // Draw a block for this cell
            for dy in 0..block_size {
                for dx in 0..block_size {
                    let pixel_x = x * block_size + dx;
                    let pixel_y = y * block_size + dy;
                    if pixel_x < fb.width as usize && pixel_y < fb.height as usize {
                        fb.set_pixel(pixel_x as u32, pixel_y as u32, color);
                    }
                }
            }
        }
    }
    
    // Draw player
    let player_x = player.pos.x as u32;
    let player_y = player.pos.y as u32;
    
    // Draw player as a small circle (simplified as a square)
    for dy in 0..5 {
        for dx in 0..5 {
            let px = player_x + dx;
            let py = player_y + dy;
            if px < fb.width && py < fb.height {
                fb.set_pixel(px, py, Color::WHITE);
            }
        }
    }
    
    // Draw player direction
    let end_x = player.pos.x + player.a.cos() * 20.0;
    let end_y = player.pos.y + player.a.sin() * 20.0;
    
    if end_x >= 0.0 && end_x < fb.width as f32 && end_y >= 0.0 && end_y < fb.height as f32 {
        fb.set_pixel(end_x as u32, end_y as u32, Color::YELLOW);
    }
}

pub fn render_3d(fb: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>, texture_manager: &TextureManager) {
    const SCREEN_WIDTH: usize = 800;
    const SCREEN_HEIGHT: usize = 600;
    const FOV: f32 = std::f32::consts::PI / 3.0;
    const NUM_RAYS: usize = SCREEN_WIDTH; // One ray per column
    const BLOCK_SIZE: f32 = 50.0; // Size of maze blocks in world units

    // Cast rays and render
    for i in 0..NUM_RAYS {
        let current_ray_angle = player.a - (FOV / 2.0) + (FOV * i as f32 / NUM_RAYS as f32);
        let ray_result = cast_ray(fb, player, current_ray_angle, texture_manager, maze, BLOCK_SIZE);
        render_wall_slice(fb, texture_manager, player, &ray_result, i, SCREEN_HEIGHT);
    }
}