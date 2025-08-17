use raylib::prelude::*;
use crate::player::Player;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, Color::BLACK);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color: Color::BLACK,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(self.width as i32, self.height as i32, self.background_color);
    }

    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            self.color_buffer.draw_pixel(x as i32, y as i32, self.current_color);
        }
    }

    pub fn set_pixel_with_color(&mut self, x: u32, y: u32, color: Color) {
        if x < self.width && y < self.height {
            self.color_buffer.draw_pixel(x as i32, y as i32, color);
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn _render_to_file(&self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }

    pub fn swap_buffers(
        &self,
        window: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
    ) {
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let mut renderer = window.begin_drawing(raylib_thread);
            renderer.clear_background(Color::BLACK);
            renderer.draw_texture(&texture, 0, 0, Color::WHITE);
        }
    }

    pub fn swap_buffers_with_fps(
        &self,
        window: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
        fps: f32,
    ) {
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let mut renderer = window.begin_drawing(raylib_thread);
            renderer.draw_texture(&texture, 0, 0, Color::WHITE);
            
            // Draw FPS counter with color coding
            let fps_color = if fps >= 15.0 { 
                Color::GREEN 
            } else if fps >= 10.0 { 
                Color::YELLOW 
            } else { 
                Color::RED 
            };
            let fps_text = format!("FPS: {:.1}", fps);
            renderer.draw_text(&fps_text, 10, 10, 20, fps_color);
            
            // Draw performance status
            let status = if fps >= 15.0 { 
                "GOOD" 
            } else if fps >= 10.0 { 
                "OK" 
            } else { 
                "LOW" 
            };
            renderer.draw_text(&format!("Performance: {}", status), 10, 35, 16, fps_color);
            
            // Draw controls info
            renderer.draw_text("M: Toggle 2D/3D | WASD/Arrows: Move", 10, self.height as i32 - 30, 16, Color::WHITE);
        }
    }

    pub fn swap_buffers_with_fps_and_minimap(
        &self,
        window: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
        fps: f32,
        player: &Player,
        maze: &Vec<Vec<char>>,
        current_track_info: Option<&str>,
    ) {
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let mut renderer = window.begin_drawing(raylib_thread);
            renderer.clear_background(Color::BLACK);
            renderer.draw_texture(&texture, 0, 0, Color::WHITE);
            
            // Draw FPS counter with color coding
            let fps_color = if fps >= 15.0 { 
                Color::GREEN 
            } else if fps >= 10.0 { 
                Color::YELLOW 
            } else { 
                Color::RED 
            };
            let fps_text = format!("FPS: {:.1}", fps);
            renderer.draw_text(&fps_text, 10, 10, 20, fps_color);
            
            // Draw performance status
            let status = if fps >= 15.0 { 
                "GOOD" 
            } else if fps >= 10.0 { 
                "OK" 
            } else { 
                "LOW" 
            };
            renderer.draw_text(&format!("Performance: {}", status), 10, 35, 16, fps_color);
            
            // Draw minimap in top-right corner
            self.draw_minimap(&mut renderer, player, maze);
            
            // Draw Taylor Swift music info
            if let Some(track_info) = current_track_info {
                renderer.draw_text("🎵 Now Playing:", 10, 60, 16, Color::new(255, 192, 203, 255)); // Pink
                renderer.draw_text(track_info, 10, 80, 14, Color::new(255, 105, 180, 255)); // Hot pink
            }
            
            // Draw music controls
            renderer.draw_text("N: Next Track | P: Previous | SPACE: Toggle Audio", 10, self.height as i32 - 50, 14, Color::new(200, 200, 200, 255));
            
            // Draw controls info
            renderer.draw_text("M: Toggle 2D/3D | WASD/Arrows: Move", 10, self.height as i32 - 30, 16, Color::WHITE);
        }
    }

    fn draw_minimap(&self, renderer: &mut RaylibDrawHandle, player: &Player, maze: &Vec<Vec<char>>) {
        let minimap_width = 160;
        let minimap_height = 120;
        let minimap_x = self.width as i32 - minimap_width - 10; // 10 pixels from right edge
        let minimap_y = 10; // 10 pixels from top

        let scale_x = minimap_width as f32 / maze[0].len() as f32;
        let scale_y = minimap_height as f32 / maze.len() as f32;
        
        // Draw minimap background
        renderer.draw_rectangle(minimap_x, minimap_y, minimap_width, minimap_height, Color::new(0, 0, 0, 150));
        renderer.draw_rectangle_lines(minimap_x, minimap_y, minimap_width, minimap_height, Color::YELLOW);
        
        // Draw minimap title
        renderer.draw_text("MINIMAP", minimap_x + 5, minimap_y - 20, 14, Color::YELLOW);
        
        // Draw the maze
        for (row_idx, row) in maze.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                let rect_x = minimap_x + (col_idx as f32 * scale_x) as i32;
                let rect_y = minimap_y + (row_idx as f32 * scale_y) as i32;
                let rect_w = scale_x.max(1.0) as i32;
                let rect_h = scale_y.max(1.0) as i32;
                
                let color = match cell {
                    '+' | '-' | '|' => Color::BROWN,
                    'g' => Color::GOLD,
                    's' => Color::LIME,
                    _ => Color::new(34, 139, 34, 100), // Green translucent for open spaces
                };
                
                if cell != ' ' {
                    renderer.draw_rectangle(rect_x, rect_y, rect_w, rect_h, color);
                }
            }
        }
        
        // Draw player position (using correct world block size)
        let world_block_size = 20.0; // Must match the block size used in player.rs and rendering
        let player_x = minimap_x + (player.pos.x / world_block_size * scale_x) as i32;
        let player_y = minimap_y + (player.pos.y / world_block_size * scale_y) as i32;
        renderer.draw_circle(player_x, player_y, 4.0, Color::RED);
        
        // Draw player direction indicator
        let dir_length = 12.0;
        let end_x = player_x + (player.a.cos() * dir_length) as i32;
        let end_y = player_y + (player.a.sin() * dir_length) as i32;
        renderer.draw_line(player_x, player_y, end_x, end_y, Color::YELLOW);

        // Draw player coordinates for debugging
        let coord_text = format!("({:.0}, {:.0})", player.pos.x / world_block_size, player.pos.y / world_block_size);
        renderer.draw_text(&coord_text, minimap_x, minimap_y + minimap_height + 5, 12, Color::WHITE);
    }

    // Simplified draw method for game states that don't need framebuffer rendering
    pub fn draw_ui_overlay(
        &self,
        d: &mut RaylibDrawHandle,
        fps: f32,
        player: &Player,
        maze: &Vec<Vec<char>>,
    ) {
        // Draw FPS counter with color coding
        let fps_color = if fps >= 15.0 { 
            Color::GREEN 
        } else if fps >= 10.0 { 
            Color::YELLOW 
        } else { 
            Color::RED 
        };
        let fps_text = format!("FPS: {:.1}", fps);
        d.draw_text(&fps_text, 10, 10, 20, fps_color);
        
        // Draw performance status
        let status = if fps >= 15.0 { 
            "GOOD" 
        } else if fps >= 10.0 { 
            "OK" 
        } else { 
            "LOW" 
        };
        d.draw_text(&format!("Performance: {}", status), 10, 35, 16, fps_color);
        
        // Draw minimap in top-right corner
        self.draw_minimap_to_handle(d, player, maze);
        
        // Draw controls info
        d.draw_text("M: Toggle 2D/3D | WASD/Arrows: Move | C: Toggle Cursor | ESC: Menu", 10, self.height as i32 - 30, 14, Color::WHITE);
    }

    fn draw_minimap_to_handle(&self, d: &mut RaylibDrawHandle, player: &Player, maze: &Vec<Vec<char>>) {
        let minimap_size = 8; // Size of each cell in the minimap
        let minimap_width = maze[0].len() * minimap_size;
        let minimap_height = maze.len() * minimap_size;
        let minimap_x = self.width as i32 - minimap_width as i32 - 10;
        let minimap_y = 60;
        let world_block_size = 20.0;

        // Draw minimap background
        d.draw_rectangle(minimap_x - 2, minimap_y - 2, minimap_width as i32 + 4, minimap_height as i32 + 4, Color::BLACK);

        // Draw maze cells
        for (row_index, row) in maze.iter().enumerate() {
            for (col_index, &cell) in row.iter().enumerate() {
                let x = minimap_x + (col_index * minimap_size) as i32;
                let y = minimap_y + (row_index * minimap_size) as i32;

                let color = match cell {
                    '+' | '-' | '|' => Color::BROWN,    // Walls
                    'g' => Color::GOLD,                  // Goal
                    's' => Color::LIME,                  // Start
                    _ => Color::new(200, 200, 200, 255), // Empty space
                };

                d.draw_rectangle(x, y, minimap_size as i32, minimap_size as i32, color);
            }
        }

        // Draw player position and direction
        let player_minimap_x = minimap_x + (player.pos.x / world_block_size * minimap_size as f32) as i32;
        let player_minimap_y = minimap_y + (player.pos.y / world_block_size * minimap_size as f32) as i32;

        // Player dot
        d.draw_circle(player_minimap_x, player_minimap_y, 3.0, Color::RED);

        // Direction line
        let line_length = 15.0;
        let end_x = player_minimap_x as f32 + player.a.cos() * line_length;
        let end_y = player_minimap_y as f32 + player.a.sin() * line_length;
        d.draw_line(player_minimap_x, player_minimap_y, end_x as i32, end_y as i32, Color::YELLOW);

        // Draw player coordinates for debugging
        let coord_text = format!("({:.0}, {:.0})", player.pos.x / world_block_size, player.pos.y / world_block_size);
        d.draw_text(&coord_text, minimap_x, minimap_y + minimap_height as i32 + 5, 12, Color::WHITE);
    }
}