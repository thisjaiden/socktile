cargo build
rm -d -r testenv
mkdir -p testenv/server
cp target/debug/socktile testenv/server/socktile
osascript -e 'tell app "Terminal"
  do script "cd ~/Desktop/socktile/testenv/server; ./socktile server"
end tell'

cargo bundle --target aarch64-apple-ios-sim
xcrun simctl boot "iPhone 12 mini"  
open /Applications/Xcode.app/Contents/Developer/Applications/Simulator.app 
xcrun simctl install booted "target/aarch64-apple-ios-sim/debug/bundle/ios/socktile.app"
xcrun simctl launch --console booted "patcatgames.socktile"
# TODO inject in ./target/aarch64-apple-ios-sim/debug/bundle/ios/socktile.app/Info.plist:
# <key>UIInterfaceOrientation</key>
# <string>UIInterfaceOrientationLandscapeRight</string>
