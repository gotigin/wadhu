@echo off
REM Double-click uninstaller for SMS Client Demo (Windows).
cd /d "%~dp0"
title Uninstall SMS Client Demo
echo Uninstalling SMS Client Demo...
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
