# Uninstall SMS Client Demo (Windows). Re-runnable: if app files are gone but
# books data remains, still offers to delete the data directory.
#
# Usage:
#   powershell -ExecutionPolicy Bypass -File .\uninstall.ps1
#   powershell -ExecutionPolicy Bypass -File .\uninstall.ps1 -Dev
param(
    [switch]$Dev,
    [switch]$Prod,
    [switch]$NoPause
)

$ErrorActionPreference = "Stop"
$ScriptDir = $PSScriptRoot

function Pause-IfInteractive {
    if ($NoPause) { return }
    if ([Environment]::UserInteractive -and -not [Console]::IsInputRedirected) {
        Write-Host ""
        Write-Host "Press Enter to close this window..."
        try { [void][Console]::ReadLine() } catch { Read-Host "Press Enter to close" | Out-Null }
    }
}

function Read-Manifest([string]$path) {
    $map = @{}
    if (-not (Test-Path $path)) { return $map }
    Get-Content $path | ForEach-Object {
        $line = $_.Trim()
        if (-not $line -or $line.StartsWith("#")) { return }
        $i = $line.IndexOf("=")
        if ($i -lt 1) { return }
        $map[$line.Substring(0, $i).Trim()] = $line.Substring($i + 1).Trim().Trim('"')
    }
    return $map
}

function Resolve-Identity {
    $local = Join-Path $ScriptDir "install-manifest.env"
    if (Test-Path $local) { return (Read-Manifest $local) }

    $bookmarkDir = Join-Path $env:LOCALAPPDATA "Wadhu"
    if ($Dev -and (Test-Path (Join-Path $bookmarkDir "wadhu-sms-client-demo-debug.env"))) {
        return (Read-Manifest (Join-Path $bookmarkDir "wadhu-sms-client-demo-debug.env"))
    }
    if ($Prod -and (Test-Path (Join-Path $bookmarkDir "wadhu-sms-client-demo.env"))) {
        return (Read-Manifest (Join-Path $bookmarkDir "wadhu-sms-client-demo.env"))
    }
    if (Test-Path (Join-Path $bookmarkDir "wadhu-sms-client-demo.env")) {
        return (Read-Manifest (Join-Path $bookmarkDir "wadhu-sms-client-demo.env"))
    }
    if (Test-Path (Join-Path $bookmarkDir "wadhu-sms-client-demo-debug.env")) {
        return (Read-Manifest (Join-Path $bookmarkDir "wadhu-sms-client-demo-debug.env"))
    }

    if ($Dev) {
        return @{
            WADHU_ENV = "development"
            WADHU_APP_ID = "com.wadhu.smsclientdemo.debug"
            WADHU_APP_LABEL = "SMS Client Demo Dev"
        }
    }
    return @{
        WADHU_ENV = "production"
        WADHU_APP_ID = "com.wadhu.smsclientdemo"
        WADHU_APP_LABEL = "SMS Client Demo"
    }
}

$m = Resolve-Identity
$AppId = $m["WADHU_APP_ID"]
$Label = $m["WADHU_APP_LABEL"]
$EnvName = $m["WADHU_ENV"]
$IsDebug = $AppId.EndsWith(".debug") -or $EnvName -eq "development"
$ProgramsName = if ($IsDebug) { "SMS Client Demo Dev" } else { "SMS Client Demo" }
$slug = if ($IsDebug) { "wadhu-sms-client-demo-debug" } else { "wadhu-sms-client-demo" }

$InstallDir = $m["INSTALL_DIR"]
if (-not $InstallDir) {
    $InstallDir = Join-Path $env:LOCALAPPDATA "Programs\$ProgramsName"
}
# Prefer the folder we were launched from when it looks like an install tree.
if (Test-Path (Join-Path $ScriptDir "wadhu-sms-client-demo.exe")) {
    $InstallDir = $ScriptDir
}

$DataDir = Join-Path $env:APPDATA $AppId
$StartMenu = Join-Path $env:APPDATA "Microsoft\Windows\Start Menu\Programs\$ProgramsName"
$DesktopLnk = Join-Path ([Environment]::GetFolderPath('Desktop')) "$Label.lnk"
$TaskbarPinLnk = Join-Path $env:APPDATA "Microsoft\Internet Explorer\Quick Launch\User Pinned\TaskBar\$Label.lnk"
$UninstallKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\$AppId"
$Bookmark = Join-Path $env:LOCALAPPDATA "Wadhu\$slug.env"

Write-Host "Uninstall $Label"
Write-Host "  App id:    $AppId"
Write-Host "  App files: $InstallDir"
Write-Host "  Data:      $DataDir"

$exe = Join-Path $InstallDir "wadhu-sms-client-demo.exe"
$appPresent = Test-Path $exe

# Ask about data while the exe can still wipe SQLite categories.
Write-Host ""
Write-Host "$Label - local data:"
Write-Host "  $DataDir"
$wiped = $false
if (Test-Path $DataDir) {
    Write-Host ""
    Write-Host "Delete all local data for this app id?"
    $ans = Read-Host "[y/N]"
    if ($ans -match '^(y|yes)$') {
        if (Test-Path $exe) {
            Write-Host "Wiping local data..."
            Start-Process -FilePath $exe -ArgumentList @("--uninstall-wipe", "all") -Wait -WindowStyle Hidden -ErrorAction SilentlyContinue
        }
        if (Test-Path $DataDir) {
            Remove-Item -Recurse -Force -ErrorAction SilentlyContinue $DataDir
        }
        $wiped = $true
    } else {
        Write-Host "Keeping local data at: $DataDir"
    }
} else {
    Write-Host "  (no data directory found - nothing to remove)"
}

if ($appPresent) {
    Write-Host ""
    Write-Host "Removing app files..."
    # Remove Start Menu / Desktop / taskbar pin copy + ARP first.
    if (Test-Path $StartMenu) {
        Remove-Item -Recurse -Force $StartMenu
    }
    if (Test-Path $DesktopLnk) {
        Remove-Item -Force $DesktopLnk
    }
    if (Test-Path $TaskbarPinLnk) {
        Remove-Item -Force $TaskbarPinLnk
    }
    if (Test-Path $UninstallKey) {
        Remove-Item -Recurse -Force $UninstallKey
    }
    if ($ScriptDir -eq $InstallDir) {
        # Delete after process exits.
        $cleanup = $InstallDir
    } else {
        Remove-Item -Recurse -Force $InstallDir
        $cleanup = $null
    }
    Write-Host "  App files removed."
} else {
    Write-Host ""
    Write-Host "App files already gone (or never installed here)."
    if (Test-Path $StartMenu) { Remove-Item -Recurse -Force $StartMenu }
    if (Test-Path $DesktopLnk) { Remove-Item -Force $DesktopLnk }
    if (Test-Path $TaskbarPinLnk) { Remove-Item -Force $TaskbarPinLnk }
    if (Test-Path $UninstallKey) { Remove-Item -Recurse -Force $UninstallKey }
    $cleanup = $null
}

if (Test-Path $Bookmark) { Remove-Item -Force $Bookmark }

Write-Host ""
Write-Host "Done."
if (-not $wiped -and (Test-Path $DataDir)) {
    Write-Host "Reminder: local data is still on disk. Re-run uninstall to remove it."
}

if ($cleanup) {
    Start-Process powershell -WindowStyle Hidden -ArgumentList @(
        "-NoProfile", "-Command",
        "Start-Sleep -Seconds 1; Remove-Item -LiteralPath '$cleanup' -Recurse -Force -ErrorAction SilentlyContinue"
    )
}

Pause-IfInteractive
