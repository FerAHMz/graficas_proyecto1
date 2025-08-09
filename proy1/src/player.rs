use raylib::prelude::*;
use std::f32::consts::PI;

pub struct Player {
    pub pos: Vector2,
    pub a: f32,
    pub fov: f32, // field of view
}

impl Player {
    pub fn new(pos: Vector2, a: f32, fov: f32) -> Self {
        Player {
            pos,
            a,
            fov,
        }
    }
}

pub fn process_events(player: &mut Player, rl: &RaylibHandle, maze: &Vec<Vec<char>>) {
    const MOVE_SPEED: f32 = 3.0;
    const ROTATION_SPEED: f32 = PI / 30.0;
    const BLOCK_SIZE: f32 = 20.0; // Updated to match render.rs world_block_size

    // Rotation doesn't need collision detection
    if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_A) {
        player.a += ROTATION_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_D) {
        player.a -= ROTATION_SPEED;
    }

    // Movement with collision detection
    if rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_S) {
        let new_x = player.pos.x - MOVE_SPEED * player.a.cos();
        let new_y = player.pos.y - MOVE_SPEED * player.a.sin();
        
        if is_position_valid(new_x, new_y, maze, BLOCK_SIZE) {
            player.pos.x = new_x;
            player.pos.y = new_y;
        }
    }
    if rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_W) {
        let new_x = player.pos.x + MOVE_SPEED * player.a.cos();
        let new_y = player.pos.y + MOVE_SPEED * player.a.sin();
        
        if is_position_valid(new_x, new_y, maze, BLOCK_SIZE) {
            player.pos.x = new_x;
            player.pos.y = new_y;
        }
    }
}

fn is_position_valid(x: f32, y: f32, maze: &Vec<Vec<char>>, block_size: f32) -> bool {
    // Convert world coordinates to maze coordinates
    let maze_x = (x / block_size) as usize;
    let maze_y = (y / block_size) as usize;
    
    // Check bounds
    if maze_y >= maze.len() || maze_x >= maze[0].len() {
        return false;
    }
    
    // Check if the cell is empty (walkable)
    // Allow walking on: empty spaces, goal, start, and any other non-wall characters
    match maze[maze_y][maze_x] {
        '+' | '-' | '|' => false,  // These are walls
        _ => true,  // Everything else is walkable (including ' ', 'g', 's', etc.)
    }
}
