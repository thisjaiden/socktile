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
echo "[3/3] Starting programs..."
start /d "testenv\server" socktile.exe --ggs
timeout /t 5 /nobreak
start /d "testenv\client" socktile.exe
echo "All done!"
