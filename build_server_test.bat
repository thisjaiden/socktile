echo off
cd "%~dp0"
echo "[1/3] Building..."
cargo build --release
echo "[2/3] Copying files..."
rmdir /q /s "testenv"
mkdir "testenv\server"
copy "target\release\socktile.exe" "testenv\client_a\socktile.exe"
copy "target\release\socktile.exe" "testenv\client_b\socktile.exe"
copy "target\release\socktile.exe" "testenv\server\socktile.exe"
echo "[3/3] Starting programs..."
start /d "testenv\server" socktile.exe --ggs
timeout /t 5 /nobreak
start /d "testenv\client_a" socktile.exe
timeout /t 1 /nobreak
start /d "testenv\client_b" socktile.exe
echo "All done!"
