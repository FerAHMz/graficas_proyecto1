mod framebuffer;
mod player;
mod textures;
mod raycasting;
mod render;
mod maze;
mod audio;
mod game_state;
mod sprites;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use player::Player;
use textures::TextureManager;
use render::{render_2d, render_3d};
use crate::maze::Maze;
use crate::audio::AudioManager;
use crate::game_state::{GameState, GameStateManager};
use crate::sprites::SpriteManager;
use std::time::Instant;

fn main() {
    // Inicializar Raylib
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Pokémon Raycaster - Escape el Laberinto!")
        .build();
    
    rl.set_target_fps(60);
    rl.disable_cursor(); // Para captura del mouse

    // Inicializar managers
    let texture_manager = TextureManager::new(&mut rl, &thread);
    // let mut audio_manager = AudioManager::new(&mut rl, &thread); // Comentado para debugging
    let mut game_state_manager = GameStateManager::new();
    let mut sprite_manager = SpriteManager::new();

    // Comenzar directamente en el juego para debugging
    game_state_manager.current_state = GameState::Playing;

    // Variables del juego
    let mut maze_obj = Maze::new(8, 6);
    let mut maze = &maze_obj.map;
    
    let mut player = create_player_from_maze(&maze_obj);
    
    let mut framebuffer = Framebuffer::new(800, 600, Color::BLACK);
    let mut current_level = 0; // Track current level to avoid regenerating maze
    
    // Variables para FPS
    let mut frame_count = 0;
    let mut fps_timer = Instant::now();
    let mut current_fps = 0.0;
    
    // Variables del juego
    let mut show_2d = false;
    let mut game_won = false;
    let mut goal_pos = maze_obj.find_goal().unwrap_or((maze_obj.width - 2, maze_obj.height - 2));
    let mut victory_sound_played = false;
    let mut last_movement_time = Instant::now();

    println!("¡Bienvenido al Pokémon Raycaster!");

    // Bucle principal del juego
    while !rl.window_should_close() {
        // Actualizar FPS
        frame_count += 1;
        if fps_timer.elapsed().as_secs_f32() >= 1.0 {
            current_fps = frame_count as f32 / fps_timer.elapsed().as_secs_f32();
            frame_count = 0;
            fps_timer = Instant::now();
        }
        
        // Actualizar el manager de estado del juego
        game_state_manager.update(&mut rl);
        
        // Manejar cambio de nivel SOLO cuando es necesario
        if game_state_manager.current_state == GameState::Playing && 
           current_level != game_state_manager.selected_level {
            // Generar nuevo laberinto solo cuando el nivel cambió
            let level_sizes = [(8, 6), (10, 8), (12, 10)];
            let (w, h) = level_sizes[game_state_manager.selected_level.min(2)];
            
            println!("Cambiando a nivel {}: {}x{}", game_state_manager.selected_level, w, h);
            maze_obj = Maze::new(w, h);
            maze = &maze_obj.map;
            player = create_player_from_maze(&maze_obj);
            goal_pos = maze_obj.find_goal().unwrap_or((maze_obj.width - 2, maze_obj.height - 2));
            game_won = false;
            victory_sound_played = false;
            current_level = game_state_manager.selected_level; // Update current level
        }
        
        // Actualizar sprites
        sprite_manager.update();
        
        // Actualizar audio
        // audio_manager.update_music(&mut rl); // Comentado para debugging
        
        match game_state_manager.current_state {
            GameState::Playing => {
                // Actualizar jugador
                if !game_won {
                    let old_pos = player.pos;
                    player.update_keyboard(&mut rl);
                    player.update_mouse(&mut rl); // Rotación con mouse activada
                    
                    // Verificar colisiones con paredes - mejorado
                    let new_map_x = (player.pos.x / 50.0) as usize;
                    let new_map_y = (player.pos.y / 50.0) as usize;
                    
                    // Verificar límites del mapa
                    if new_map_x >= maze_obj.width || new_map_y >= maze_obj.height || 
                       new_map_x == 0 || new_map_y == 0 ||
                       maze_obj.is_wall(new_map_x, new_map_y) {
                        // Retroceder si estamos en una pared o fuera de límites
                        player.pos = old_pos;
                    }
                    
                    // Sonido de pasos cuando se mueve
                    if (player.pos.x - old_pos.x).abs() > 1.0 || (player.pos.y - old_pos.y).abs() > 1.0 {
                        if last_movement_time.elapsed().as_millis() > 300 {
                            // println!("👟 *paso*"); // Simulación de sonido - comentado para reducir spam
                            last_movement_time = Instant::now();
                        }
                    }
                    
                    // Verificar si llegamos a la meta
                    if new_map_x == goal_pos.0 && new_map_y == goal_pos.1 {
                        game_won = true;
                        if !victory_sound_played {
                            println!("🎉 ¡VICTORIA!");
                            victory_sound_played = true;
                        }
                        game_state_manager.current_state = GameState::Victory;
                    }
                }
                
                // Cambiar vista
                if rl.is_key_pressed(KeyboardKey::KEY_M) {
                    show_2d = !show_2d;
                }

                // Limpiar framebuffer
                framebuffer.clear();

                // Renderizar
                if show_2d {
                    render_2d(&mut framebuffer, &player, maze);
                } else {
                    render_3d(&mut framebuffer, &player, maze, &texture_manager);
                }
                
                // Dibujar sprites animados
                sprite_manager.draw(&mut framebuffer);

                // Verificar si se presiona ESC antes del drawing
                let escape_pressed = rl.is_key_pressed(KeyboardKey::KEY_ESCAPE);

                // Dibujar en pantalla - OPTIMIZADO
                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::BLACK);
                
                // Usar el nuevo método optimizado de framebuffer que reutiliza la textura
                framebuffer.draw_to_screen(&mut d, &thread);
                
                // Dibujar FPS
                let fps_color = if current_fps >= 15.0 { Color::GREEN } else { Color::RED };
                d.draw_text(&format!("FPS: {:.1}", current_fps), 10, 10, 20, fps_color);
                
                // Dibujar minimapa (esquina superior derecha)
                draw_minimap(&mut d, &player, maze, 600, 10, 180, 120);
                
                // Mostrar controles
                d.draw_text("WASD: Mover | Mouse: Rotar | M: Vista 2D | ESC: Menú", 10, 570, 14, Color::WHITE);
                
                // Información de debug
                d.draw_text(&format!("Pos: ({:.1}, {:.1})", player.pos.x, player.pos.y), 10, 40, 14, Color::YELLOW);
                d.draw_text(&format!("Mapa: ({}, {})", (player.pos.x / 50.0) as usize, (player.pos.y / 50.0) as usize), 10, 60, 14, Color::YELLOW);
                
                // Finalizar drawing antes de verificar ESC
                drop(d);
                
                if escape_pressed {
                    game_state_manager.current_state = GameState::MainMenu;
                }
            },
            
            GameState::MainMenu => {
                let mut d = rl.begin_drawing(&thread);
                game_state_manager.draw_main_menu(&mut d);
            },
            
            GameState::LevelSelect => {
                let mut d = rl.begin_drawing(&thread);
                game_state_manager.draw_level_select(&mut d);
            },
            
            GameState::Victory => {
                // Dibujar el juego de fondo
                framebuffer.clear();
                if show_2d {
                    render_2d(&mut framebuffer, &player, maze);
                } else {
                    render_3d(&mut framebuffer, &player, maze, &texture_manager);
                }
                sprite_manager.draw(&mut framebuffer);
                
                // Manejar input de victoria primero
                let escape_pressed = rl.is_key_pressed(KeyboardKey::KEY_ESCAPE);
                let r_pressed = rl.is_key_pressed(KeyboardKey::KEY_R);
                
                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::BLACK);
                
                // Usar el nuevo método optimizado de framebuffer que reutiliza la textura
                framebuffer.draw_to_screen(&mut d, &thread);
                
                // Dibujar pantalla de victoria encima
                game_state_manager.draw_victory(&mut d);
                
                drop(d); // Liberar el handle de dibujo antes de usar rl de nuevo
                
                if escape_pressed {
                    game_state_manager.current_state = GameState::MainMenu;
                    game_won = false;
                    victory_sound_played = false;
                }
                
                if r_pressed {
                    // Reiniciar nivel actual con el tamaño correcto
                    let level_sizes = [(8, 6), (10, 8), (12, 10)];
                    let (w, h) = level_sizes[game_state_manager.selected_level.min(2)];
                    maze_obj = Maze::new(w, h);
                    maze = &maze_obj.map;
                    player = create_player_from_maze(&maze_obj);
                    goal_pos = maze_obj.find_goal().unwrap_or((maze_obj.width - 2, maze_obj.height - 2));
                    game_won = false;
                    victory_sound_played = false;
                    game_state_manager.current_state = GameState::Playing;
                }
            },
        }
    }
}

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
    let end_x = player_x + (player.angle.cos() * 12.0) as i32;
    let end_y = player_y + (player.angle.sin() * 12.0) as i32;
    d.draw_line(player_x, player_y, end_x, end_y, Color::YELLOW);
}