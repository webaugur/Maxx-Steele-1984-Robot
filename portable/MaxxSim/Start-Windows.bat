@echo off
setlocal
set "ROOT=%~dp0"
set "BIN=%ROOT%windows\maxx.exe"
if not exist "%BIN%" (
  echo Missing %BIN%
  echo Download the Windows artifact from GitHub Actions into windows\
  exit /b 1
)
if exist "%ROOT%carts\default.532" (
  "%BIN%" simulate --gui "%ROOT%carts\default.532"
) else (
  "%BIN%" simulate --gui
)
