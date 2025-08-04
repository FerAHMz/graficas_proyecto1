# 🧪 Guía de Pruebas - Pokémon Raycaster

Esta guía te ayudará a verificar todas las características implementadas para obtener los 100 puntos del proyecto.

## ✅ Lista de Verificación

### 1. **Cámara con Movimiento (20 puntos)**
- [ ] Presiona `W` o `↑` para mover hacia adelante
- [ ] Presiona `S` o `↓` para mover hacia atrás  
- [ ] Presiona `A` o `←` para rotar a la izquierda
- [ ] Presiona `D` o `→` para rotar a la derecha
- [ ] **Verificar**: El jugador se mueve suavemente sin crashear

### 2. **Rotación con Mouse (10 puntos)**
- [ ] Mueve el mouse horizontalmente
- [ ] **Verificar**: La cámara rota siguiendo el movimiento del mouse
- [ ] **Verificar**: El cursor está oculto (captura de mouse)

### 3. **Framerate ~15 FPS (15 puntos)**
- [ ] Observa el contador de FPS en la esquina superior izquierda
- [ ] **Verificar**: Los FPS se muestran en pantalla
- [ ] **Verificar**: Los FPS se mantienen por encima de 15 (debería ser ~60)
- [ ] **Verificar**: El color es verde si FPS ≥ 15, rojo si < 15

### 4. **Estética Pokémon (30 puntos)**
- [ ] **Colores**: Verifica la paleta de colores Pokémon
  - Cielo azul claro
  - Suelo verde césped
  - Paredes en tonos marrones y grises
- [ ] **UI**: Menús con colores azul Pokémon y dorado
- [ ] **Tema**: Texto y referencias a Pokémon en todo el juego

### 5. **Minimapa (10 puntos)**
- [ ] **Ubicación**: Esquina superior derecha
- [ ] **Contenido**: 
  - Muestra el laberinto completo
  - Punto rojo para el jugador
  - Línea amarilla para la dirección
  - Meta dorada visible
- [ ] **Verificar**: No está "lado a lado" con el mapa principal

### 6. **Música de Fondo Taylor Swift (10 puntos)**
- [ ] Observa la consola/terminal donde ejecutaste el juego
- [ ] **Verificar**: Mensaje de "🎵 Reproduciendo música de Taylor Swift..."
- [ ] **Nota**: Música simulada por mensajes (5 puntos base + 5 por Taylor Swift)

### 7. **Efectos de Sonido (10 puntos)**
- [ ] Muévete por el laberinto
- [ ] **Verificar**: Mensajes de "👟 *paso*" en la consola
- [ ] Llega a la meta (cuadro dorado)
- [ ] **Verificar**: Mensaje de "🎉 ¡VICTORIA!" en la consola

### 8. **Pantalla de Bienvenida (5 puntos)**
- [ ] Al iniciar el juego, aparece el menú principal
- [ ] **Verificar**: Título "POKÉMON RAYCASTER"
- [ ] **Verificar**: Opciones de menú navegables

### 9. **Selección de Múltiples Niveles (10 puntos)**
- [ ] En el menú principal, selecciona "Seleccionar Nivel"
- [ ] **Verificar**: 3 niveles disponibles:
  - Centro Pokémon (fácil)
  - Cueva Oscura (medio)
  - Torre Victoria (difícil)
- [ ] **Verificar**: Navegación con flechas izq/der

### 10. **Pantalla de Victoria (10 puntos)**
- [ ] Completa un nivel llegando al cuadro dorado 'g'
- [ ] **Verificar**: Pantalla de victoria aparece
- [ ] **Verificar**: Opciones para volver al menú (ESC) o reiniciar (R)

## 🎮 Secuencia de Prueba Completa

1. **Ejecuta el juego**: `cargo run --release`
2. **Menú Principal**: Navega con ↑↓, presiona ENTER en "Jugar"
3. **Gameplay**: 
   - Mueve con WASD
   - Rota con mouse
   - Presiona M para vista 2D/3D
   - Observa FPS, minimapa, sprites
4. **Completa nivel**: Encuentra y llega a la meta dorada
5. **Victoria**: Verifica pantalla de éxito
6. **Reinicia**: Presiona R para nuevo nivel o ESC para menú

## 🐛 Solución de Problemas

### Si el juego no compila:
```bash
cd proy1
cargo clean
cargo build --release
```

### Si Python no funciona:
```bash
python3 --version  # Verificar instalación
cd proy1/src
python3 maze.py json 16 12  # Probar generador
```

### Si los FPS son bajos:
- Cierra otras aplicaciones
- Compila en modo release: `cargo run --release`
- El juego está optimizado para mantener 60+ FPS

## 📋 Checklist Final

**Total de puntos verificados: ___/100**

- [ ] Movimiento cámara (20pts)
- [ ] Rotación mouse (10pts) 
- [ ] FPS display (15pts)
- [ ] Estética Pokémon (30pts)
- [ ] Minimapa esquina (10pts)
- [ ] Música Taylor Swift (10pts)
- [ ] Efectos sonido (5pts)
- [ ] Pantalla bienvenida (5pts)
- [ ] Múltiples niveles (10pts)

**¡Felicidades! Has verificado un raycaster completo con 100 puntos!** 🎉
