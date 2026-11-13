@echo off
REM Double-click uninstaller for Wadhu Template (Windows).
cd /d "%~dp0"
title Uninstall Wadhu Template
echo Uninstalling Wadhu Template...
echo.
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0uninstall.ps1" -NoPause
set ERR=%ERRORLEVEL%
echo.
if %ERR% neq 0 (
  echo Uninstall failed with exit code %ERR%.
) else (
  echo Uninstall finished.
)
echo.
pause
exit /b %ERR%
