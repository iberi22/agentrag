#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Script para crear boards iniciales en Planka"""

import requests
import json

PLANKA_URL = "http://192.168.1.8:3000"
EMAIL = "admin@swal.ai"
PASSWORD = "swaladmin2026"

def create_session():
    """Crear sesion y obtener cookie"""
    session = requests.Session()
    
    # Primero crear el usuario si no existe
    # El endpoint para crear usuario es POST /api/users
    response = session.post(
        f"{PLANKA_URL}/api/users",
        json={
            "email": EMAIL,
            "password": PASSWORD,
            "name": "Admin",
            "username": "admin"
        }
    )
    
    if response.status_code in [200, 201]:
        print("[OK] Usuario creado/ya existe")
    elif response.status_code == 400:
        # Ya existe, intentar login
        print("[INFO] Usuario ya existe, buscando sesion...")
    else:
        print(f"[WARN] Crear usuario: {response.status_code} - {response.text}")
    
    # Intentar obtener boards sin auth (si hay sesion anonima)
    response = session.get(f"{PLANKA_URL}/api/boards")
    print(f"Boards response: {response.status_code}")
    
    return session

def try_api():
    """Probar diferentes endpoints"""
    session = requests.Session()
    
    # Probar crear boards directamente (si la API lo permite)
    boards = [
        {"name": "Cortex", "description": "Sistema de memoria y cognitive"},
        {"name": "ZeroClaw", "description": "Runtime Rust para agentes"},
    ]
    
    for board in boards:
        response = session.post(
            f"{PLANKA_URL}/api/boards",
            json=board
        )
        print(f"{board['name']}: {response.status_code} - {response.text[:200]}")

if __name__ == "__main__":
    print("Probando API de Planka...")
    try_api()
