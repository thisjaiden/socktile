echo "[1/3] Building..."
cargo build
echo "[2/3] Copying files..."
rm -d -r testenv
mkdir -p testenv/server
mkdir testenv/client
cp target/debug/socktile testenv/client/socktile
cp target/debug/socktile testenv/server/socktile
echo "[3/3] Starting programs..."
open testenv/client/socktile
./testenv/server/socktile server
