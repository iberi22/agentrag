@echo off
setlocal enabledelayedexpansion

echo ========================================
echo  SWAL Labs - Planka Setup
echo ========================================
echo.

REM Check if Docker is running
docker info >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Docker is not running. Please start Docker Desktop.
    pause
    exit /b 1
)

REM Change to planka directory
cd /d "%~dp0"

echo [1/3] Starting Planka services...
docker-compose up -d

if %errorlevel% neq 0 (
    echo [ERROR] Failed to start services
    pause
    exit /b 1
)

echo.
echo [2/3] Waiting for services to be healthy...
timeout /t 10 /nobreak >nul

REM Check Planka
echo Checking Planka...
for /L %%i in (1,1,30) do (
    curl -s http://localhost:3000 >nul 2>&1
    if !errorlevel! equ 0 (
        echo [OK] Planka is running on http://localhost:3000
        goto :planka_ready
    )
    timeout /t 2 /nobreak >nul
)

:planka_ready

REM Check kanban-mcp
echo Checking kanban-mcp...
for /L %%i in (1,1,15) do (
    curl -s http://localhost:8081/health >nul 2>&1
    if !errorlevel! equ 0 (
        echo [OK] kanban-mcp is running on http://localhost:8081
        goto :mcp_ready
    )
    timeout /t 2 /nobreak >nul
)

:mcp_ready

echo.
echo ========================================
echo  Services Status
echo ========================================
echo  Planka UI:    http://localhost:3000
echo  Kanban MCP:  http://localhost:8081
echo.
echo  Default credentials:
echo    Email:    admin@swal.ai
echo    Password: admin123
echo ========================================
echo.
echo To stop: docker-compose down
echo To view logs: docker-compose logs -f
echo.

pause
