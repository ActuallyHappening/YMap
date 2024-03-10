let APP_NAME = open Cargo.toml | select package.name;
let BUNDLE_ID = open Cargo.toml | select package.metadata.bundle.identifier

cargo bundle --target aarch64-apple-ios-sim
xcrun simctl boot "iPhone 12 mini"
open /Applications/Xcode.app/Contents/Developer/Applications/Simulator.app
xcrun simctl install booted "target/aarch64-apple-ios-sim/debug/bundle/ios/$APP_NAME.app"
xcrun simctl launch --console booted "$BUNDLE_ID"
