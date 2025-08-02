use raylib::prelude::*;
use raylib::consts::{GamepadAxis, GamepadButton};

pub struct Player {
    pub pos: Vector2, // Posición del jugador
    pub a: f32,       // Ángulo de visión (hacia dónde está mirando el jugador)
    pub fov: f32,     // Campo de visión (field of view)
}

impl Player {
    // Crear un nuevo jugador con posición inicial y campo de visión
    pub fn new(start_pos: Vector2, start_angle: f32, fov: f32) -> Self {
        Player {
            pos: start_pos,
            a: start_angle,
            fov,
        }
    }

    // Mover al jugador hacia adelante (dependiendo del ángulo de visión)
    pub fn move_forward(&mut self, speed: f32) {
        self.pos.x += speed * self.a.cos(); // Mueve en la dirección del ángulo
        self.pos.y += speed * self.a.sin(); // Mueve en la dirección del ángulo
    }

    // Mover al jugador hacia atrás (dependiendo del ángulo de visión)
    pub fn move_backward(&mut self, speed: f32) {
        self.pos.x -= speed * self.a.cos();
        self.pos.y -= speed * self.a.sin();
    }

    // Rotar al jugador a la izquierda
    pub fn turn_left(&mut self, speed: f32) {
        self.a -= speed; // Reducir el ángulo (giro hacia la izquierda)
        if self.a < 0.0 {
            self.a += std::f32::consts::PI * 2.0; // Asegurarse de que el ángulo se mantenga en el rango [0, 2π]
        }
    }

    // Rotar al jugador a la derecha
    pub fn turn_right(&mut self, speed: f32) {
        self.a += speed; // Aumentar el ángulo (giro hacia la derecha)
        if self.a > std::f32::consts::PI * 2.0 {
            self.a -= std::f32::consts::PI * 2.0; // Asegurarse de que el ángulo se mantenga en el rango [0, 2π]
        }
    }

    // Control del jugador usando el teclado
    pub fn update_keyboard(&mut self, window: &mut RaylibHandle) {
        const MOVE_SPEED: f32 = 5.0;     // Velocidad de movimiento
        const ROTATION_SPEED: f32 = std::f32::consts::PI / 32.0; // Velocidad de rotación

        // Moverse hacia adelante
        if window.is_key_down(KeyboardKey::KEY_W) {
            self.move_forward(MOVE_SPEED);
        }

        // Moverse hacia atrás
        if window.is_key_down(KeyboardKey::KEY_S) {
            self.move_backward(MOVE_SPEED);
        }

        // Rotar a la izquierda
        if window.is_key_down(KeyboardKey::KEY_A) {
            self.turn_left(ROTATION_SPEED);
        }

        // Rotar a la derecha
        if window.is_key_down(KeyboardKey::KEY_D) {
            self.turn_right(ROTATION_SPEED);
        }
    }

    // Control del jugador usando un controlador (como PS4)
    pub fn update_gamepad(&mut self, window: &mut RaylibHandle) {
        const MOVE_SPEED: f32 = 5.0;
        const ROTATION_SPEED: f32 = std::f32::consts::PI / 32.0;

        // Usar el joystick para mover el jugador
        let move_x = window.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_X); // Updated to correct method
        let move_y = window.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_Y); // Updated to correct method

        if move_x != 0.0 {
            self.pos.x += move_x * MOVE_SPEED;
        }
        if move_y != 0.0 {
            self.pos.y += move_y * MOVE_SPEED;
        }

        // Usar los botones para rotar el jugador
        if window.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_LEFT_THUMB) { // Updated to valid variant
            self.turn_left(ROTATION_SPEED);
        }
        if window.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_RIGHT_THUMB) { // Updated to valid variant
            self.turn_right(ROTATION_SPEED);
        }
    }
}