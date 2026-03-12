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

# Crear proyecto
r = session.post(
    f"{PLANKA_URL}/api/projects",
    json={"name": "Test", "description": "Test", "type": "private"}
)
print(f"Status: {r.status_code}")
print(f"Response: {r.text}")
print(f"JSON: {r.json()}")
