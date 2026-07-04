#Requires -Version 5.0
$ErrorActionPreference = "Stop"

# ============================================================
$Repo    = "asifali411/toylang-prototype"
$BinName = "toylang"
$AssetName = "$BinName-windows.exe"
$InstallDir = "$env:LOCALAPPDATA\Programs\$BinName"
# ============================================================

function Write-Info($msg) {
    Write-Host "==> " -ForegroundColor Cyan -NoNewline
    Write-Host $msg
}

function Write-ErrorAndExit($msg) {
    Write-Host "error: $msg" -ForegroundColor Red
    exit 1
}

# --- resolve latest release download URL ---
Write-Info "Fetching latest release info..."
$ApiUrl = "https://api.github.com/repos/$Repo/releases/latest"

try {
    $release = Invoke-RestMethod -Uri $ApiUrl -Headers @{ "User-Agent" = "$BinName-installer" }
} catch {
    Write-ErrorAndExit "Failed to fetch release info from $ApiUrl"
}

$asset = $release.assets | Where-Object { $_.name -eq $AssetName }
if (-not $asset) {
    Write-ErrorAndExit "Could not find asset '$AssetName' in the latest release.`nCheck https://github.com/$Repo/releases for available downloads."
}

$DownloadUrl = $asset.browser_download_url

# --- download ---
Write-Info "Downloading $AssetName..."
New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
$ExeDest = Join-Path $InstallDir "$BinName.exe"
Invoke-WebRequest -Uri $DownloadUrl -OutFile $ExeDest

Write-Info "Installed $BinName to $InstallDir"

# --- PATH check (User scope, no admin required) ---
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    $NewPath = "$UserPath;$InstallDir"
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
    Write-Info "Added $InstallDir to your User PATH."
    Write-Info "Restart your terminal (or log off/on) for the change to take effect."
} else {
    Write-Info "$InstallDir is already on your PATH."
}

Write-Info "Done! Open a new terminal and run '$BinName --version' to verify."
