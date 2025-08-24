use raylib::prelude::*;
use gilrs::{Gilrs, Button, Event, EventType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Welcome,     // Pantalla de bienvenida con temática Pokémon
    LevelSelect, // Selección de niveles temáticos
    Playing,
    Victory,     // Pantalla de éxito al completar nivel
}

pub struct GameStateManager {
    pub current_state: GameState,
    pub selected_level: usize,
    pub selected_menu_option: usize,
    pub menu_options: Vec<String>,
}

impl GameStateManager {
    pub fn new() -> Self {
        GameStateManager {
            current_state: GameState::Welcome,
            selected_level: 0,
            selected_menu_option: 0,
            menu_options: vec![
                "Iniciar Aventura".to_string(),
                "Seleccionar Región".to_string(),
                "Salir".to_string(),
            ],
        }
    }

    // Obtener el tamaño del laberinto según el nivel seleccionado
    pub fn get_maze_size(&self) -> (usize, usize) {
        match self.selected_level {
            0 => (4, 4),   // Fácil: 4x4
            1 => (8, 8),   // Medio: 8x8  
            2 => (12, 12), // Difícil: 12x12
            _ => (8, 8),   // Por defecto: 8x8
        }
    }

    // Obtener información del nivel actual
    pub fn get_level_info(&self) -> (&str, &str, &str) {
        match self.selected_level {
            0 => ("Centro Pokémon", "Un laberinto básico para entrenadores novatos", "⭐ FÁCIL"),
            1 => ("Cueva Oscura", "Laberinto medio con obstáculos adicionales", "⭐⭐ MEDIO"),
            2 => ("Torre Victoria", "El desafío final para maestros Pokémon", "⭐⭐⭐ DIFÍCIL"),
            _ => ("Cueva Oscura", "Laberinto medio con obstáculos adicionales", "⭐⭐ MEDIO"),
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, gilrs: &mut Gilrs) {
        match self.current_state {
            GameState::Welcome => self.update_welcome(rl, gilrs),
            GameState::LevelSelect => self.update_level_select(rl, gilrs),
            GameState::Victory => self.update_victory(rl, gilrs),
            _ => {}
        }
    }

    fn update_welcome(&mut self, rl: &mut RaylibHandle, gilrs: &mut Gilrs) {
        // Procesar eventos de gamepad
        while let Some(Event { id, event, time: _ }) = gilrs.next_event() {
            match event {
                EventType::ButtonPressed(button, _) => {
                    match button {
                        Button::South => { // X en PS4 (Cross) - ACCEPT
                            self.current_state = GameState::LevelSelect;
                            return;
                        },
                        Button::Start => { // Options en PS4 - MENU
                            self.current_state = GameState::LevelSelect;
                            return;
                        },
                        _ => {}
                    }
                }
                EventType::Connected => {
                    println!("🎮 Controller connected: {}", gilrs.gamepad(id).name());
                }
                EventType::Disconnected => {
                    println!("🎮 Controller disconnected");
                }
                _ => {}
            }
        }
        
        // Verificar estado actual de botones - REMOVIDO para evitar doble activación
        for (_id, gamepad) in gilrs.gamepads() {
            // Los botones se procesan solo en eventos ButtonPressed arriba
        }
        
        // Controles de teclado
        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) || rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            self.current_state = GameState::LevelSelect;
        }
    }

    fn update_level_select(&mut self, rl: &mut RaylibHandle, gilrs: &mut Gilrs) {
        // Procesar eventos de gamepad
        while let Some(Event { id: _, event, time: _ }) = gilrs.next_event() {
            match event {
                EventType::ButtonPressed(button, _) => {
                    match button {
                        Button::DPadLeft => {
                            if self.selected_level > 0 {
                                self.selected_level -= 1;
                                println!("🎮 Level: {}", self.selected_level);
                            }
                        },
                        Button::DPadRight => {
                            if self.selected_level < 2 {
                                self.selected_level += 1;
                                println!("🎮 Level: {}", self.selected_level);
                            }
                        },
                        Button::DPadUp => {
                            if self.selected_level > 0 {
                                self.selected_level -= 1;
                                println!("🎮 Level up: {}", self.selected_level);
                            }
                        },
                        Button::DPadDown => {
                            if self.selected_level < 2 {
                                self.selected_level += 1;
                                println!("🎮 Level down: {}", self.selected_level);
                            }
                        },
                        Button::South => { // X en PS4 - ACCEPT
                            self.current_state = GameState::Playing;
                            println!("🎮 Starting game");
                            return;
                        },
                        Button::East => { // Circle en PS4 - BACK
                            self.current_state = GameState::Welcome;
                            println!("🎮 Back to menu");
                            return;
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        
        // Verificar estado actual de botones solo para navegación continua
        for (_id, gamepad) in gilrs.gamepads() {
            // REMOVIDO: No verificar South/East aquí para evitar doble activación
            // Solo se procesan en los eventos ButtonPressed arriba
        }
        
        // Controles de teclado
        if rl.is_key_pressed(KeyboardKey::KEY_LEFT) && self.selected_level > 0 {
            self.selected_level -= 1;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) && self.selected_level < 2 {
            self.selected_level += 1;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_UP) && self.selected_level > 0 {
            self.selected_level -= 1;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) && self.selected_level < 2 {
            self.selected_level += 1;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
            self.current_state = GameState::Playing;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            self.current_state = GameState::Welcome;
        }
    }

    fn update_victory(&mut self, rl: &mut RaylibHandle, gilrs: &mut Gilrs) {
        // Procesar eventos de gamepad
        while let Some(Event { id: _, event, time: _ }) = gilrs.next_event() {
            match event {
                EventType::ButtonPressed(button, _) => {
                    match button {
                        Button::East => { // Circle en PS4 - BACK TO MENU
                            self.current_state = GameState::Welcome;
                            return;
                        },
                        Button::South => { // X en PS4 - RESTART LEVEL
                            self.current_state = GameState::Playing;
                            return;
                        },
                        Button::North => { // Triangle en PS4 - RESTART LEVEL
                            self.current_state = GameState::Playing;
                            return;
                        },
                        Button::Select => { // Share en PS4 - BACK TO MENU
                            self.current_state = GameState::Welcome;
                            return;
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        
        // Verificar estado actual de botones - REMOVIDO para evitar doble activación
        for (_id, gamepad) in gilrs.gamepads() {
            // Los botones se procesan solo en eventos ButtonPressed arriba
        }
        
        // Controles de teclado
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            self.current_state = GameState::Welcome;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            self.current_state = GameState::Playing;
        }
    }

    pub fn draw_welcome(&self, d: &mut RaylibDrawHandle) {
        self.draw_background(d, Color::new(25, 42, 86, 255));
        self.draw_title(d, "POKÉMON RAYCASTER", 202, 152, 48, Color::BLACK, Color::new(255, 204, 51, 255));
        d.draw_text("Aventura en el Laberinto", 240, 210, 24, Color::WHITE);
        d.draw_text("¡Bienvenido, joven entrenador!", 220, 280, 24, Color::new(255, 204, 51, 255));
        d.draw_text("Tu aventura en el mundo Pokémon está a punto de comenzar.", 120, 320, 18, Color::WHITE);
        d.draw_text("Explora laberintos místicos y encuentra todos los Pokémon.", 130, 350, 18, Color::WHITE);

        self.draw_controls(d);
        self.draw_pokeball_decorations(d);
        d.draw_text("Presiona ESPACIO o ENTER para comenzar", 180, 540, 20, Color::new(255, 204, 51, 255));
    }

    pub fn draw_level_select(&self, d: &mut RaylibDrawHandle) {
        self.draw_background(d, Color::new(25, 42, 86, 255));
        self.draw_title(d, "SELECCIONAR REGIÓN", 252, 102, 32, Color::BLACK, Color::new(255, 204, 51, 255));

        let levels = ["Centro Pokémon (4x4)", "Cueva Oscura (8x8)", "Torre Victoria (12x12)"];
        let descriptions = [
            "Un laberinto básico para entrenadores novatos - Pequeño y manejable",
            "Laberinto medio con obstáculos adicionales - Tamaño estándar",
            "El desafío final para maestros Pokémon - Extenso y complejo",
        ];
        let difficulties = ["⭐ FÁCIL", "⭐⭐ MEDIO", "⭐⭐⭐ DIFÍCIL"];

        for (i, ((level, desc), diff)) in levels.iter().zip(descriptions.iter()).zip(difficulties.iter()).enumerate() {
            let y = 180 + (i * 100) as i32;
            let is_selected = i == self.selected_level;

            self.draw_level_option(d, i, y, level, desc, diff, is_selected);
        }

        d.draw_rectangle(100, 520, 600, 60, Color::new(50, 50, 50, 200));
        d.draw_text("← → para cambiar región, ENTER para comenzar aventura", 120, 535, 18, Color::WHITE);
        d.draw_text("ESC para volver al menú principal", 250, 555, 16, Color::LIGHTGRAY);
    }

    pub fn draw_victory(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(0, 0, 800, 600, Color::new(0, 0, 0, 180));
        d.draw_rectangle(150, 120, 500, 360, Color::new(25, 42, 86, 255));
        d.draw_rectangle_lines_ex(Rectangle::new(150.0, 120.0, 500.0, 360.0), 5.0, Color::GOLD);
        d.draw_rectangle_lines_ex(Rectangle::new(155.0, 125.0, 490.0, 350.0), 2.0, Color::new(255, 204, 51, 255));

        self.draw_title(d, "¡VICTORIA!", 302, 162, 48, Color::BLACK, Color::GOLD);
        d.draw_text("¡Has completado la región!", 220, 220, 24, Color::WHITE);
        d.draw_text("¡Eres un verdadero", 280, 260, 20, Color::new(255, 204, 51, 255));
        d.draw_text("Maestro Pokémon!", 300, 280, 20, Color::new(255, 204, 51, 255));

        self.draw_statistics(d);

        d.draw_rectangle(200, 420, 400, 40, Color::new(50, 50, 50, 200));
        d.draw_text("ESC: Volver al menú   |   R: Jugar de nuevo", 220, 435, 16, Color::WHITE);

        self.draw_trophy_decorations(d);
        self.draw_particle_effects(d);
    }

    fn draw_background(&self, d: &mut RaylibDrawHandle, color: Color) {
        d.clear_background(color);
        d.draw_rectangle(0, 0, 800, 100, Color::new(255, 204, 51, 255)); // Amarillo Pokémon
        d.draw_rectangle(0, 90, 800, 20, Color::new(255, 165, 0, 255)); // Naranja
    }

    fn draw_title(&self, d: &mut RaylibDrawHandle, title: &str, x: i32, y: i32, size: i32, shadow_color: Color, text_color: Color) {
        d.draw_text(title, x, y, size, shadow_color); // Sombra
        d.draw_text(title, x - 2, y - 2, size, text_color); // Texto
    }

    fn draw_controls(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(150, 420, 500, 80, Color::new(50, 50, 50, 200));
        d.draw_text("CONTROLES:", 170, 435, 20, Color::new(255, 204, 51, 255));
        d.draw_text("• WASD o Flechas: Mover    • Mouse: Rotar cámara", 170, 460, 16, Color::WHITE);
        d.draw_text("• M: Cambiar vista 2D/3D   • C: Mostrar/Ocultar cursor", 170, 480, 16, Color::WHITE);
    }

    fn draw_pokeball_decorations(&self, d: &mut RaylibDrawHandle) {        d.draw_circle(100, 300, 15.0, Color::RED);
        d.draw_circle(100, 300, 12.0, Color::WHITE);
        d.draw_circle(100, 300, 5.0, Color::BLACK);
        
        d.draw_circle(700, 300, 15.0, Color::RED);
        d.draw_circle(700, 300, 12.0, Color::WHITE);
        d.draw_circle(700, 300, 5.0, Color::BLACK);
    }

    fn draw_level_option(&self, d: &mut RaylibDrawHandle, i: usize, y: i32, level: &str, desc: &str, diff: &str, is_selected: bool) {
        // Fondo del nivel seleccionado
        if is_selected {
            d.draw_rectangle(80, y - 10, 640, 90, Color::new(255, 204, 51, 100));
            d.draw_rectangle_lines_ex(Rectangle::new(80.0, (y - 10) as f32, 640.0, 90.0), 3.0, Color::new(255, 204, 51, 255));
        }

        let text_color = if is_selected { Color::new(255, 204, 51, 255) } else { Color::WHITE };
        let prefix = if is_selected { "► " } else { "  " };

        // Nombre del nivel
        d.draw_text(&format!("{}{}", prefix, level), 100, y, 28, text_color);

        // Descripción
        d.draw_text(desc, 120, y + 30, 16, Color::LIGHTGRAY);

        // Dificultad
        d.draw_text(diff, 120, y + 50, 14, if is_selected { Color::ORANGE } else { Color::GRAY });

        // Decoración tipo badge
        if is_selected {
            d.draw_circle(650, y + 25, 20.0, Color::new(255, 204, 51, 200));
            d.draw_circle(650, y + 25, 15.0, Color::new(255, 165, 0, 255));
            d.draw_text("✓", 645, y + 18, 20, Color::WHITE);
        }
    }

    fn draw_statistics(&self, d: &mut RaylibDrawHandle) {
        d.draw_text("Estadísticas:", 200, 320, 18, Color::LIGHTGRAY);
        d.draw_text("• Tiempo: --:--", 220, 345, 16, Color::WHITE);
        d.draw_text("• Pokémon encontrados: ✓", 220, 365, 16, Color::WHITE);
        d.draw_text("• Región completada: ✓", 220, 385, 16, Color::WHITE);
    }

    fn draw_trophy_decorations(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle(180, 180, 25.0, Color::GOLD);
        d.draw_circle(180, 180, 20.0, Color::YELLOW);
        d.draw_text("🏆", 170, 165, 30, Color::WHITE);

        d.draw_circle(620, 180, 25.0, Color::GOLD);
        d.draw_circle(620, 180, 20.0, Color::YELLOW);
        d.draw_text("⭐", 610, 165, 30, Color::WHITE);
    }

    fn draw_particle_effects(&self, d: &mut RaylibDrawHandle) {
        for i in 0..8 {
            let angle = (i as f32 * 45.0).to_radians();
            let radius = 80.0;
            let x = (400.0 + angle.cos() * radius) as i32;
            let y = (300.0 + angle.sin() * radius) as i32;
            d.draw_circle(x, y, 3.0, Color::new(255, 204, 51, 150));
        }
    }
}
