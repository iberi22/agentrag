#!/bin/bash
# launch_cortex.sh - Lanza Cortex y espera a que esté listo

echo "🚀 Launching Cortex..."

# Cambiar al directorio de Cortex
cd E:/scripts-python/cortex

# Verificar si ya está corriendo
if curl -s http://localhost:8003/health > /dev/null 2>&1; then
    echo "✅ Cortex already running"
    exit 0
fi

# Verificar Docker
if ! command -v docker &> /dev/null; then
    echo "❌ Docker not found"
    exit 1
fi

# Verificar si hay docker-compose
if [ -f "docker-compose.yml" ]; then
    echo "📦 Starting with docker-compose..."
    docker-compose up -d
    
    # Esperar a que esté listo
    echo "⏳ Waiting for Cortex..."
    for i in {1..30}; do
        if curl -s http://localhost:8003/health > /dev/null 2>&1; then
            echo "✅ Cortex is ready!"
            exit 0
        fi
        sleep 2
    done
    
    echo "❌ Cortex failed to start"
    exit 1
else
    # Intentar con docker run
    echo "📦 Starting with docker run..."
    docker run -d --name cortex -p 8003:8003 iberi22/cortex:latest
    
    echo "⏳ Waiting..."
    sleep 10
    
    if curl -s http://localhost:8003/health > /dev/null 2>&1; then
        echo "✅ Cortex is ready!"
    else
        echo "❌ Failed"
    fi
fi
