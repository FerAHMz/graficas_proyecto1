pub struct Maze {
    pub map: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        println!("DEBUG: Intentando generar laberinto con Python {}x{}", width, height);
        
        // Primero intentar con el script de Python
        match Self::generate_with_python(width, height) {
            Ok(maze) => {
                println!("DEBUG: Laberinto generado exitosamente con Python");
                maze
            },
            Err(e) => {
                println!("WARNING: No se pudo generar con Python: {}", e);
                println!("DEBUG: Usando laberinto hardcodeado como fallback");
                Self::create_fallback_maze(width, height)
            }
        }
    }
    
    fn generate_with_python(width: usize, height: usize) -> Result<Self, String> {
        println!("DEBUG: Ejecutando comando: python3 src/maze.py json {} {}", width, height);
        
        // Obtener el directorio actual
        let current_dir = std::env::current_dir()
            .map_err(|e| format!("No se pudo obtener directorio actual: {}", e))?;
        println!("DEBUG: Directorio actual: {:?}", current_dir);
        
        // Verificar que el archivo existe
        let maze_py_path = current_dir.join("src").join("maze.py");
        println!("DEBUG: Buscando archivo en: {:?}", maze_py_path);
        
        if !maze_py_path.exists() {
            return Err(format!("El archivo maze.py no existe en: {:?}", maze_py_path));
        }
        
        // Intentar con timeout usando un approach más simple
        println!("DEBUG: Ejecutando comando Python...");
        
        let mut child = std::process::Command::new("python3")
            .arg(maze_py_path)
            .arg("json")
            .arg(width.to_string())
            .arg(height.to_string())
            .current_dir(&current_dir)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Error spawning comando: {}", e))?;
        
        println!("DEBUG: Proceso Python iniciado, esperando resultado...");
        
        // Usar wait en lugar de wait_with_output para simplificar
        let output = child.wait_with_output()
            .map_err(|e| format!("Error esperando proceso Python: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            return Err(format!("Python script failed with status: {}, stderr: {}, stdout: {}", 
                              output.status, stderr, stdout));
        }
        
        println!("DEBUG: Comando Python ejecutado exitosamente");
        
        let json_str = String::from_utf8(output.stdout)
            .map_err(|e| format!("Invalid UTF-8 in output: {}", e))?;
        
        println!("DEBUG: Respuesta JSON recibida, longitud: {} chars", json_str.len());
        if json_str.len() < 200 {
            println!("DEBUG: JSON content preview: {}", json_str);
        }
        
        let parsed: Vec<Vec<String>> = serde_json::from_str(&json_str)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;
        
        // Convertir String a char
        let map: Vec<Vec<char>> = parsed
            .into_iter()
            .map(|row| row.into_iter().map(|s| s.chars().next().unwrap_or(' ')).collect())
            .collect();
        
        println!("DEBUG: Laberinto convertido exitosamente, tamaño: {}x{}", map.len(), map.get(0).map_or(0, |row| row.len()));
        
        Ok(Maze {
            width: map[0].len(),
            height: map.len(),
            map,
        })
    }
    
    fn create_fallback_maze(width: usize, height: usize) -> Self {
        // Fallback: crear un laberinto simple hardcodeado
        let map = Self::create_simple_maze(width, height);
        
        Maze {
            width: map[0].len(),
            height: map.len(),
            map,
        }
    }
    
    fn create_simple_maze(width: usize, height: usize) -> Vec<Vec<char>> {
        let mut map = vec![vec![' '; width]; height];
        
        // Crear bordes
        for y in 0..height {
            for x in 0..width {
                if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                    map[y][x] = if (x == 0 || x == width - 1) && (y == 0 || y == height - 1) {
                        '+' // Esquinas
                    } else if y == 0 || y == height - 1 {
                        '-' // Bordes horizontales
                    } else {
                        '|' // Bordes verticales
                    };
                }
            }
        }
        
        // Agregar algunas paredes internas para hacer más interesante
        for y in 2..height-2 {
            for x in 2..width-2 {
                if (x % 4 == 0 && y % 3 == 0) || (x % 3 == 0 && y % 4 == 0) {
                    map[y][x] = '+';
                }
            }
        }
        
        // Colocar jugador y meta
        map[1][1] = 'p'; // Jugador en la esquina superior izquierda
        map[height-2][width-2] = 'g'; // Meta en la esquina inferior derecha
        
        map
    }
    
    pub fn get_cell(&self, x: usize, y: usize) -> char {
        if y < self.height && x < self.width {
            self.map[y][x]
        } else {
            '+' // Pared por defecto fuera de límites
        }
    }
    
    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        let cell = self.get_cell(x, y);
        cell == '+' || cell == '-' || cell == '|'
    }
    
    pub fn find_player_start(&self) -> Option<(usize, usize)> {
        for (y, row) in self.map.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == 'p' {
                    return Some((x, y));
                }
            }
        }
        None
    }
    
    pub fn find_goal(&self) -> Option<(usize, usize)> {
        for (y, row) in self.map.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == 'g' {
                    return Some((x, y));
                }
            }
        }
        None
    }
}
