#![allow(unused_imports)]
#![allow(dead_code)]

mod line;
mod framebuffer;
mod maze;
mod caster;
mod player;
mod render;
mod textures;
mod game_state;

use line::line;
use maze::Maze;
use caster::{cast_ray, Intersect};
use framebuffer::Framebuffer;
use player::{Player, process_events};
use render::{render_2d, render_3d};
use textures::TextureManager;
use game_state::{GameState, GameStateManager};

use raylib::prelude::*;
use std::thread;
use std::time::{Duration, Instant};
use std::f32::consts::PI;

fn main() {
    let window_width = 800;
    let window_height = 600;

    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Pokémon Raycaster")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    // Enable mouse for camera rotation
    rl.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_CROSSHAIR);
    rl.disable_cursor(); // Hide cursor for FPS-style controls

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
    framebuffer.set_background_color(Color::new(50, 50, 100, 255));

    let maze_obj = Maze::new(8, 6);
    let maze = &maze_obj.map;
    
    // Create player at a simple, known valid position for 8x6 maze
    let mut player = Player {
        pos: Vector2::new(30.0, 30.0), // Position adjusted for BLOCK_SIZE = 20 (1.5, 1.5 in maze coordinates)
        a: PI / 4.0,
        fov: PI / 3.0,
    };

    let texture_manager = TextureManager::new(&mut rl, &thread);
    let mut mode_3d = true;
    let mut mouse_enabled = true; // Track mouse control state
    let mut game_state_manager = GameStateManager::new();
    
    // FPS tracking
    let mut frame_count = 0;
    let mut fps_timer = Instant::now();
    let mut current_fps = 0.0;
    let _target_fps = 60.0; // Target FPS para comparison
    
    // Game variables - commented out for now since we need to implement these modules
    // let mut show_2d = false;
    // let mut game_won = false;
    // let mut goal_pos = maze_obj.find_goal().unwrap_or((maze_obj.width - 2, maze_obj.height - 2));
    // let mut victory_sound_played = false;
    // let mut last_movement_time = Instant::now();

    println!("¡Bienvenido al Pokémon Raycaster!");

    // Main game loop
    while !rl.window_should_close() {
        // Update FPS (update every 0.5 seconds for more responsive display)
        frame_count += 1;
        if fps_timer.elapsed().as_secs_f32() >= 0.5 {
            current_fps = frame_count as f32 / fps_timer.elapsed().as_secs_f32();
            frame_count = 0;
            fps_timer = Instant::now();
        }
        
        // Update game state
        game_state_manager.update(&mut rl);
        
        // Check if player reached goal in Playing state
        if game_state_manager.current_state == GameState::Playing {
            let world_block_size = 20.0;
            let maze_x = (player.pos.x / world_block_size) as usize;
            let maze_y = (player.pos.y / world_block_size) as usize;
            
            if maze_y < maze.len() && maze_x < maze[0].len() && maze[maze_y][maze_x] == 'g' {
                game_state_manager.current_state = GameState::Victory;
            }
        }
        
        match game_state_manager.current_state {
            GameState::Welcome => {
                let mut d = rl.begin_drawing(&thread);
                game_state_manager.draw_welcome(&mut d);
            },
            GameState::LevelSelect => {
                let mut d = rl.begin_drawing(&thread);
                game_state_manager.draw_level_select(&mut d);
            },
            GameState::Playing => {
                // 1. clear framebuffer
                framebuffer.clear();

                // 2. move the player on user input
                process_events(&mut player, &rl, &maze);

                // 3. toggle between 2D and 3D mode
                if rl.is_key_pressed(KeyboardKey::KEY_M) {
                    mode_3d = !mode_3d;
                }

                // Toggle mouse control with C key
                if rl.is_key_pressed(KeyboardKey::KEY_C) {
                    mouse_enabled = !mouse_enabled;
                    if mouse_enabled {
                        rl.disable_cursor();
                    } else {
                        rl.enable_cursor();
                    }
                }
                
                // Back to menu with ESC
                if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                    game_state_manager.current_state = GameState::Welcome;
                }

                // 4. draw stuff
                if mode_3d {
                    render_3d(&mut framebuffer, &player, &maze, &texture_manager);
                } else {
                    render_2d(&mut framebuffer, &player, &maze);
                }

                // 5. draw framebuffer content with FPS and minimap
                framebuffer.swap_buffers_with_fps_and_minimap(&mut rl, &thread, current_fps, &player, &maze);
            },
            GameState::Victory => {
                // Check input first (before begin_drawing)
                let should_exit = rl.is_key_pressed(KeyboardKey::KEY_ESCAPE);
                let should_restart = rl.is_key_pressed(KeyboardKey::KEY_R);
                
                let mut d = rl.begin_drawing(&thread);
                game_state_manager.draw_victory(&mut d);
                
                // Apply state changes after drawing
                if should_exit {
                    game_state_manager.current_state = GameState::Welcome;
                }
                if should_restart {
                    // Reset player position when restarting
                    player.pos = Vector2::new(30.0, 30.0);
                    game_state_manager.current_state = GameState::Playing;
                }
            }
        }

        thread::sleep(Duration::from_millis(16));
    }
}