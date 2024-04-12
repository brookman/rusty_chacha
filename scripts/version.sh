#!/bin/bash

CURR_VERSION=rusty_chacha-v`awk '/^version: /{print $2}' packages/rusty_chacha/pubspec.yaml`

# iOS & macOS
APPLE_HEADER="release_tag_name = '$CURR_VERSION' # generated; do not edit"
sed -i.bak "1 s/.*/$APPLE_HEADER/" packages/rusty_chacha/ios/rusty_chacha.podspec
sed -i.bak "1 s/.*/$APPLE_HEADER/" packages/rusty_chacha/macos/rusty_chacha.podspec
rm packages/rusty_chacha/macos/*.bak packages/rusty_chacha/ios/*.bak

# CMake platforms (Linux, Windows, and Android)
CMAKE_HEADER="set(rustyChachaVersion \"$CURR_VERSION\") # generated; do not edit"
for CMAKE_PLATFORM in android linux windows
do
    sed -i.bak "1 s/.*/$CMAKE_HEADER/" packages/rusty_chacha/$CMAKE_PLATFORM/CMakeLists.txt
    rm packages/rusty_chacha/$CMAKE_PLATFORM/*.bak
done

git add packages/rusty_chacha/
