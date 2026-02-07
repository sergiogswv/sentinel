@echo off
REM Sentinel Installer for Windows (Batch)
REM Version: 1.0.0

echo ================================
echo   Sentinel Installer
echo ================================
echo.

REM Check if PowerShell is available
where powershell >nul 2>nul
if %errorlevel% neq 0 (
    echo ERROR: PowerShell no esta disponible
    echo Por favor usa Windows 7 o superior
    pause
    exit /b 1
)

echo Ejecutando instalador de PowerShell...
echo.

powershell -ExecutionPolicy Bypass -File "%~dp0install.ps1"

if %errorlevel% neq 0 (
    echo.
    echo ERROR: La instalacion fallo
    pause
    exit /b 1
)

echo.
echo Instalacion completada!
pause
