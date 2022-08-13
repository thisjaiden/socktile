echo off
cd "%~dp0"
echo "[1/3] Building..."
cargo build
echo "[2/3] Copying files..."
rmdir /q /s "testenv"
mkdir "testenv\server"
mkdir "testenv\client"
copy "target\debug\socktile.exe" "testenv\client\socktile.exe"
copy "target\debug\socktile.exe" "testenv\server\socktile.exe"
:: This is needed while modular assets loads subassets of terrain states dynamically
xcopy /q /e /i "assets" "testenv\client\assets"
echo socktile.exe server > testenv\server\launch.bat
echo pause >> testenv\server\launch.bat
echo set RUST_BACKTRACE=full > testenv\client\crash_debug.bat
echo socktile.exe >> testenv\client\crash_debug.bat
echo pause >> testenv\client\crash_debug.bat
echo "[3/3] Starting programs..."
start /d "testenv\server" launch.bat
start /d "testenv\client" crash_debug.bat
echo "All done!"
