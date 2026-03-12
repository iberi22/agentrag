#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Test login and check token format"""

import requests

PLANKA_URL = "http://192.168.1.8:3000"
EMAIL = "admin@swal.ai"
PASSWORD = "swaladmin2026"

session = requests.Session()

response = session.post(
    f"{PLANKA_URL}/api/access-tokens",
    json={
        "emailOrUsername": EMAIL,
        "password": PASSWORD
    }
)

print(f"Status: {response.status_code}")
print(f"Response: {response.text}")
print(f"Headers: {dict(session.headers)}")
