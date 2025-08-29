@echo off
REM nshell.bat - Windows launcher for nshell

setlocal

REM Set up environment
set NSHELL_HOME=%~dp0
set PATH=%NSHELL_HOME%bin;%PATH%

REM Check for Python
where python >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo Python is not in your PATH. Please install Python 3.6 or later.
    pause
    exit /b 1
)

REM Install Python dependencies if needed
if not exist "%NSHELL_HOME%venv\" (
    echo Setting up Python virtual environment...
    python -m venv "%NSHELL_HOME%venv"
    call "%NSHELL_HOME%venv\Scripts\activate.bat"
    pip install -r "%NSHELL_HOME%requirements.txt"
) else (
    call "%NSHELL_HOME%venv\Scripts\activate.bat"
)

REM Start nshell
echo Starting nshell...
bash "%NSHELL_HOME%bin\nshell.sh"

REM Pause if there was an error
if %ERRORLEVEL% NEQ 0 (
    pause
)
