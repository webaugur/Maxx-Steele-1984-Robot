@echo off
setlocal
set "ROOT=%~dp0"
set "BIN=%ROOT%maxx\windows\maxx.exe"
if not exist "%BIN%" (
  echo Missing %BIN%
  echo The Windows program belongs in maxx\windows\
  exit /b 1
)
if not "%~1"=="" (
  "%BIN%" simulate --gui "%~1"
  exit /b %ERRORLEVEL%
)
if exist "%ROOT%carts\default.532" (
  "%BIN%" simulate --gui "%ROOT%carts\default.532"
) else (
  "%BIN%" simulate --gui
)
