# Delete __APP_LABEL__ local data for __APP_ID__.
# Installed into the app folder by the MSI; Start Menu shortcut runs this.
# Placeholders are replaced by build-msi.sh. Prompts before deleting anything.
$ErrorActionPreference = "Stop"
$DataDir = Join-Path $env:APPDATA "__APP_ID__"
$Exe = Join-Path $PSScriptRoot "__BIN_NAME__.exe"

Write-Host "__APP_LABEL__ local data:"
Write-Host "  $DataDir"
Write-Host ""
if (-not (Test-Path $DataDir)) {
    Write-Host "No local data found - nothing to delete."
    Write-Host ""
    Write-Host "Press Enter to close this window..."
    try { [void][Console]::ReadLine() } catch { Read-Host | Out-Null }
    exit 0
}

Add-Type -AssemblyName System.Windows.Forms | Out-Null

$result = [System.Windows.Forms.MessageBox]::Show(
    "Delete all local data for __APP_LABEL__?`n`n$DataDir`n`nThis cannot be undone.",
    "Delete __APP_LABEL__ data",
    [System.Windows.Forms.MessageBoxButtons]::YesNo,
    [System.Windows.Forms.MessageBoxIcon]::Warning
)
if ($result -ne [System.Windows.Forms.DialogResult]::Yes) {
    Write-Host "Keeping local data."
    exit 0
}

if (Test-Path $Exe) {
    Write-Host "Asking the app to wipe local data..."
    Start-Process -FilePath $Exe -ArgumentList @("--uninstall-wipe", "all") -Wait -WindowStyle Hidden -ErrorAction SilentlyContinue
}
if (Test-Path $DataDir) {
    Write-Host "Removing $DataDir ..."
    Remove-Item -Recurse -Force $DataDir
}
Write-Host "Done."
Write-Host ""
Write-Host "Press Enter to close this window..."
try { [void][Console]::ReadLine() } catch { Read-Host | Out-Null }
