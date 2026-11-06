@echo off
setlocal
set "ROOT=%~dp0"
set "BIN=%ROOT%maxx\windows\maxx.exe"
set "ICON=%ROOT%icons\rom-chip.ico"
if not exist "%BIN%" (
  echo Missing %BIN%
  exit /b 1
)
reg add "HKCU\Software\Classes\.532" /ve /d "Maxx.Cartridge" /f >nul
reg add "HKCU\Software\Classes\Maxx.Cartridge" /ve /d "Maxx Steele cartridge" /f >nul
reg add "HKCU\Software\Classes\Maxx.Cartridge\DefaultIcon" /ve /d "%ICON%,0" /f >nul
reg add "HKCU\Software\Classes\Maxx.Cartridge\shell\open\command" /ve /d "\"%BIN%\" simulate --gui \"%%1\"" /f >nul
echo Registered .532 -^> %BIN% simulate --gui
echo Run this again if the USB drive letter changes.
