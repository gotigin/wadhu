@echo off
REM Double-click installer for Wadhu Template (Windows).
REM Runs install.ps1 and always waits so errors are visible.
cd /d "%~dp0"
title Wadhu Template Setup
echo Installing Wadhu Template...
echo.
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0install.ps1" -NoPause
set ERR=%ERRORLEVEL%
echo.
if %ERR% neq 0 (
  echo Setup failed with exit code %ERR%.
) else (
  echo Setup finished.
)
echo.
pause
exit /b %ERR%
