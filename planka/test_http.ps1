$client = New-Object System.Net.Sockets.TcpClient('localhost', 3000)
$stream = $client.GetStream()
$writer = New-Object System.IO.StreamWriter($stream)
$reader = New-Object System.IO.StreamReader($stream)

$writer.Write("GET / HTTP/1.1`r`nHost: localhost`r`n`r`n")
$writer.Flush()

Start-Sleep -Seconds 2

Write-Host $reader.ReadToEnd()

$client.Close()