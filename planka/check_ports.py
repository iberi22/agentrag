import requests

# Try different ports
for port in [3000, 3001, 4000, 8080, 8000]:
    try:
        r = requests.get(f'http://192.168.1.8:{port}/api/', timeout=5)
        content_type = r.headers.get("content-type", "")
        is_json = "application/json" in content_type
        print(f'Port {port}: {r.status_code}, is_json={is_json}')
    except Exception as e:
        print(f'Port {port}: error - {e}')
