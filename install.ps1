# Sentinel Installer for Windows (PowerShell)
# Version: 1.0.0

$ErrorActionPreference = "Stop"

# Configuration
$RepoUrl = "https://github.com/sergiogswv/sentinel-rust.git"
$InstallDir = "$env:USERPROFILE\.sentinel"
$BinName = "sentinel.exe"

# Functions
function Print-Header {
    Write-Host "================================" -ForegroundColor Blue
    Write-Host "  🛡️  Sentinel Installer" -ForegroundColor Blue
    Write-Host "================================`n" -ForegroundColor Blue
}

function Print-Success {
    param([string]$Message)
    Write-Host "✅ $Message" -ForegroundColor Green
}

function Print-Error {
    param([string]$Message)
    Write-Host "❌ $Message" -ForegroundColor Red
}

function Print-Info {
    param([string]$Message)
    Write-Host "ℹ️  $Message" -ForegroundColor Yellow
}

function Check-Dependencies {
    Print-Info "Verificando dependencias..."

    # Check Git
    if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
        Print-Error "Git no está instalado"
        Write-Host "`nPor favor instala Git primero:"
        Write-Host "  - Descarga desde: https://git-scm.com/download/win"
        Write-Host "  - O usa winget: winget install --id Git.Git -e --source winget"
        exit 1
    }

    # Check Rust
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        Print-Info "Rust no está instalado. Instalando Rust..."

        # Download rustup-init.exe
        $RustupUrl = "https://win.rustup.rs/x86_64"
        $RustupPath = "$env:TEMP\rustup-init.exe"

        Print-Info "Descargando Rust installer..."
        Invoke-WebRequest -Uri $RustupUrl -OutFile $RustupPath

        Print-Info "Ejecutando instalador de Rust..."
        Start-Process -FilePath $RustupPath -ArgumentList "-y" -Wait -NoNewWindow

        # Refresh environment variables
        $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")

        Print-Success "Rust instalado correctamente"
    } else {
        Print-Success "Rust ya está instalado"
    }
}

function Clone-OrUpdate {
    if (Test-Path $InstallDir) {
        Print-Info "Directorio de instalación encontrado. Actualizando..."
        Set-Location $InstallDir

        # Get current branch
        $CurrentBranch = git branch --show-current

        # Fetch latest changes
        git fetch origin

        # Check if there are updates
        $Local = git rev-parse "@"
        $Remote = git rev-parse "@{u}"

        if ($Local -eq $Remote) {
            Print-Info "Ya tienes la última versión"
        } else {
            Print-Info "Nuevas actualizaciones disponibles. Descargando..."
            git pull origin $CurrentBranch
            Print-Success "Código actualizado correctamente"
        }
    } else {
        Print-Info "Clonando repositorio..."
        git clone $RepoUrl $InstallDir
        Set-Location $InstallDir
        Print-Success "Repositorio clonado correctamente"
    }
}

function Build-Project {
    Print-Info "Compilando Sentinel (esto puede tomar unos minutos)..."
    Set-Location $InstallDir
    cargo build --release
    Print-Success "Compilación exitosa"
}

function Install-Binary {
    Print-Info "Instalando binario..."

    # Create bin directory in user profile if it doesn't exist
    $UserBinDir = "$env:USERPROFILE\.local\bin"
    if (-not (Test-Path $UserBinDir)) {
        New-Item -ItemType Directory -Path $UserBinDir | Out-Null
    }

    # Copy binary
    $SourceBin = "$InstallDir\target\release\sentinel-rust.exe"
    $DestBin = "$UserBinDir\$BinName"
    Copy-Item -Path $SourceBin -Destination $DestBin -Force

    # Add to PATH if not already there
    $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($UserPath -notlike "*$UserBinDir*") {
        Print-Info "Agregando directorio al PATH..."
        [Environment]::SetEnvironmentVariable("Path", "$UserPath;$UserBinDir", "User")
        $env:Path += ";$UserBinDir"
        Print-Success "Directorio agregado al PATH"
    }

    Print-Success "Binario instalado en $DestBin"
}

