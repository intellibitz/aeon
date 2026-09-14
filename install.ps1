# SUSI Micro-Substrate Installer for Windows
# Tier 0 Onboarding Protocol: Zero-Config Deployment

$SusiRepo = "https://github.com/intellibitz/susi"
$ReleaseUrl = "$SusiRepo/releases/latest/download"
$HomeDir = [Environment]::GetFolderPath("UserProfile")
$GlobalDir = Join-Path $HomeDir ".susi"
$GlobalBinDir = Join-Path $GlobalDir "bin"

# Ensure substrate structure exists
if (-not (Test-Path $GlobalBinDir)) {
    New-Item -ItemType Directory -Force -Path $GlobalBinDir | Out-Null
}

Write-Host "[susi] Initializing 100% Sandboxed Native AI Runtime (Repo: $SusiRepo)..." -ForegroundColor Cyan

$LauncherExePath = Join-Path $GlobalBinDir "susi.exe"
$EngineExePath = Join-Path $GlobalBinDir "susi-engine.exe"

$Installed = $false

# 1. Try Binary Download First (Lightning Fast)
if ($null -eq $env:LOCAL_SOURCE) {
    Write-Host "🔐 Fetching latest susi executables..." -ForegroundColor Yellow

    $LauncherBinary = "susi-windows-x86_64.exe"
    $EngineBinary = "susi-engine-windows-x86_64.exe"

    Stop-Process -Name "susi" -ErrorAction SilentlyContinue
    Stop-Process -Name "susi-engine" -ErrorAction SilentlyContinue

    try {
        Invoke-WebRequest -Uri "$ReleaseUrl/$LauncherBinary" -OutFile $LauncherExePath -UseBasicParsing
        Invoke-WebRequest -Uri "$ReleaseUrl/$EngineBinary" -OutFile $EngineExePath -UseBasicParsing
        $Installed = $true
        Write-Host "  ✅ Downloaded & installed binaries from GitHub." -ForegroundColor Green
    } catch {
        Write-Host "  ⚠️ Binary download failed. Falling back to source build." -ForegroundColor Yellow
    }
}

# 2. Fallback to Source Build
if (-not $Installed -and (Get-Command "cargo" -ErrorAction SilentlyContinue)) {
    Write-Host "[susi Native] Compiling standalone Rust AI engine & launcher..." -ForegroundColor Yellow

    Stop-Process -Name "susi" -ErrorAction SilentlyContinue
    Stop-Process -Name "susi-engine" -ErrorAction SilentlyContinue

    # Build Engine
    Write-Host "  Building engine..." -ForegroundColor Gray
    cargo build --release | Out-Null

    # Build Launcher
    Write-Host "  Building launcher..." -ForegroundColor Gray
    Push-Location "src/native/susi"
    cargo build --release | Out-Null
    Pop-Location

    $EngineSrc = "target\release\susi-engine.exe"
    $LauncherSrc = "src\native\susi\target\release\susi.exe"

    if ((Test-Path $EngineSrc) -and (Test-Path $LauncherSrc)) {
        Copy-Item $EngineSrc $EngineExePath -Force
        Copy-Item $LauncherSrc $LauncherExePath -Force
        $Installed = $true
        Write-Host "  ✅ Compiled & installed native binaries." -ForegroundColor Green
    }
}

if (-not $Installed) {
    Write-Error "Installation failed. Ensure 'cargo' is available or binary downloads are accessible."
    exit 1
}

# 3. PATH Automation (0-Effort Onboarding)
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$GlobalBinDir*") {
    Write-Host "[susi] Automatically adding '$GlobalBinDir' to User PATH..." -ForegroundColor Cyan
    [Environment]::SetEnvironmentVariable("Path", "$GlobalBinDir;$UserPath", "User")
    $env:Path = "$GlobalBinDir;$env:Path"
    Write-Host "  ✅ User PATH updated!" -ForegroundColor Green
}

Write-Host "`n🎉 Global susi is ready! Type 'susi status' or 'susi mcp' to verify." -ForegroundColor Green
Write-Host "Restart your terminal to use the 'susi' command." -ForegroundColor Gray
