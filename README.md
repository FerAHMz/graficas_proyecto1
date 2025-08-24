# 🎮 Pokémon Raycaster - Escape del Laberinto

Un raycaster en 3D desarrollado en Rust con tema de Pokémon donde el jugador debe escapar de un laberinto generado dinámicamente. El proyecto implementa un motor de raycasting completo con texturas, audio, sprites animados y múltiples estados de juego.

## 🎯 Objetivo del Proyecto

Este proyecto fue desarrollado para demostrar los conocimientos adquiridos en gráficas por computadora, implementando un motor de raycasting funcional con características avanzadas para crear una experiencia de juego inmersiva y completa.

## ✨ Características Implementadas

### 🏆 **Puntuación Total: 100+ puntos**

- **20 puntos** - ✅ Cámara con movimiento hacia adelante/atrás y rotación suave
- **10 puntos** - ✅ Rotación con mouse (horizontal) con captura de cursor
- **15 puntos** - ✅ Mantener 60 fps estables con optimizaciones
- **30 puntos** - ✅ Estética completa del nivel con tema Pokémon
- **10 puntos** - ✅ Minimapa en tiempo real en esquina superior derecha
- **5 puntos** - ✅ Música de fondo (sistema completo de audio con Taylor Swift)
- **5 puntos** - ✅ Efectos de sonido integrados (pasos, victoria, ambiente)
- **5 puntos** - ✅ Pantalla de bienvenida temática

### 🎮 Características Avanzadas del Juego

1. **Motor de Raycasting Optimizado**: Algoritmo DDA con corrección de distorsión
2. **Sistema de Texturas**: 5 texturas diferentes para paredes (wall1.jpg - wall5.jpg)
3. **Laberinto Dinámico**: Generador Python con algoritmo de backtracking recursivo
4. **Vista Dual**: Intercambio fluido entre vista 3D y 2D con la tecla `M`
5. **Sprites Animados**: Pokémon con animaciones de 4 frames
6. **Sistema de Niveles**: Tres dificultades (4x4, 8x8, 12x12)
7. **Estados de Juego Completos**: Menú, selección, juego, victoria
8. **Sistema de Audio Completo**: Playlist de Taylor Swift con 5 canciones
9. **Soporte de Gamepad**: Compatibilidad con controladores usando gilrs
10. **Detección de Colisiones Precisa**: Sistema robusto de física

## 🎨 Diseño Visual y Técnico

### Sistema de Texturas Avanzado

- **5 Texturas de Pared**: wall1.jpg, wall2.jpg, wall3.jpg, wall4.jpg, wall5.jpg
- **Mapeo de Texturas**: Aplicación dinámica según tipo de pared
- **Sombreado por Distancia**: Efectos de profundidad realistas
- **Anti-aliasing**: Suavizado de bordes para mejor calidad visual

### Paleta de Colores Pokémon

