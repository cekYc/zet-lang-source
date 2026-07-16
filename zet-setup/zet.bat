@echo off
setlocal enabledelayedexpansion
color 0A

REM =============================================
REM   ZET LANG v0.4 — Evrensel Baslatici
REM   Herhangi bir klasorden calisir.
REM   Bosluklu ve parantezli yollari destekler.
REM =============================================

REM --- Zet'in kurulu oldugu yer (zet.bat'in yani) ---
set "ZET_HOME=%~dp0"
set "RUNTIME_DIR=%ZET_HOME%runtime"
set "COMPILER=%ZET_HOME%bin\zet-compiler.exe"

REM --- Derleyici mevcut mu? ---
if not exist "%COMPILER%" goto :no_compiler

REM --- Runtime klasoru mevcut mu? ---
if not exist "%RUNTIME_DIR%\Cargo.toml" goto :no_runtime

REM --- Dosya argumani ---
if "%~1"=="" goto :interactive
set "USER_FILE=%~f1"
goto :run

:interactive
echo.
echo   ============================================
echo     ZET LANG v0.4
echo     Zero Trust - Guvenmedigin Kodu Derleme
echo   ============================================
echo.
echo   Kullanim:  zet dosya.zt
echo   Ornek:     zet ornek.zt
echo.
echo   Veya bir .zt dosyasini bu ikona surukleyin.
echo.
set /p "USER_INPUT=  Dosya yolu girin> "
if "!USER_INPUT!"=="" goto :eof
set "USER_FILE=!USER_INPUT:"=!"
goto :run

:run
REM --- Dosya var mi? ---
if not exist "%USER_FILE%" goto :no_file

REM --- Kullanicinin .zt dosyasini runtime'a kopyala ---
copy /Y "%USER_FILE%" "%RUNTIME_DIR%\run.zt" >nul 2>&1

REM --- Runtime dizinine gir ve derle ---
pushd "%RUNTIME_DIR%"
"%COMPILER%" run.zt
set "EXIT_CODE=!ERRORLEVEL!"
popd

REM --- Eger cift tiklama ile acildiysa bekle ---
if "%~1"=="" (
    echo.
    echo Cikis icin bir tusa basin...
    pause >nul
)

endlocal
exit /b %EXIT_CODE%

:no_compiler
color 0C
echo [HATA] zet-compiler.exe bulunamadi!
echo Beklenen konum: %COMPILER%
echo Lutfen kurulumu tekrar yapin.
pause
exit /b 1

:no_runtime
color 0C
echo [HATA] Runtime klasoru bulunamadi!
echo Beklenen konum: %RUNTIME_DIR%
echo Lutfen kurulumu tekrar yapin.
pause
exit /b 1

:no_file
color 0C
echo [HATA] Dosya bulunamadi: %USER_FILE%
pause
exit /b 1
