$body = @{
    email = "demo@planka.dev"
    password = "demo"
} | ConvertTo-Json

$result = Invoke-RestMethod -Uri "http://localhost:3000/api/auth/login" -Method POST -Body $body -ContentType "application/json"
$result | ConvertTo-Json