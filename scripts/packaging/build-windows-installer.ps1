# PowerShell script to build Windows installer for fsPrompt
# Requires Inno Setup to be installed

$ErrorActionPreference = "Stop"

# Get version from Cargo.toml
$cargoToml = Get-Content "Cargo.toml" -Raw
if ($cargoToml -match 'version\s*=\s*"(.+?)"') {
    $version = $matches[1]
} else {
    Write-Error "Could not extract version from Cargo.toml"
    exit 1
}

Write-Host "Building fsPrompt v$version for Windows x64..." -ForegroundColor Green

# Build release binary
Write-Host "Building release binary..." -ForegroundColor Yellow
cargo build --release --target x86_64-pc-windows-msvc
if ($LASTEXITCODE -ne 0) {
    Write-Error "Failed to build release binary"
    exit 1
}

# Check if Inno Setup is installed
$innoSetupPaths = @(
    "C:\Program Files (x86)\Inno Setup 6\ISCC.exe",
    "C:\Program Files\Inno Setup 6\ISCC.exe",
    "${env:ProgramFiles(x86)}\Inno Setup 6\ISCC.exe",
    "${env:ProgramFiles}\Inno Setup 6\ISCC.exe"
)

$iscc = $null
foreach ($path in $innoSetupPaths) {
    if (Test-Path $path) {
        $iscc = $path
        break
    }
}

if (-not $iscc) {
    Write-Warning "Inno Setup not found. Please install it from: https://jrsoftware.org/isdl.php"
    Write-Warning "Or install via Chocolatey: choco install innosetup"
    exit 1
}

# Create dist directory
New-Item -ItemType Directory -Force -Path "dist" | Out-Null

# Build installer
Write-Host "Building installer..." -ForegroundColor Yellow
& $iscc /Q "scripts\packaging\windows-installer.iss"
if ($LASTEXITCODE -ne 0) {
    Write-Error "Failed to build installer"
    exit 1
}

Write-Host "Installer created successfully!" -ForegroundColor Green
Write-Host "Output: dist\fsprompt-v$version-x86_64-pc-windows-msvc-setup.exe" -ForegroundColor Cyan
Write-Host ""
Write-Host "Note: The installer is unsigned and will trigger Windows SmartScreen warnings." -ForegroundColor Yellow
Write-Host "Users will need to click 'More info' and then 'Run anyway' to install." -ForegroundColor Yellow