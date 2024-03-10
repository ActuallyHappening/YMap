let APP_NAME = open Cargo.toml | get package.name | to text;
let BUNDLE_ID = open Cargo.toml | get package.metadata.bundle.identifier | to text

cargo bundle --target aarch64-apple-ios-sim
do --ignore-errors { xcrun simctl boot "iPad (10th generation)" }
^open /Applications/Xcode.app/Contents/Developer/Applications/Simulator.app
xcrun simctl install booted $"target/aarch64-apple-ios-sim/debug/bundle/ios/($APP_NAME).app"
xcrun simctl launch --console booted $"($BUNDLE_ID)"

ios-deploy --debug --id "00008103-001560CE01E3401E" --bundle $"target/aarch64-apple-ios-sim/debug/bundle/ios/infi-map.app"