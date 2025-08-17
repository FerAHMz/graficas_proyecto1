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
mod audio;

use line::line;
use maze::Maze;
use caster::{cast_ray, Intersect};
use framebuffer::Framebuffer;
use player::{Player, process_events};
use render::{render_2d, render_3d};
use textures::TextureManager;
use game_state::{GameState, GameStateManager};
use audio::AudioManager;

use raylib::prelude::*;
use std::thread;
use std::time::{Duration, Instant};
use std::f32::consts::PI;
use gilrs::{Gilrs, Button, Axis};

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

    // Initialize gamepad support
    let mut gilrs = Gilrs::new().unwrap_or_else(|err| {
        println!("⚠️ Error initializing gamepad: {}", err);
        println!("🎮 Continuing without gamepad support...");
        Gilrs::new().expect("Failed to create empty Gilrs instance")
    });
    
    // Check for connected gamepads
    println!("🎮 Scanning for controllers...");
    let mut gamepad_count = 0;
    for (_id, gamepad) in gilrs.gamepads() {
        gamepad_count += 1;
        println!("🎮 Controller detected: {}", gamepad.name());
    }
    
    if gamepad_count == 0 {
        println!("❌ No controllers detected");
    } else {
        println!("✅ {} controller(s) ready", gamepad_count);
    }
    
    let maze_obj = Maze::new(8, 6);
    let maze = &maze_obj.map;
    
    // Create player at a safe starting position for 8x6 maze
    let mut player = Player {
        pos: Vector2::new(25.0, 25.0), // Position adjusted for BLOCK_SIZE = 20 (1.25, 1.25 in maze coordinates)
        a: PI / 4.0,
        fov: PI / 3.0,
    };

    let texture_manager = TextureManager::new(&mut rl, &thread);
    let mut mode_3d = true;
    let mut mouse_enabled = true; // Track mouse control state
    let mut game_state_manager = GameStateManager::new();
    let mut frame_counter_since_playing = 0u32; // Counter for frames since entering Playing state
    let mut previous_state = GameState::Welcome; // Track previous state for transitions
    
    // Inicializar sistema de audio con Taylor Swift
    let mut audio_manager = match AudioManager::new() {
        Ok(audio) => {
            println!("🎵 Sistema de audio inicializado con música de Taylor Swift! ✨");
            Some(audio)
        },
        Err(e) => {
            println!("⚠️ Error inicializando audio: {}. Continuando sin audio.", e);
            None
        }
    };
    
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
    
    // Iniciar música de fondo de Taylor Swift
    if let Some(ref mut audio) = audio_manager {
        if let Err(e) = audio.play_background_music() {
            println!("⚠️ Error reproduciendo música: {}", e);
        }
    }

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
        game_state_manager.update(&mut rl, &mut gilrs);
        
        // Check for state transitions and reset player position when leaving Victory state
        if previous_state == GameState::Victory && game_state_manager.current_state != GameState::Victory {
            player.pos = Vector2::new(25.0, 25.0); // Reset player to starting position
            frame_counter_since_playing = 0; // Reset victory delay counter
            println!("🔄 Jugador reseteado a posición inicial tras salir de Victoria");
        }
        previous_state = game_state_manager.current_state;
        
        // Actualizar sistema de audio
        if let Some(ref mut audio) = audio_manager {
            audio.update_music();
            // Handle gamepad controls for audio
            if let Err(e) = audio.handle_gamepad_controls(&gilrs) {
                println!("Error with gamepad audio controls: {}", e);
            }
        }
        
        // Detectar gamepads dinámicamente (solo eventos importantes)
        static mut GAMEPAD_CHECK_COUNTER: u32 = 0;
        unsafe {
            GAMEPAD_CHECK_COUNTER += 1;
            if GAMEPAD_CHECK_COUNTER % 300 == 0 { // Check every 5 seconds
                let current_gamepads = gilrs.gamepads().count();
                if current_gamepads > 0 {
                    for (_id, gamepad) in gilrs.gamepads() {
                        println!("🎮 Connected: {}", gamepad.name());
                    }
                }
            }
        }
        
        // Check if player reached goal in Playing state (with delay to prevent instant victory)
        if game_state_manager.current_state == GameState::Playing {
            frame_counter_since_playing += 1;
            
            // Only check for victory after 60 frames (1 second at 60fps) to prevent instant triggers
            if frame_counter_since_playing > 60 {
                let world_block_size = 20.0;
                let maze_x = (player.pos.x / world_block_size) as usize;
                let maze_y = (player.pos.y / world_block_size) as usize;
                
                if maze_y < maze.len() && maze_x < maze[0].len() && maze[maze_y][maze_x] == 'g' {
                    game_state_manager.current_state = GameState::Victory;
                    // Reproducir sonido de victoria
                    if let Some(ref audio) = audio_manager {
                        let _ = audio.play_victory();
                    }
                }
            }
        } else {
            // Reset counter when not in Playing state
            frame_counter_since_playing = 0;
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
                // Controles de audio de Taylor Swift
                if rl.is_key_pressed(KeyboardKey::KEY_N) {
                    if let Some(ref mut audio) = audio_manager {
                        if let Err(e) = audio.next_track() {
                            println!("Error cambiando canción: {}", e);
                        }
                    }
                }
                if rl.is_key_pressed(KeyboardKey::KEY_P) {
                    if let Some(ref mut audio) = audio_manager {
                        if let Err(e) = audio.previous_track() {
                            println!("Error cambiando canción: {}", e);
                        }
                    }
                }
                if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                    if let Some(ref mut audio) = audio_manager {
                        audio.toggle_audio();
                    }
                }
                
                // TEST GAMEPAD - Presiona G para testear gamepad
                if rl.is_key_pressed(KeyboardKey::KEY_G) {
                    println!("\n🎮 === GAMEPAD TEST ===");
                    let gamepad_count = gilrs.gamepads().count();
                    
                    if gamepad_count == 0 {
                        println!("❌ No gamepads detected");
                    } else {
                        for (id, gamepad) in gilrs.gamepads() {
                            println!("🎮 Gamepad: {} (ID: {})", gamepad.name(), id);
                            println!("   Connected: {}", gamepad.is_connected());
                        }
                    }
                    println!("===================\n");
                }
                
                // 1. clear framebuffer
                framebuffer.clear();

                // 2. move the player on user input
                let old_pos = player.pos;
                process_events(&mut player, &rl, &maze, &mut gilrs);
                
                // Reproducir sonido de pasos si el jugador se movió
                if old_pos != player.pos {
                    if let Some(ref audio) = audio_manager {
                        let _ = audio.play_footstep();
                    }
                }

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
                let track_info = audio_manager.as_ref().map(|audio| audio.get_current_track_info());
                framebuffer.swap_buffers_with_fps_and_minimap(
                    &mut rl, 
                    &thread, 
                    current_fps, 
                    &player, 
                    &maze,
                    track_info.as_deref()
                );
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
                    // Player position will be reset automatically by transition detection above
                    game_state_manager.current_state = GameState::Playing;
                }
            }
        }

        thread::sleep(Duration::from_millis(16));
    }
}