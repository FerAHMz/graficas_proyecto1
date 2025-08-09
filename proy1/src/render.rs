use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::textures::TextureManager;
use crate::caster::{cast_ray, Intersect};

// Sample a color from a texture at given UV coordinates
fn sample_texture(texture: &Texture2D, u: f32, v: f32) -> Color {
    let x = (u * texture.width as f32) as i32;
    let y = (v * texture.height as f32) as i32;
    
    let x = x.max(0).min(texture.width - 1);
    let y = y.max(0).min(texture.height - 1);
    
    // Since we can't directly sample pixels from Texture2D in raylib-rs,
    // we'll use a simplified approach with base colors
    // This could be improved by keeping the original Image data
    Color::SKYBLUE // Placeholder - in a full implementation you'd sample the actual texture
}

// Generate Pokemon-themed sky colors (inspired by classic Pokemon sky)
fn get_pokemon_sky_color(x: u32, y: u32) -> Color {
    let pattern = (x / 20 + y / 15) % 3;
    match pattern {
        0 => Color::new(135, 206, 250, 255), // Light sky blue
        1 => Color::new(176, 224, 230, 255), // Powder blue  
        _ => Color::new(173, 216, 230, 255), // Light blue
    }
}

// Generate Pokemon-themed floor colors (inspired by classic Pokemon grass)
fn get_pokemon_floor_color(x: u32, y: u32) -> Color {
    let pattern = (x / 15 + y / 20) % 4;
    match pattern {
        0 => Color::new(34, 139, 34, 255),   // Forest green
        1 => Color::new(50, 205, 50, 255),   // Lime green
        2 => Color::new(46, 125, 50, 255),   // Dark green
        _ => Color::new(76, 175, 80, 255),   // Medium green
    }
}

fn cell_to_color(cell: char) -> Color {
    match cell {
        '+' => Color::BROWN,
        '-' => Color::GRAY,
        '|' => Color::DARKGRAY,
        'g' => Color::GOLD,          // Goal - gold color but walkable
        's' => Color::LIME,          // Start - lime color but walkable  
        _ => Color::new(200, 200, 200, 255), // Light gray for empty spaces
    }
}

fn draw_cell(
    framebuffer: &mut Framebuffer,
    xo: usize,
    yo: usize,
    block_size: usize,
    cell: char,
) {
    // Only draw walls and special markers, not empty spaces
    if cell == ' ' {
        return;
    }
    
    let color = cell_to_color(cell);
    framebuffer.set_current_color(color);

    // For goal and start, just draw a small marker in the center
    if cell == 'g' || cell == 's' {
        let center_x = xo + block_size / 2;
        let center_y = yo + block_size / 2;
        let marker_size = block_size / 3;
        
        for x in center_x.saturating_sub(marker_size/2)..center_x + marker_size/2 {
            for y in center_y.saturating_sub(marker_size/2)..center_y + marker_size/2 {
                framebuffer.set_pixel(x as u32, y as u32);
            }
        }
    } else {
        // Draw full blocks for walls
        for x in xo..xo + block_size {
            for y in yo..yo + block_size {
                framebuffer.set_pixel(x as u32, y as u32);
            }
        }
    }
}

pub fn render_2d(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>) {
    let block_size = 32; // 2D display block size (8x6 maze = 256x192 pixels) 
    let world_block_size = 20; // Size used in the world coordinates
    
    // Draw the maze
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = col_index * block_size;
            let yo = row_index * block_size;
            draw_cell(framebuffer, xo, yo, block_size, cell);
        }
    }

    // Scale player position to 2D view coordinates
    let scaled_player_x = player.pos.x / world_block_size as f32 * block_size as f32;
    let scaled_player_y = player.pos.y / world_block_size as f32 * block_size as f32;

    framebuffer.set_current_color(Color::RED);

    // Draw player position - make it more visible
    let player_x = scaled_player_x as u32;
    let player_y = scaled_player_y as u32;
    
    // Draw a larger circle for the player (proportional to new block size)
    let player_size = (block_size / 10).max(4); // Scale player size with block size
    for dy in 0..player_size * 2 {
        for dx in 0..player_size * 2 {
            let px = player_x.saturating_sub(player_size as u32).saturating_add(dx as u32);
            let py = player_y.saturating_sub(player_size as u32).saturating_add(dy as u32);
            if px < framebuffer.width && py < framebuffer.height {
                // Draw a circle pattern
                let center_x = player_size;
                let center_y = player_size;
                let distance = ((dx as i32 - center_x as i32).pow(2) + (dy as i32 - center_y as i32).pow(2)) as f32;
                if distance <= (player_size as f32).powi(2) {
                    framebuffer.set_pixel(px, py);
                }
            }
        }
    }
    
    // Draw direction indicator
    framebuffer.set_current_color(Color::YELLOW);
    let dir_length = (block_size as f32 * 0.4).max(20.0); // Scale direction length with block size
    let end_x = player_x as f32 + player.a.cos() * dir_length;
    let end_y = player_y as f32 + player.a.sin() * dir_length;
    
    // Simple line drawing for direction
    let steps = 10;
    for i in 0..steps {
        let t = i as f32 / steps as f32;
        let x = (player_x as f32 + t * (end_x - player_x as f32)) as u32;
        let y = (player_y as f32 + t * (end_y - player_y as f32)) as u32;
        if x < framebuffer.width && y < framebuffer.height {
            framebuffer.set_pixel(x, y);
        }
    }

    framebuffer.set_current_color(Color::WHITESMOKE);

    // Create a scaled player for raycasting in 2D view
    let scaled_player = Player {
        pos: Vector2::new(scaled_player_x, scaled_player_y),
        a: player.a,
        fov: player.fov,
    };

    // draw what the player sees with scaled coordinates
    let num_rays = 10;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = scaled_player.a - (scaled_player.fov / 2.0) + (scaled_player.fov * current_ray);
        cast_ray_2d(framebuffer, &maze, &scaled_player, a, block_size);
    }
}

