#!/usr/bin/env python3
import requests

PLANKA_URL = "http://192.168.1.8:3000"
EMAIL = "admin@swal.ai"
PASSWORD = "swaladmin2026"

session = requests.Session()
r = session.post(
    f"{PLANKA_URL}/api/access-tokens",
    json={"emailOrUsername": EMAIL, "password": PASSWORD}
)
token = r.json().get("item")
session.headers.update({"Authorization": f"Bearer {token}"})

# Listar proyectos
r = session.get(f"{PLANKA_URL}/api/projects")
print("Proyectos:", r.json())

# Crear proyecto
r = session.post(
    f"{PLANKA_URL}/api/projects",
    json={"name": "Test2", "description": "Test", "type": "private"}
)
project_id = r.json().get("item", {}).get("id")
print(f"Proyecto ID: {project_id}")

# Crear board
r2 = session.post(
    f"{PLANKA_URL}/api/projects/{project_id}/boards",
    json={"name": "Test Board"}
)
print(f"Board status: {r2.status_code}")
print(f"Board response: {r2.text[:200]}")
