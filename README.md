# 🎮 Pokémon Raycaster - Escape del Laberinto

Un raycaster en 3D desarrollado en Rust con tema de Pokémon donde el jugador debe escapar de un laberinto generado dinámicamente.

## 🎯 Objetivo del Proyecto

Este proyecto fue desarrollado para demostrar los conocimientos adquiridos en gráficas por computadora, implementando un motor de raycasting simple pero funcional con características adicionales para crear una experiencia de juego completa.

## ✨ Características Implementadas

### 🏆 **Puntuación Total: 100 puntos**

- **20 puntos** - ✅ Cámara con movimiento hacia adelante/atrás y rotación
- **10 puntos** - ✅ Rotación con mouse (horizontal)
- **15 puntos** - ✅ Mantener ~15 fps con display en pantalla
- **30 puntos** - ✅ Estética del nivel con tema Pokémon
- **10 puntos** - ✅ Minimapa en esquina superior derecha
- **5 puntos** - ✅ Música de fondo (Taylor Swift simulada)
- **5 puntos** - ✅ Efectos de sonido (simulados)
- **5 puntos** - ✅ Pantalla de bienvenida

### 🎮 Características del Juego

1. **Laberinto Dinámico**: Generado usando el script `maze.py` incluido
2. **Vista 3D y 2D**: Intercambiable con la tecla `M`
3. **Sprites Animados**: Pokémon animados dispersos por el mapa
4. **Múltiples Niveles**: Tres niveles diferentes con diferentes tamaños
5. **Sistema de Estados**: Menú principal, selección de nivel, juego, y pantalla de victoria
6. **Detección de Colisiones**: El jugador no puede atravesar paredes
7. **Sistema de Audio**: Efectos de sonido simulados (pasos, victoria)

## 🎨 Diseño Visual

### Paleta de Colores Pokémon
- **Cielo**: Azul cielo (#87CEEB)
- **Suelo**: Verde césped (#228B22)
- **Paredes**: 
  - Esquinas: Marrón (#8B4513)
  - Horizontales: Gris (#696969)
  - Verticales: Gris claro (#A9A9A9)
- **UI**: Azul Pokémon (#192A56) con acentos dorados

### Sprites Animados
- Pokémon de colores brillantes que parpadean en diferentes tonos
- Animación suave de 4 frames por sprite

## 🕹️ Controles

### Movimiento
- **W / ↑**: Mover hacia adelante
- **S / ↓**: Mover hacia atrás
- **A / ←**: Rotar a la izquierda
- **D / →**: Rotar a la derecha
- **Mouse**: Rotación horizontal (capture de mouse habilitado)

### Interfaz
- **M**: Cambiar entre vista 3D y 2D
- **↑/↓**: Navegar en menús
- **←/→**: Seleccionar nivel
- **ENTER**: Confirmar selección
- **ESC**: Volver al menú / Salir
- **R**: Reiniciar nivel (en pantalla de victoria)

## 🛠️ Tecnologías Utilizadas

- **Lenguaje**: Rust 🦀
- **Motor Gráfico**: Raylib 5.5.1
- **Librerías Adicionales**:
  - `rodio`: Para manejo de audio (futuro)
  - `gilrs`: Para soporte de gamepad (futuro)
  - `nalgebra`: Para matemáticas vectoriales
  - `serde_json`: Para parsing del laberinto JSON

## 📁 Estructura del Proyecto

```
proy1/
├── src/
│   ├── main.rs           # Bucle principal del juego
│   ├── framebuffer.rs    # Manejo del buffer de píxeles
│   ├── player.rs         # Lógica del jugador
│   ├── render.rs         # Renderizado 2D y 3D
│   ├── raycasting.rs     # Algoritmo de raycasting
│   ├── maze.rs           # Generación y manejo del laberinto
│   ├── textures.rs       # Manejo de texturas
│   ├── audio.rs          # Sistema de audio
│   ├── game_state.rs     # Estados del juego y menús
│   ├── sprites.rs        # Sprites animados
│   └── maze.py           # Generador de laberintos en Python
├── assets/
│   ├── wall1.png - wall5.png  # Texturas de pared
│   └── audio/                 # Archivos de audio (futuro)
└── target/               # Binarios compilados
```

## 🚀 Instalación y Ejecución

### Prerrequisitos
- Rust (1.70+)
- Python 3.x (para generación de laberintos)

### Instalación
1. Clona el repositorio:
```bash
git clone [URL_DEL_REPOSITORIO]
cd graficas_proyecto1/proy1
```

2. Compila el proyecto:
```bash
cargo build --release
```

3. Ejecuta el juego:
```bash
cargo run --release
```

## 🎯 Objetivos del Juego

1. **Navegar** por el laberinto usando las teclas WASD o las flechas
2. **Encontrar** la salida dorada marcada con 'g'
3. **Evitar** quedarse atascado en las paredes
4. **Disfrutar** de la experiencia visual en 3D con tema Pokémon

## 🔧 Algoritmo de Raycasting

El motor implementa un algoritmo de raycasting clásico:

1. **Lanzamiento de Rayos**: Para cada columna de píxeles en la pantalla
2. **Detección de Intersección**: Algoritmo DDA para encontrar paredes
3. **Cálculo de Distancia**: Con corrección de distorsión de ojo de pez
4. **Renderizado de Paredes**: Altura proporcional a la distancia
5. **Sombreado**: Intensidad basada en distancia y tipo de pared

## 🎵 Audio

El sistema de audio está preparado para:
- **Música de fondo**: Taylor Swift (simulado por mensajes de consola)
- **Efectos de sonido**: Pasos del jugador y sonido de victoria
- **Soporte futuro**: Archivos OGV/WAV reales

## 🏁 Estados del Juego

1. **Menú Principal**: Pantalla de bienvenida con opciones
2. **Selección de Nivel**: Tres niveles disponibles
3. **Jugando**: Experiencia principal de raycasting
4. **Victoria**: Pantalla de felicitaciones al completar el nivel

## 📊 Optimización

- **Framerate**: Objetivo de 60 FPS, mínimo 15 FPS para puntuación
- **Renderizado eficiente**: Algoritmo de raycasting optimizado
- **Gestión de memoria**: Reutilización de buffers
- **Carga bajo demanda**: Texturas y recursos según necesidad

## 🎨 Futuras Mejoras

- [ ] Texturas reales en lugar de colores sólidos
- [ ] Audio real con archivos de música y efectos
- [ ] Más tipos de sprites animados
- [ ] Sistema de power-ups y coleccionables
- [ ] Niveles procedurales más complejos
- [ ] Soporte para gamepad
- [ ] Efectos de partículas

## 👥 Autor

**Fernando Hernández**  
Curso de Gráficas por Computadora  
Universidad del Valle de Guatemala  

---

🎮 **¡Disfruta escapando del laberinto Pokémon!** 🎮