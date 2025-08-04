#!/bin/bash

# Script de build para Pokémon Raycaster
echo "🎮 Compilando Pokémon Raycaster..."

cd proy1

# Verificar que Python está disponible para maze.py
if ! command -v python3 &> /dev/null; then
    echo "❌ Error: Python 3 no está instalado"
    exit 1
fi

# Compilar en modo release para mejor rendimiento
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Compilación exitosa!"
    echo "🚀 Para ejecutar el juego, usa: cd proy1 && cargo run --release"
    echo "📊 O ejecuta directamente: ./proy1/target/release/proy1"
else
    echo "❌ Error en la compilación"
    exit 1
fi