function Create-UpdateScript {
    Print-Info "Creando script de actualización..."

    $UpdateScript = "$InstallDir\update.ps1"
    $UpdateScriptContent = @'
# Sentinel Update Script
$ErrorActionPreference = "Stop"

Write-Host "`n🔄 Actualizando Sentinel...`n" -ForegroundColor Yellow

Set-Location "$env:USERPROFILE\.sentinel"
git pull origin master
cargo build --release

$UserBinDir = "$env:USERPROFILE\.local\bin"
Copy-Item -Path "target\release\sentinel-rust.exe" -Destination "$UserBinDir\sentinel.exe" -Force

Write-Host "`n✅ Sentinel actualizado correctamente" -ForegroundColor Green
Write-Host "Ejecuta: sentinel" -ForegroundColor Green
'@

    Set-Content -Path $UpdateScript -Value $UpdateScriptContent
    Print-Success "Script de actualización creado en $UpdateScript"
}

function Create-UpdateBatch {
    Print-Info "Creando batch de actualización..."

    $UpdateBatch = "$InstallDir\update.bat"
    $UpdateBatchContent = @'
@echo off
echo.
echo Actualizando Sentinel...
echo.

cd %USERPROFILE%\.sentinel
git pull origin master
cargo build --release

set USERBIN=%USERPROFILE%\.local\bin
copy /Y target\release\sentinel-rust.exe "%USERBIN%\sentinel.exe"

echo.
echo Sentinel actualizado correctamente
echo Ejecuta: sentinel
pause
'@

    Set-Content -Path $UpdateBatch -Value $UpdateBatchContent
    Print-Success "Script batch de actualización creado en $UpdateBatch"
}

function Add-Aliases {
    # Create PowerShell profile if it doesn't exist
    if (-not (Test-Path $PROFILE)) {
        New-Item -ItemType File -Path $PROFILE -Force | Out-Null
    }

    # Add alias for update
    $AliasLine = "function sentinel-update { & `"$InstallDir\update.ps1`" }"
    $ProfileContent = Get-Content $PROFILE -Raw -ErrorAction SilentlyContinue

    if ($ProfileContent -notlike "*sentinel-update*") {
        Add-Content -Path $PROFILE -Value "`n# Sentinel aliases"
        Add-Content -Path $PROFILE -Value $AliasLine
        Print-Success "Alias 'sentinel-update' agregado al perfil de PowerShell"
    }
}

function Print-Completion {
    Write-Host "`n================================" -ForegroundColor Green
    Write-Host "  ✅ Instalación Completada" -ForegroundColor Green
    Write-Host "================================`n" -ForegroundColor Green
    Write-Host "📋 Próximos pasos:`n" -ForegroundColor Blue
    Write-Host "  1️⃣  Abre una nueva terminal y ejecuta: " -NoNewline
    Write-Host "sentinel" -ForegroundColor Yellow
    Write-Host "  2️⃣  Para actualizar en PowerShell: " -NoNewline
    Write-Host "sentinel-update" -ForegroundColor Yellow
    Write-Host "  3️⃣  O ejecuta manualmente: " -NoNewline
    Write-Host "$InstallDir\update.ps1" -ForegroundColor Yellow
    Write-Host "  4️⃣  O usa el batch: " -NoNewline
    Write-Host "$InstallDir\update.bat`n" -ForegroundColor Yellow
    Write-Host "📖 Documentación: " -NoNewline
    Write-Host "https://github.com/sergiogswv/sentinel-rust`n" -ForegroundColor Cyan
    Write-Host "⚠️  IMPORTANTE: " -NoNewline -ForegroundColor Yellow
    Write-Host "Cierra y abre una nueva terminal para usar 'sentinel'" -ForegroundColor White
}

# Main Installation Flow
function Main {
    try {
        Print-Header
        Check-Dependencies
        Clone-OrUpdate
        Build-Project
        Install-Binary
        Create-UpdateScript
        Create-UpdateBatch
        Add-Aliases
        Print-Completion
    } catch {
        Print-Error "Error durante la instalación: $_"
        exit 1
    }
}

Main
