use raylib::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    MainMenu,
    LevelSelect,
    Playing,
    Victory,
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
            current_state: GameState::MainMenu,
            selected_level: 0,
            selected_menu_option: 0,
            menu_options: vec![
                "Jugar".to_string(),
                "Seleccionar Nivel".to_string(),
                "Salir".to_string(),
            ],
        }
    }
    
    pub fn update(&mut self, rl: &mut RaylibHandle) {
        match self.current_state {
            GameState::MainMenu => self.update_main_menu(rl),
            GameState::LevelSelect => self.update_level_select(rl),
            _ => {}
        }
    }
    
    fn update_main_menu(&mut self, rl: &mut RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_UP) && self.selected_menu_option > 0 {
            self.selected_menu_option -= 1;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) && self.selected_menu_option < self.menu_options.len() - 1 {
            self.selected_menu_option += 1;
        }
        
        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
            match self.selected_menu_option {
                0 => self.current_state = GameState::Playing,
                1 => self.current_state = GameState::LevelSelect,
                2 => std::process::exit(0),
                _ => {}
            }
        }
    }
    
    fn update_level_select(&mut self, rl: &mut RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_LEFT) && self.selected_level > 0 {
            self.selected_level -= 1;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) && self.selected_level < 2 {
            self.selected_level += 1;
        }
        
        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
            self.current_state = GameState::Playing;
        }
        
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            self.current_state = GameState::MainMenu;
        }
    }
    
    pub fn draw_main_menu(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::new(25, 42, 86, 255)); // Azul Pokémon
        
        // Título principal
        d.draw_text("POKÉMON RAYCASTER", 200, 100, 40, Color::YELLOW);
        d.draw_text("Escape del Laberinto", 250, 150, 24, Color::WHITE);
        
        // Opciones del menú
        for (i, option) in self.menu_options.iter().enumerate() {
            let y = 250 + (i * 50) as i32;
            let color = if i == self.selected_menu_option {
                Color::YELLOW
            } else {
                Color::WHITE
            };
            
            let prefix = if i == self.selected_menu_option { "> " } else { "  " };
            d.draw_text(&format!("{}{}", prefix, option), 350, y, 24, color);
        }
        
        // Instrucciones
        d.draw_text("Usa ↑↓ para navegar, ENTER para seleccionar", 200, 450, 16, Color::LIGHTGRAY);
    }
    
    pub fn draw_level_select(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::new(25, 42, 86, 255));
        
        d.draw_text("SELECCIONAR NIVEL", 250, 100, 32, Color::YELLOW);
        
        let levels = ["Centro Pokémon", "Cueva Oscura", "Torre Victoria"];
        let descriptions = [
            "Un laberinto básico para entrenadores novatos",
            "Laberinto medio con obstáculos adicionales", 
            "El desafío final para maestros Pokémon"
        ];
        
        for (i, (level, desc)) in levels.iter().zip(descriptions.iter()).enumerate() {
            let y = 200 + (i * 80) as i32;
            let color = if i == self.selected_level {
                Color::YELLOW
            } else {
                Color::WHITE
            };
            
            let prefix = if i == self.selected_level { "> " } else { "  " };
            d.draw_text(&format!("{}{}", prefix, level), 100, y, 24, color);
            d.draw_text(desc, 120, y + 25, 16, Color::LIGHTGRAY);
        }
        
        d.draw_text("← → para cambiar nivel, ENTER para jugar, ESC para volver", 150, 500, 16, Color::LIGHTGRAY);
    }
    
    pub fn draw_victory(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(0, 0, 800, 600, Color::new(0, 0, 0, 200));
        
        d.draw_text("¡VICTORIA!", 300, 200, 48, Color::GOLD);
        d.draw_text("¡Has capturado todos los Pokémon!", 200, 270, 24, Color::WHITE);
        d.draw_text("¡Eres un verdadero Maestro Pokémon!", 190, 300, 20, Color::YELLOW);
        
        d.draw_text("Presiona ESC para volver al menú", 250, 400, 18, Color::LIGHTGRAY);
        d.draw_text("Presiona R para jugar de nuevo", 260, 430, 18, Color::LIGHTGRAY);
    }
}
