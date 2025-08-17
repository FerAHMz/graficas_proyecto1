use raylib::prelude::*;
use std::f32::consts::PI;
use gilrs::{Gilrs, Button, Axis, Event, EventType};

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

    // Gamepad input handling
    pub fn update_gamepad(&mut self, gilrs: &mut Gilrs, maze: &Vec<Vec<char>>) {
        const MOVE_SPEED: f32 = 3.0;
        const ROTATION_SPEED: f32 = PI / 30.0;
        const STICK_DEADZONE: f32 = 0.1;

        // Handle gamepad events (sin debug verbose)
        while let Some(Event { id, event, time: _ }) = gilrs.next_event() {
            match event {
                EventType::Connected => {
                    println!("🎮 Controller conectado: {}", gilrs.gamepad(id).name());
                }
                EventType::Disconnected => {
                    println!("🎮 Controller desconectado");
                }
                _ => {}
            }
        }

        // Check active gamepads for continuous input
        let gamepad_count = gilrs.gamepads().count();
        if gamepad_count == 0 {
            return; // No hay gamepads, salir temprano
        }
        
        for (_id, gamepad) in gilrs.gamepads() {
            // Left stick for movement
            let left_x = gamepad.value(Axis::LeftStickX);
            let left_y = gamepad.value(Axis::LeftStickY);

            // Right stick for camera rotation
            let right_x = gamepad.value(Axis::RightStickX);

            // D-pad for movement (digital) - REMOVED: now only for menus
            // Face buttons are now only for menu interaction, not movement
            
            // Only analog sticks for player movement in-game
            // This prevents interference with menu navigation

            // Triggers for rotation (alternative)
            let left_trigger = gamepad.value(Axis::LeftZ);
            let right_trigger = gamepad.value(Axis::RightZ);

            // Movement with left stick (with deadzone)
            if left_y.abs() > STICK_DEADZONE {
                if left_y > 0.0 {
                    self.move_forward(MOVE_SPEED * left_y, maze);
                } else {
                    self.move_backward(MOVE_SPEED * -left_y, maze);
                }
            }

            if left_x.abs() > STICK_DEADZONE {
                if left_x > 0.0 {
                    self.strafe_right(MOVE_SPEED * left_x, maze);
                } else {
                    self.strafe_left(MOVE_SPEED * -left_x, maze);
                }
            }

            // Camera rotation with right stick - FIXED: removed negative sign
            if right_x.abs() > STICK_DEADZONE {
                self.rotate(right_x * ROTATION_SPEED * 2.0); // Multiply for faster rotation
            }

            // Trigger rotation (alternative control scheme)
            if left_trigger > 0.1 {
                self.rotate(ROTATION_SPEED * left_trigger);
            }
            if right_trigger > 0.1 {
                self.rotate(-ROTATION_SPEED * right_trigger);
            }
        }
    }

    // Mouse input handling
    pub fn update_mouse(&mut self, rl: &RaylibHandle) {
        const MOUSE_SENSITIVITY: f32 = 0.002; // Adjust for sensitivity
        
        let mouse_delta_x = rl.get_mouse_delta().x;
        
        // Only rotate if there's mouse movement - FIXED: removed negative sign
        if mouse_delta_x.abs() > 0.1 {
            self.rotate(mouse_delta_x * MOUSE_SENSITIVITY);
        }
    }
}

// Updated function that uses the new camera system with gamepad support
pub fn process_events(player: &mut Player, rl: &RaylibHandle, maze: &Vec<Vec<char>>, gilrs: &mut Gilrs) {
    // Use the new camera movement system
    player.update_keyboard(rl, maze);
    
    // Only use mouse if cursor is hidden (FPS mode)
    if rl.is_cursor_hidden() {
        player.update_mouse(rl);
    }
    
    // Update gamepad input
    player.update_gamepad(gilrs, maze);
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
