use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::textures::TextureManager;

pub struct RaycastResult {
    pub distance: f32,
    pub hit_wall: char,
}

pub fn cast_ray(
    _fb: &mut Framebuffer,
    player: &Player,
    angle: f32,
    _texture_manager: &TextureManager,
    maze: &Vec<Vec<char>>,
    block_size: f32,
) -> RaycastResult {
    let mut ray_distance = 0.0;
    let mut hit_wall = ' ';
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();
    
    let step_size = 0.1; // Smaller step for more precision

    // Avanzar el rayo en la dirección del ángulo
    loop {
        ray_distance += step_size;
        let x = player.pos.x + ray_distance * cos_angle;
        let y = player.pos.y + ray_distance * sin_angle;

        // Convert world coordinates to maze coordinates
        let maze_x = (x / block_size) as usize;
        let maze_y = (y / block_size) as usize;

        // Check bounds
        if maze_y >= maze.len() || maze_x >= maze[0].len() {
            hit_wall = '#'; // Treat out of bounds as wall
            break;
        }

        // Check if ray hit a wall
        let current_cell = maze[maze_y][maze_x];
        if current_cell != ' ' {
            hit_wall = current_cell;
            break;
        }

        // Prevent infinite loops
        if ray_distance > 1000.0 {
            break;
        }
    }

    RaycastResult {
        distance: ray_distance,
        hit_wall,
    }
}

pub fn render_wall_slice(
    fb: &mut Framebuffer,
    _texture_manager: &TextureManager,
    _player: &Player,
    ray_result: &RaycastResult,
    x_pos: usize,
    screen_height: usize,
) {
    if ray_result.distance == 0.0 {
        return;
    }

    // Calculate wall height on screen
    let wall_height = (screen_height as f32 / ray_result.distance) * 50.0;
    let wall_height = wall_height.min(screen_height as f32) as usize;

    // Calculate wall start and end positions
    let wall_start = (screen_height / 2).saturating_sub(wall_height / 2);
    let wall_end = (screen_height / 2) + (wall_height / 2);

    // Choose color based on wall type
    let wall_color = match ray_result.hit_wall {
        '#' => Color::RED,
        '+' => Color::GREEN,
        '-' => Color::BLUE,
        '|' => Color::YELLOW,
        _ => Color::WHITE,
    };

    // Draw ceiling (above wall)
    for y in 0..wall_start {
        if y < screen_height && x_pos < fb.width as usize {
            fb.set_pixel(x_pos as u32, y as u32, Color::SKYBLUE);
        }
    }

    // Draw wall
    for y in wall_start..wall_end.min(screen_height) {
        if x_pos < fb.width as usize {
            fb.set_pixel(x_pos as u32, y as u32, wall_color);
        }
    }

    // Draw floor (below wall)
    for y in wall_end..screen_height {
        if x_pos < fb.width as usize {
            fb.set_pixel(x_pos as u32, y as u32, Color::DARKGREEN);
        }
    }
}