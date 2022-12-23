echo "[1/3] Building..."
if [ $# -eq 1 ]
  then
    cargo clean
fi
cargo build
trunk build
echo "[2/3] Copying files..."
rm -d -r testenv
mkdir -p testenv/server
cp target/debug/socktile testenv/server/socktile
echo "[3/3] Starting programs..."
# this is janky but it gets the job done. Not sure how to do this cleaner?
osascript -e 'tell app "Terminal"
  do script "cd ~/Desktop/socktile/testenv/server; ./socktile server"
end tell'
trunk serve
