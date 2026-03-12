import requests

# Try to find what endpoints exist
endpoints = [
    '/api',
    '/api/users',
    '/api/boards',
    '/api/lists',
    '/api/cards',
]

for ep in endpoints:
    r = requests.options(f'http://192.168.1.8:3000{ep}')
    allow = r.headers.get("Allow", "no Allow header")
    print(f'{ep}: {r.status_code} - {allow}')
