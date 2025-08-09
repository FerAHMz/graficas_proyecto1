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

    // Camera movement methods
    pub fn move_forward(&mut self, speed: f32, maze: &Vec<Vec<char>>) {
        let new_x = self.pos.x + speed * self.a.cos();
        let new_y = self.pos.y + speed * self.a.sin();
        
        if is_position_valid(new_x, new_y, maze, 20.0) {
            self.pos.x = new_x;
            self.pos.y = new_y;
        }
    }

    pub fn move_backward(&mut self, speed: f32, maze: &Vec<Vec<char>>) {
        let new_x = self.pos.x - speed * self.a.cos();
        let new_y = self.pos.y - speed * self.a.sin();
        
        if is_position_valid(new_x, new_y, maze, 20.0) {
            self.pos.x = new_x;
            self.pos.y = new_y;
        }
    }

    pub fn strafe_left(&mut self, speed: f32, maze: &Vec<Vec<char>>) {
        let new_x = self.pos.x + speed * (self.a - PI / 2.0).cos();
        let new_y = self.pos.y + speed * (self.a - PI / 2.0).sin();
        
        if is_position_valid(new_x, new_y, maze, 20.0) {
            self.pos.x = new_x;
            self.pos.y = new_y;
        }
    }

    pub fn strafe_right(&mut self, speed: f32, maze: &Vec<Vec<char>>) {
        let new_x = self.pos.x + speed * (self.a + PI / 2.0).cos();
        let new_y = self.pos.y + speed * (self.a + PI / 2.0).sin();
        
        if is_position_valid(new_x, new_y, maze, 20.0) {
            self.pos.x = new_x;
            self.pos.y = new_y;
        }
    }

    pub fn rotate(&mut self, angle_delta: f32) {
        self.a += angle_delta;
        // Normalize angle to [0, 2π]
        if self.a < 0.0 {
            self.a += 2.0 * PI;
        } else if self.a >= 2.0 * PI {
            self.a -= 2.0 * PI;
        }
    }

    // Keyboard input handling
    pub fn update_keyboard(&mut self, rl: &RaylibHandle, maze: &Vec<Vec<char>>) {
        const MOVE_SPEED: f32 = 3.0;
        const ROTATION_SPEED: f32 = PI / 30.0;

        // Movement
        if rl.is_key_down(KeyboardKey::KEY_W) || rl.is_key_down(KeyboardKey::KEY_UP) {
            self.move_forward(MOVE_SPEED, maze);
        }
        if rl.is_key_down(KeyboardKey::KEY_S) || rl.is_key_down(KeyboardKey::KEY_DOWN) {
            self.move_backward(MOVE_SPEED, maze);
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            self.strafe_left(MOVE_SPEED, maze);
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            self.strafe_right(MOVE_SPEED, maze);
        }

        // Keyboard rotation
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.rotate(ROTATION_SPEED);
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.rotate(-ROTATION_SPEED);
        }
    }

    // Mouse input handling
    pub fn update_mouse(&mut self, rl: &RaylibHandle) {
        const MOUSE_SENSITIVITY: f32 = 0.002; // Adjust for sensitivity
        
        let mouse_delta_x = rl.get_mouse_delta().x;
        
        // Only rotate if there's mouse movement
        if mouse_delta_x.abs() > 0.1 {
            self.rotate(-mouse_delta_x * MOUSE_SENSITIVITY);
        }
    }
}

// Updated function that uses the new camera system
pub fn process_events(player: &mut Player, rl: &RaylibHandle, maze: &Vec<Vec<char>>) {
    // Use the new camera movement system
    player.update_keyboard(rl, maze);
    // Only use mouse if cursor is hidden (FPS mode)
    if rl.is_cursor_hidden() {
        player.update_mouse(rl);
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
