#!/bin/bash

# Setup
RUN_BUILD_RS=1
BUILD_DIR=platform-build
mkdir $BUILD_DIR
cd $BUILD_DIR

# Install build dependencies
cargo install cargo-zigbuild
cargo install cargo-xwin

zig_build () {
    local TARGET="$1"
    local PLATFORM_NAME="$2"
    local LIBNAME="$3"
    rustup target add "$TARGET"
    cargo zigbuild --target "$TARGET" -r
    mkdir "$PLATFORM_NAME"
    cp "../target/$TARGET/release/$LIBNAME" "$PLATFORM_NAME/"
}

win_build () {
    local TARGET="$1"
    local PLATFORM_NAME="$2"
    local LIBNAME="$3"
    rustup target add "$TARGET"
    cargo xwin build --target "$TARGET" -r
    mkdir "$PLATFORM_NAME"
    cp "../target/$TARGET/release/$LIBNAME" "$PLATFORM_NAME/"
}

win_build_gnu () {
    local PLATFORM_NAME="$1"
    local LIBNAME="$2"
    brew install mingw-w64
    rustup target add x86_64-pc-windows-gnu
    cargo build --target=x86_64-pc-windows-gnu -r
    mkdir "$PLATFORM_NAME"
    cp "../target/x86_64-pc-windows-gnu/release/$LIBNAME" "$PLATFORM_NAME/"
}

# Build all the dynamic libraries
LINUX_LIBNAME=libembedded_rusty_chacha.so
zig_build aarch64-unknown-linux-gnu linux-arm64 $LINUX_LIBNAME
zig_build x86_64-unknown-linux-gnu linux-x64 $LINUX_LIBNAME
WINDOWS_LIBNAME=embedded_rusty_chacha.dll
#win_build aarch64-pc-windows-msvc windows-arm64 $WINDOWS_LIBNAME
#win_build x86_64-pc-windows-msvc windows-x64 $WINDOWS_LIBNAME
win_build_gnu windows-x64 $WINDOWS_LIBNAME

# Archive the dynamic libs
tar -czvf EmbeddedRustyChacha.tar.gz linux-* windows-*

# Cleanup
rm -rf linux-* windows-*
