#![allow(unused_imports)]
#![allow(dead_code)]

mod line;
mod framebuffer;
mod maze;
mod caster;
mod player;
mod render;
mod textures;

use line::line;
use maze::Maze;
use caster::{cast_ray, Intersect};
use framebuffer::Framebuffer;
use player::{Player, process_events};
use render::{render_2d, render_3d};
use textures::TextureManager;

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
        
        // 1. clear framebuffer
        framebuffer.clear();

        // 2. move the player on user input
        process_events(&mut player, &rl, &maze);

        // 3. toggle between 2D and 3D mode
        if rl.is_key_pressed(KeyboardKey::KEY_M) {
            mode_3d = !mode_3d;
        }

        // Toggle mouse control
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            mouse_enabled = !mouse_enabled;
            if mouse_enabled {
                rl.disable_cursor();
            } else {
                rl.enable_cursor();
            }
        }

        // 4. draw stuff
        if mode_3d {
            render_3d(&mut framebuffer, &player, &maze, &texture_manager);
        } else {
            render_2d(&mut framebuffer, &player, &maze);
        }

        // 5. swap buffers with FPS display and minimap
        framebuffer.swap_buffers_with_fps_and_minimap(&mut rl, &thread, current_fps, &player, &maze);

        thread::sleep(Duration::from_millis(16));
    }
}

/*
fn create_player_from_maze(maze_obj: &Maze) -> Player {
    let start_pos = maze_obj.find_player_start().unwrap_or((1, 1));
    Player::new(
        Vector2::new(start_pos.0 as f32 * 50.0 + 25.0, start_pos.1 as f32 * 50.0 + 25.0), 
        std::f32::consts::PI / 4.0, 
        std::f32::consts::PI / 3.0
    )
}

fn draw_minimap(d: &mut RaylibDrawHandle, player: &Player, maze: &Vec<Vec<char>>, x: i32, y: i32, width: i32, height: i32) {
    let scale_x = width as f32 / maze[0].len() as f32;
    let scale_y = height as f32 / maze.len() as f32;
    
    // Fondo del minimapa
    d.draw_rectangle(x, y, width, height, Color::new(0, 0, 0, 150));
    d.draw_rectangle_lines(x, y, width, height, Color::YELLOW);
    
    // Título del minimapa
    d.draw_text("MAPA", x + 5, y - 20, 14, Color::YELLOW);
    
    // Dibujar el mapa
    for (row_idx, row) in maze.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            let rect_x = x + (col_idx as f32 * scale_x) as i32;
            let rect_y = y + (row_idx as f32 * scale_y) as i32;
            let rect_w = scale_x.max(1.0) as i32;
            let rect_h = scale_y.max(1.0) as i32;
            
            let color = match cell {
                '+' | '-' | '|' => Color::BROWN,
                'g' => Color::GOLD,
                _ => Color::new(34, 139, 34, 100), // Verde translúcido para espacios abiertos
            };
            
            if cell != ' ' {
                d.draw_rectangle(rect_x, rect_y, rect_w, rect_h, color);
            }
        }
    }
    
    // Dibujar jugador
    let player_x = x + (player.pos.x / 50.0 * scale_x) as i32;
    let player_y = y + (player.pos.y / 50.0 * scale_y) as i32;
    d.draw_circle(player_x, player_y, 4.0, Color::RED);
    
    // Dibujar dirección del jugador
    let end_x = player_x + (player.a.cos() * 12.0) as i32;
    let end_y = player_y + (player.a.sin() * 12.0) as i32;
    d.draw_line(player_x, player_y, end_x, end_y, Color::YELLOW);
}
*/