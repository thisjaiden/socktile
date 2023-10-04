echo "[1/4] Removing old builds..."
rm -d -r testenv
rm -d -r dist

echo "[2/4] Building..."
# Build std exe for server
cargo build
# Build WASM binary
cargo build --release --target wasm32-unknown-unknown
# Bindgen it
wasm-bindgen --out-name socktile --out-dir dist \
  --target web ./target/wasm32-unknown-unknown/release/socktile.wasm

echo "[3/4] Copying files..."
# Create directories
mkdir -p testenv/server
mkdir dist
# Copy files
cp target/debug/socktile testenv/server/socktile
cp index.html dist/index.html
cp -a assets dist/assets

echo "[4/4] Starting programs..."
# This is janky but it gets the job done. Not sure how to do this cleaner?
# Starts the server
osascript -e 'tell app "Terminal"
  do script "cd ~/Desktop/socktile/testenv/server; ./socktile server"
end tell'
# Starts a websever on localhost:4000
basic-http-server dist
