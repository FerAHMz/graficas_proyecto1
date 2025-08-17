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
        // Detectar el sistema operativo para usar el comando Python correcto
        let python_commands = if cfg!(windows) {
            vec!["python", "python3", "py"]  // En Windows, intentar python, python3, y py
        } else {
            vec!["python3", "python"]  // En Mac/Linux, intentar python3 primero, luego python
        };
        
        let current_dir = std::env::current_dir()
            .map_err(|e| format!("No se pudo obtener directorio actual: {}", e))?;
        
        let maze_py_path = current_dir.join("src").join("maze.py");
        
        if !maze_py_path.exists() {
            return Err(format!("El archivo maze.py no existe en: {:?}", maze_py_path));
        }
        
        // Intentar cada comando Python hasta que uno funcione
        for python_cmd in python_commands {
            println!("DEBUG: Intentando comando: {} src/maze.py json {} {}", python_cmd, width, height);
            
            let result = std::process::Command::new(python_cmd)
                .arg(&maze_py_path)
                .arg("json")
                .arg(width.to_string())
                .arg(height.to_string())
                .current_dir(&current_dir)
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn();
            
            let mut child = match result {
                Ok(child) => child,
                Err(e) => {
                    println!("DEBUG: {} no disponible: {}", python_cmd, e);
                    continue;  // Intentar el siguiente comando
                }
            };
            
            println!("DEBUG: Proceso Python iniciado con {}, esperando resultado...", python_cmd);
            
            // Usar wait en lugar de wait_with_output para simplificar
            let output = match child.wait_with_output() {
                Ok(output) => output,
                Err(e) => {
                    println!("DEBUG: Error ejecutando {}: {}", python_cmd, e);
                    continue;  // Intentar el siguiente comando
                }
            };
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("DEBUG: {} falló - status: {}, stderr: {}", python_cmd, output.status, stderr);
                continue;  // Intentar el siguiente comando
            }
            
            println!("DEBUG: Comando Python ejecutado exitosamente con {}", python_cmd);
            
            let json_str = match String::from_utf8(output.stdout) {
                Ok(s) => s,
                Err(e) => {
                    println!("DEBUG: Invalid UTF-8 en output de {}: {}", python_cmd, e);
                    continue;  // Intentar el siguiente comando
                }
            };
            
            // Si llegamos aquí, el comando fue exitoso, procesar el resultado
            return Self::parse_json_maze(&json_str);
        }
        
        // Si ningún comando Python funcionó
        Err("No se pudo ejecutar Python con ningún comando disponible (python, python3, py)".to_string())
    }
    
    fn parse_json_maze(json_str: &str) -> Result<Self, String> {
        println!("DEBUG: Respuesta JSON recibida, longitud: {} chars", json_str.len());
        if json_str.len() < 200 {
            println!("DEBUG: JSON content preview: {}", json_str);
        }
        
        let parsed: Vec<Vec<String>> = serde_json::from_str(json_str)
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
