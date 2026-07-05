#Requires -Version 5.0
$ErrorActionPreference = "Stop"
# ============================================================
$BinName    = "toylang"
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

# --- remove installed binary/directory ---
if (Test-Path $InstallDir) {
    Write-Info "Removing $InstallDir..."
    Remove-Item -Path $InstallDir -Recurse -Force
    Write-Info "Removed $InstallDir"
} else {
    Write-Info "$InstallDir does not exist; nothing to remove."
}

# --- remove from User PATH ---
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($null -ne $UserPath -and $UserPath -like "*$InstallDir*") {
    $PathParts = $UserPath -split ";" | Where-Object { $_ -and ($_.TrimEnd('\') -ne $InstallDir.TrimEnd('\')) }
    $NewPath = ($PathParts -join ";")
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
    Write-Info "Removed $InstallDir from your User PATH."
    Write-Info "Restart your terminal (or log off/on) for the change to take effect."
} else {
    Write-Info "$InstallDir was not found on your User PATH."
}

Write-Info "Done! $BinName has been uninstalled."