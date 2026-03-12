import requests
import re
import json

s = requests.Session()

# Get the login page to extract any CSRF token
r = s.get('http://192.168.1.8:3000/login')
print('Login page status:', r.status_code)
print('Cookies before:', s.cookies.get_dict())

# Look for CSRF token in the page
csrf_match = re.search(r'name="_csrf"[^>]*value="([^"]+)"', r.text)
csrf = None
if csrf_match:
    csrf = csrf_match.group(1)
    print('CSRF token found:', csrf[:20] + '...')
else:
    print('No CSRF token found in page')
    # Try other patterns
    csrf_match = re.search(r'csrf["\s:=]+["\']([^"\']+)["\']', r.text, re.IGNORECASE)
    if csrf_match:
        csrf = csrf_match.group(1)
        print('Alternative CSRF:', csrf[:20] + '...')

# Try to login with CSRF
if csrf:
    r = s.post('http://192.168.1.8:3000/login', data={
        'email': 'admin@swal.ai',
        'password': 'swaladmin2026',
        '_csrf': csrf
    }, allow_redirects=False)
    print('Login response:', r.status_code)
    print('Cookies after:', s.cookies.get_dict())
    print('Location:', r.headers.get('Location', 'N/A'))
    
    # If redirected, follow it
    if r.status_code in [301, 302, 303, 307, 308]:
        r = s.get(r.headers.get('Location'))
        print('After redirect:', r.status_code)
        print('Cookies final:', s.cookies.get_dict())
else:
    # Try without CSRF
    r = s.post('http://192.168.1.8:3000/login', data={
        'email': 'admin@swal.ai',
        'password': 'swaladmin2026'
    }, allow_redirects=False)
    print('Login without CSRF:', r.status_code)
    print('Response:', r.text[:500])
