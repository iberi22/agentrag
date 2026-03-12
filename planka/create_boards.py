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

# Obtener proyectos existentes
r = session.get(f"{PLANKA_URL}/api/projects")
items = r.json().get("items", [])
proyectos_existentes = {p["name"]: p["id"] for p in items}
print(f"Proyectos existentes: {list(proyectos_existentes.keys())}")

# Proyectos a crear boards
targets = ["Cortex", "ZeroClaw", "Trading Bot", "ManteniApp", "Research", "Ops"]

for name in targets:
    if name in proyectos_existentes:
        project_id = proyectos_existentes[name]
        
        # Crear board
        r2 = session.post(
            f"{PLANKA_URL}/api/projects/{project_id}/boards",
            json={"name": name, "position": 0}
        )
        
        if r2.status_code in [200, 201]:
            board_id = r2.json().get("item", {}).get("id")
            
            # Crear columnas
            for col_name, pos in [("Backlog", 0), ("In Progress", 1), ("Done", 2)]:
                session.post(
                    f"{PLANKA_URL}/api/boards/{board_id}/lists",
                    json={"name": col_name, "position": pos}
                )
            print(f"[OK] {name}")
        else:
            print(f"[WARN] {name} board: {r2.status_code}")

print("\n=== COMPLETADO ===")
print("Refresca Planka para ver los boards.")