// Special raycasting function for 2D view with scaled coordinates
fn cast_ray_2d(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &Player,
    a: f32,
    block_size: usize,
) {
    let mut d = 0.0;
    let max_distance = 100.0; // Limit ray length for 2D view

    framebuffer.set_current_color(Color::WHITESMOKE);

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();
        let x = player.pos.x + cos;
        let y = player.pos.y + sin;

        let i = (x / block_size as f32) as usize;
        let j = (y / block_size as f32) as usize;

        if j >= maze.len() || i >= maze[0].len() || d > max_distance {
            break;
        }

        if maze[j][i] == '+' || maze[j][i] == '-' || maze[j][i] == '|' {
            break;
        }

        if x >= 0.0 && y >= 0.0 && (x as u32) < framebuffer.width && (y as u32) < framebuffer.height {
            framebuffer.set_pixel(x as u32, y as u32);
        }
        
        d += 0.5; // Smaller increment for smoother lines in 2D
    }
}

pub fn render_3d(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>, texture_manager: &TextureManager) {
    let num_rays = framebuffer.width;
    let hh = framebuffer.height as f32 / 2.0;  // precalculated half height
    let world_block_size = 20; // Must match the block size used in player.rs and 2D rendering

    framebuffer.set_current_color(Color::WHITESMOKE);

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, world_block_size, false);

        // Calculate the height of the stake
        let distance_to_wall = intersect.distance;
        let distance_to_projection_plane = 70.0;
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

        // Calculate the position to draw the stake
        let stake_top = (hh - (stake_height / 2.0)) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)) as usize;

        // Set color based on wall type
        let wall_color = match intersect.impact {
            '+' => Color::BROWN,
            '-' => Color::GRAY,
            '|' => Color::DARKGRAY,
            _ => Color::WHITE,
        };

        // Apply distance shading
        let intensity = 1.0 / (1.0 + distance_to_wall * distance_to_wall * 0.0001);
        let intensity = intensity.min(1.0).max(0.1);
        
        let shaded_color = Color::new(
            (wall_color.r as f32 * intensity) as u8,
            (wall_color.g as f32 * intensity) as u8,
            (wall_color.b as f32 * intensity) as u8,
            255
        );

        framebuffer.set_current_color(shaded_color);

        // Draw the wall column
        for y in stake_top..stake_bottom.min(framebuffer.height as usize) {
            framebuffer.set_pixel(i, y as u32);
        }

        // Draw Pokemon-themed sky
        for y in 0..stake_top {
            let sky_color = if texture_manager.sky_texture.is_some() {
                get_pokemon_sky_color(i, y as u32)
            } else {
                Color::new(135, 206, 235, 255) // Fallback sky blue
            };
            framebuffer.set_current_color(sky_color);
            framebuffer.set_pixel(i, y as u32);
        }

        // Draw Pokemon-themed floor
        for y in stake_bottom..framebuffer.height as usize {
            let floor_color = if texture_manager.floor_texture.is_some() {
                get_pokemon_floor_color(i, y as u32)
            } else {
                Color::new(34, 139, 34, 255) // Fallback green grass
            };
            framebuffer.set_current_color(floor_color);
            framebuffer.set_pixel(i, y as u32);
        }
    }
}