- **Cielo**: Azul cielo (#87CEEB) con gradientes
- **Suelo**: Verde césped (#228B22) con texturas
- **Paredes Texturizadas**:
  - Diferentes materiales por zona
  - Iluminación dinámica
  - Efectos de distancia
- **UI**: Azul Pokémon (#192A56) con acentos dorados

### Sprites y Animaciones

- **Sistema de Sprites Animados**: 4 frames por sprite con interpolación
- **Colores Pokémon**: Amarillo (Pikachu), Naranja, Rojo con efectos de brillo
- **Animación Fluida**: 60fps con sincronización de frames
- **Escalado Inteligente**: Adaptación automática según resolución

## 🎵 Sistema de Audio Completo

### Efectos de Sonido

- **Pasos del jugador** - Sonido procedural según superficie
- **Sonidos de victoria** - Feedback positivo al completar nivel
- **Audio ambiente** - Atmósfera inmersiva del mundo Pokémon
- **Sistema de volumen** - Control dinámico de audio

### Configuración de Audio

- **playlist_config.json** - Metadata completa de canciones
- **Soporte de formatos** - MP3, WAV, OGG
- **Audio 3D** - Efectos posicionales (preparado para expansión)

## 🕹️ Controles Completos

### Movimiento Principal

- **W / ↑**: Mover hacia adelante con aceleración
- **S / ↓**: Mover hacia atrás con control preciso
- **A / ←**: Rotar a la izquierda (suave)
- **D / →**: Rotar a la derecha (suave)
- **Mouse**: Rotación horizontal continua (captura de cursor habilitada)

### Controles de Interfaz

- **M**: Alternar entre vista 3D y minimapa 2D
- **↑/↓**: Navegar opciones en menús
- **←/→**: Seleccionar nivel en pantalla de selección
- **ENTER**: Confirmar selección/acción
- **ESC**: Retroceder en menús / Salir del juego
- **R**: Reiniciar nivel actual (en pantalla de victoria)

### Soporte de Gamepad (Futuro)

- **Stick izquierdo**: Movimiento del jugador
- **Stick derecho**: Rotación de cámara
- **Botones**: Navegación de menús
- **Triggers**: Funciones especiales

## 🛠️ Tecnologías y Arquitectura

### Lenguajes y Frameworks

- **Lenguaje Principal**: Rust 🦀 (Edition 2024)
- **Motor Gráfico**: Raylib 5.5.1
- **Generación de Laberintos**: Python 3.x

### Librerías y Dependencias

- **rodio**: Sistema completo de audio con soporte multi-formato
- **gilrs**: Soporte de gamepad y controladores
- **nalgebra**: Matemáticas vectoriales y transformaciones 3D
- **serde_json**: Parsing y serialización de datos del laberinto

### Arquitectura del Motor

- **Algoritmo DDA**: Raycasting optimizado con detección precisa
- **Buffer de Frame**: Manejo eficiente de píxeles con doble buffer
- **Sistema Modular**: Separación clara de responsabilidades
- **Gestión de Estados**: Máquina de estados robusta para el juego

## 📁 Estructura del Proyecto

```
graficas_proyecto1/
└── proy1/
    ├── src/
    │   ├── main.rs           # Bucle principal y coordinación del juego
    │   ├── framebuffer.rs    # Manejo del buffer de píxeles y rendering
    │   ├── player.rs         # Lógica del jugador y controles
    │   ├── render.rs         # Sistemas de renderizado 2D y 3D
    │   ├── raycasting.rs     # Motor de raycasting y algoritmo DDA
    │   ├── caster.rs         # Utilidades de casting y intersecciones
    │   ├── maze.rs           # Generación y manejo del laberinto
    │   ├── textures.rs       # Sistema de carga y manejo de texturas
    │   ├── audio.rs          # Motor de audio completo con playlist
    │   ├── game_state.rs     # Estados del juego y navegación de menús
    │   ├── sprites.rs        # Sistema de sprites animados
    │   ├── line.rs           # Algoritmos de dibujado de líneas
    │   └── maze.py           # Generador de laberintos con algoritmo recursivo
    ├── assets/
    │   ├── img/
    │   │   ├── wall1.jpg     # Textura de pared tipo 1
    │   │   ├── wall2.jpg     # Textura de pared tipo 2
    │   │   ├── wall3.jpg     # Textura de pared tipo 3
    │   │   ├── wall4.jpg     # Textura de pared tipo 4
    │   │   └── wall5.jpg     # Textura de pared tipo 5
    │   └── music/
    │       ├── shake_it_off.mp3                          # Track 1
    │       ├── blank_space.mp3                           # Track 2
    │       ├── anti_hero.mp3                             # Track 3
    │       ├── love_story.mp3                            # Track 4
    │       ├── we_are_never_getting_back_together.mp3    # Track 5
    │       ├── playlist_config.json                      # Configuración de playlist
    │       └── PLACEHOLDER_MUSIC.txt                     # Documentación de audio
    ├── target/               # Binarios compilados (debug/release)
    ├── Cargo.toml           # Configuración del proyecto Rust
    └── Cargo.lock           # Lock file de dependencias
```

## 🚀 Instalación y Ejecución

### Prerrequisitos

- **Rust**: 1.70+ con toolchain stable
- **Python**: 3.x para generación de laberintos
- **Sistema Operativo**: Windows, macOS, o Linux
- **Audio**: Drivers de audio compatibles con rodio

### Instalación Rápida

1. **Clonar el repositorio**:

```bash
git clone https://github.com/FerAHMz/graficas_proyecto1.git
cd graficas_proyecto1/proy1
```

2. **Compilar el proyecto** (optimizado):

```bash
cargo build --release
```

3. **Ejecutar el juego**:

```bash
cargo run --release
```

### Instalación para Desarrollo

```bash
# Compilación rápida para desarrollo
cargo build

# Ejecutar en modo debug (con logs detallados)
cargo run

# Verificar y corregir código
cargo clippy
cargo fmt
```

### Solución de Problemas

- **Error de Python**: Asegúrate de tener Python 3.x instalado y disponible en PATH
- **Error de Audio**: Verifica que tu sistema tenga drivers de audio compatibles
- **Rendimiento bajo**: Usa `--release` para compilación optimizada

## 🎯 Objetivos y Mecánicas del Juego

### Objetivo Principal

1. **Navegar** por el laberinto generado dinámicamente
2. **Encontrar** la salida dorada marcada con 'g' (goal)
3. **Escapar** del laberinto sin quedar atascado
4. **Disfrutar** de la experiencia visual 3D con tema Pokémon

### Mecánicas Avanzadas

- **Tres Niveles de Dificultad**:

  - 🟢 **Centro Pokémon** (4x4): Perfecto para principiantes
  - 🟡 **Cueva Oscura** (8x8): Desafío intermedio
  - 🔴 **Torre Victoria** (12x12): Para maestros Pokémon

- **Sistema de Navegación Dual**:

  - **Vista 3D**: Experiencia inmersiva de primera persona
  - **Vista 2D**: Minimapa estratégico para planificación

- **Elementos Interactivos**:
  - Sprites de Pokémon animados como puntos de referencia
  - Música dinámica que cambia según el progreso
  - Efectos visuales que reaccionan a las acciones del jugador

## 🔧 Motor de Raycasting - Implementación Técnica

### Algoritmo Core

1. **Lanzamiento de Rayos**: Para cada columna de píxeles (800 rayos por frame)
2. **Algoritmo DDA**: Digital Differential Analyzer para intersección eficiente
3. **Detección de Paredes**: Identificación precisa de colisiones con geometría
4. **Cálculo de Distancia**: Con corrección automática de distorsión de ojo de pez
5. **Renderizado de Alturas**: Altura de pared proporcional a 1/distancia
6. **Sistema de Texturas**: Mapeo UV con interpolación bilineal

### Optimizaciones Implementadas

- **Precálculo de Tablas**: Seno/coseno pre-calculados para rotaciones
- **Clipping Inteligente**: Solo renderizar geometría visible
- **Buffer de Profundidad**: Z-buffer para sprites y elementos 3D
- **Interpolación de Texturas**: Sampling eficiente con anti-aliasing básico
- **Culling de Objetos**: Solo procesar sprites en el campo de visión

### Características Avanzadas

- **Sombreado por Distancia**: Atenuación realista de iluminación
- **Diferenciación de Paredes**: Norte/Sur vs Este/Oeste con colores distintos
- **Sistema de Colisiones**: Detección precisa con márgen de seguridad
- **Interpolación de Movimiento**: Movimiento suave a 60fps constantes

## 🎵 Audio

El sistema de audio está preparado para:

- **Música de fondo**: Simulados
- **Efectos de sonido**: Pasos del jugador y sonido de victoria

## 🏁 Estados del Juego y Navegación

### Estados Implementados

1. **Welcome Screen**:

   - Pantalla de bienvenida temática con logo Pokémon
   - Música de introducción
   - Navegación con teclado y gamepad

2. **Level Select**:

   - Selección visual de tres regiones Pokémon
   - Previsualización de dificultad y tamaño
   - Información detallada de cada nivel

3. **Playing State**:

   - Motor de raycasting en tiempo real
   - HUD con información de estado
   - Minimapa interactivo

4. **Victory Screen**:
   - Celebración de éxito con efectos especiales
   - Estadísticas de completado
   - Opciones para reintentar o avanzar

### Transiciones de Estado

- **Animaciones fluidas** entre pantallas
- **Persistencia de configuración** entre sesiones
- **Sistema de guardado** de progreso (preparado)
- **Efectos de transición** con tema musical

## 📊 Rendimiento y Optimización

### Métricas de Rendimiento

- **Framerate Objetivo**: 60 FPS constantes
- **Resolución**: 800x600 optimizada para rendimiento
- **Rayos por Frame**: 800 (uno por columna)
- **Sprites Animados**: Hasta 50 simultáneos sin pérdida de FPS

### Optimizaciones Implementadas

- **Compilación Release**: Optimizaciones de nivel 3 habilitadas
- **Precálculo de Trigonometría**: Tablas LUT para funciones matemáticas
- **Gestión Eficiente de Memoria**: Reutilización de buffers
- **Culling Inteligente**: Solo renderizar elementos visibles
- **Carga Bajo Demanda**: Recursos cargados según necesidad

### Configuración de Performance

```toml
[profile.dev]
opt-level = 3        # Optimización alta incluso en debug
debug = false        # Sin información de debug para mayor velocidad

[profile.release]
lto = true          # Link-time optimization
codegen-units = 1   # Mejor optimización
panic = "abort"     # Menor overhead
```

## 👥 Autor

**Fernando Hernández**  
Curso de Gráficas por Computadora  
Universidad del Valle de Guatemala  
Sexto Semestre - 2024

### Información del Proyecto

- **Fecha de Desarrollo**: Agosto 2025
- **Lenguaje Principal**: Rust 🦀
- **Líneas de Código**: ~2000+ líneas
- **Tiempo de Desarrollo**: 3 semanas intensivas
- **Motor Gráfico**: Raylib 5.5.1

---

## � Video de Demostración

�🎮 **¡Mira el juego en acción!**  
📹 **Video de Prueba del Proyecto**: [https://youtu.be/yDyHkiSv7bg](https://youtu.be/yDyHkiSv7bg)

El video incluye:

- Demostración completa de todas las características
- Navegación por los tres niveles de dificultad
- Showcase del sistema de audio
- Comparación entre vista 3D y minimapa 2D
- Gameplay de inicio a fin mostrando mecánicas

---

🎮 **¡Disfruta escapando del laberinto Pokémon!** 🎮  
⭐ **¡No olvides darle una estrella al repositorio si te gustó el proyecto!** ⭐
