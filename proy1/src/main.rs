mod framebuffer;
mod player;
mod textures;
mod raycasting;
mod render;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use player::Player;
use textures::TextureManager;
use render::{render_2d, render_3d};

fn main() {
    // Inicializar Raylib
    let (mut rl, thread) = raylib::init().size(800, 600).title("Raycaster Game").build();

    // Cargar texturas
    let mut texture_manager = TextureManager::new(&mut rl, &thread);

    // Cargar laberinto (simple ejemplo)
    let maze = vec![
        vec!['#', '#', '#', '#', '#'],
        vec!['#', ' ', ' ', ' ', '#'],
        vec!['#', ' ', '#', ' ', '#'],
        vec!['#', ' ', ' ', ' ', '#'],
        vec!['#', '#', '#', '#', '#'],
    ];

    // Inicializar jugador
    let mut player = Player::new(Vector2::new(150.0, 150.0), std::f32::consts::PI / 3.0, std::f32::consts::PI / 3.0);

    // Inicializar framebuffer
    let mut framebuffer = Framebuffer::new(800, 600, Color::BLACK);

    // Bucle principal del juego
    while !rl.window_should_close() {
        // Actualizar jugador
        player.update_keyboard(&mut rl);

        // Limpiar framebuffer
        framebuffer.clear();

        // Renderizar el mundo en 2D o 3D
        if rl.is_key_down(KeyboardKey::KEY_M) {
            render_2d(&mut framebuffer, &player, &maze);
        } else {
            render_3d(&mut framebuffer, &player, &maze, &texture_manager);
        }

        // Dibujar el framebuffer en la ventana
        let mut d = rl.begin_drawing(&thread);

        // Convertir framebuffer a imagen y luego a textura
        let image = framebuffer.to_image();
        let texture = d.load_texture_from_image(&thread, &image).unwrap();

        // Dibujar la textura
        d.draw_texture(&texture, 0, 0, Color::WHITE);

        // El drawing handle se cierra automáticamente al salir del scope
    }
}