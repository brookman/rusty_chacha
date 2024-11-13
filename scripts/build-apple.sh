#!/bin/bash

# This script does the following:
# Build the Rust code with cargo into 5 different dynamic libraries (.dylib) for iOS, macOS (x86 & arm) and Simulator (x86 & arm)
# Use lipo to combine libraries with different architectures for the same platform
# Create a .framework for each dynamic library
# Combine the .frameworks into a single .xcframework


# Setup
RUN_BUILD_RS=1
BUILD_DIR=platform-build
rm -Rf $BUILD_DIR
mkdir $BUILD_DIR
cd $BUILD_DIR


#FRAMEWORK="EmbeddedRustyChacha.xcframework"
FRAMEWORK_NAME="EmbeddedRustyChacha"
LIB_NAME="libembedded_rusty_chacha.dylib"


# Build libs
for TARGET in \
        aarch64-apple-darwin \
        aarch64-apple-ios \
        aarch64-apple-ios-sim \
        x86_64-apple-darwin \
        x86_64-apple-ios
do
    rustup target add $TARGET
    # Apple's App Sandbox disallows SysV semaphores; use POSIX semaphores instead
    # cargo build -r --target=$TARGET --features posix-sem
    RUSTFLAGS='--cfg chacha20_force_neon' cargo build -r --target=$TARGET
done

# Combine libs
LIPO_MACOS=macos-universal
LIPO_SIMULATOR=simulator-universal
rm -Rf ../target/$LIPO_MACOS
rm -Rf ../target/$LIPO_SIMULATOR
mkdir ../target/$LIPO_MACOS
mkdir ../target/$LIPO_SIMULATOR
lipo -create -output ../target/$LIPO_MACOS/$LIB_NAME \
        ../target/aarch64-apple-ios-sim/release/$LIB_NAME \
        ../target/x86_64-apple-ios/release/$LIB_NAME
lipo -create -output ../target/$LIPO_SIMULATOR/$LIB_NAME \
        ../target/aarch64-apple-darwin/release/$LIB_NAME \
        ../target/x86_64-apple-darwin/release/$LIB_NAME


# Function to create framework bundle
create_framework() {
    TARGET_DIR=$1
    PLATFORM=$2
    ARCH=$3

    OUTPUT_DIR="${PLATFORM}-${ARCH}"
    mkdir -p "$OUTPUT_DIR"

    FRAMEWORK_DIR="$OUTPUT_DIR/$FRAMEWORK_NAME.framework"
    BINARY_NAME="$FRAMEWORK_NAME"

    # Create the framework directory
    mkdir -p "$FRAMEWORK_DIR/Versions/A"
    mkdir -p "$FRAMEWORK_DIR/Versions/A/Modules"
    mkdir -p "$FRAMEWORK_DIR/Versions/A/Headers"
    mkdir -p "$FRAMEWORK_DIR/Versions/A/Resources"

    # Copy and rename the dynamic library into the framework
    cp "../target/$TARGET_DIR/$LIB_NAME" "$FRAMEWORK_DIR/Versions/A/$BINARY_NAME"

    # Create Info.plist
    cat > "$FRAMEWORK_DIR/Versions/A/Resources/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" 
    "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>$BINARY_NAME</string>
    <key>CFBundleExecutable</key>
    <string>$BINARY_NAME</string>
    <key>CFBundleIdentifier</key>
    <string>eu32k.$BINARY_NAME</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
    <key>CFBundlePackageType</key>
    <string>FMWK</string>
    <key>CFBundleSignature</key>
    <string>????</string>
    <key>CFBundleSupportedPlatforms</key>
    <array>
        <string>$PLATFORM</string>
    </array>
</dict>
</plist>
EOF

    # Copy header file
    cp "../target/$BINARY_NAME.h" "$FRAMEWORK_DIR/Versions/A/Headers/$BINARY_NAME.h"

    # Create module.modulemap
    cat > "$FRAMEWORK_DIR/Versions/A/Modules/module.modulemap" <<EOF
module $BINARY_NAME {
    header "$BINARY_NAME.h"
    export *
}
EOF

    # Create the symlinks
    cd "$FRAMEWORK_DIR"
    cd Versions
    ln -s A Current
    cd ..
    #ln -l "$BINARY_NAME" "Versions/Current/$BINARY_NAME"
    ln -s Versions/Current/Modules Modules
    ln -s Versions/Current/Headers Headers
    ln -s Versions/Current/Resources Resources
    ln -s Versions/Current/$BINARY_NAME $BINARY_NAME
    cd ../..

    # Adjust the install_name of the dynamic library
    install_name_tool -id "@rpath/$BINARY_NAME.framework/$BINARY_NAME" "$FRAMEWORK_DIR/$BINARY_NAME"

    # Code sign the framework
    codesign --force --sign - "$FRAMEWORK_DIR"
}


create_framework "aarch64-apple-ios/release" "iPhoneOS" "arm64"
create_framework "$LIPO_MACOS" "MacOSX" "universal"
create_framework "$LIPO_SIMULATOR" "iPhoneSimulator" "universal"


# Create XCFramework
xcodebuild -create-xcframework \
    -framework "iPhoneOS-arm64/${FRAMEWORK_NAME}.framework" \
    -framework "MacOSX-universal/${FRAMEWORK_NAME}.framework" \
    -framework "iPhoneSimulator-universal/${FRAMEWORK_NAME}.framework" \
    -output "${FRAMEWORK_NAME}.xcframework"


zip -r $FRAMEWORK_NAME.xcframework.zip $FRAMEWORK_NAME.xcframework
