#!/bin/bash

# Setup
RUN_BUILD_RS=1
BUILD_DIR=platform-build
mkdir $BUILD_DIR
cd $BUILD_DIR

# Build static libs
for TARGET in \
        aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim \
        x86_64-apple-darwin aarch64-apple-darwin
do
    rustup target add $TARGET
    # Apple's App Sandbox disallows SysV semaphores; use POSIX semaphores instead
    # cargo build -r --target=$TARGET --features posix-sem
    RUSTFLAGS='--cfg chacha20_force_neon' cargo build -r --target=$TARGET
done

# List all files from target/aarch64-apple-darwin
ls -Rh ../target/aarch64-apple-darwin

# Create XCFramework zip
FRAMEWORK="EmbeddedRustyChacha.xcframework"
LIBNAME=libembedded_rusty_chacha.a
mkdir mac-lipo ios-sim-lipo
IOS_SIM_LIPO=ios-sim-lipo/$LIBNAME
MAC_LIPO=mac-lipo/$LIBNAME
lipo -create -output $IOS_SIM_LIPO \
        ../target/aarch64-apple-ios-sim/release/$LIBNAME \
        ../target/x86_64-apple-ios/release/$LIBNAME
lipo -create -output $MAC_LIPO \
        ../target/aarch64-apple-darwin/release/$LIBNAME \
        ../target/x86_64-apple-darwin/release/$LIBNAME
xcodebuild -create-xcframework \
        -library $IOS_SIM_LIPO \
        -library $MAC_LIPO \
        -library ../target/aarch64-apple-ios/release/$LIBNAME \
        -output $FRAMEWORK
zip -r $FRAMEWORK.zip $FRAMEWORK

# Cleanup
rm -rf ios-sim-lipo mac-lipo $FRAMEWORK
