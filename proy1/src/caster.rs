use raylib::color::Color;

use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub tx: f32, // Texture X coordinate (0.0 to 1.0)
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &Player,
    a: f32,
    block_size: usize,
    draw_line: bool,
) -> Intersect {
    let mut d = 0.0;

    framebuffer.set_current_color(Color::WHITESMOKE);

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();
        let x = player.pos.x + cos;
        let y = player.pos.y + sin;

        let i = (x / block_size as f32) as usize;
        let j = (y / block_size as f32) as usize;

        if j >= maze.len() || i >= maze[0].len() {
            return Intersect {
                distance: d,
                impact: '+',
                tx: 0.0,
            };
        }

        if maze[j][i] == '+' || maze[j][i] == '-' || maze[j][i] == '|' {
            // Calculate texture X coordinate
            let block_x = i as f32 * block_size as f32;
            let block_y = j as f32 * block_size as f32;
            
            let tx = if maze[j][i] == '|' {
                // Vertical wall: use Y position within the block
                (y - block_y) / block_size as f32
            } else {
                // Horizontal wall: use X position within the block
                (x - block_x) / block_size as f32
            };
            
            return Intersect {
                distance: d,
                impact: maze[j][i],
                tx: tx.fract(), // Keep only fractional part for 0.0-1.0 range
            };
        }

        if draw_line {
            framebuffer.set_pixel(x as u32, y as u32);
        }

        d += 1.0;
    }
}
