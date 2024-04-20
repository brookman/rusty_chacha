set(rustyChachaVersion "rusty_chacha-v0.1.1") # generated; do not edit

# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

# Project-level configuration.
set(PROJECT_NAME "rusty_chacha")
project(${PROJECT_NAME} LANGUAGES CXX)

# Download the binaries if they are not already present.
set(RustyChahaLibRoot "${CMAKE_CURRENT_SOURCE_DIR}/${rustyChachaVersion}")
set(RustyChahaArchivePath "${RustyChahaLibRoot}.tar.gz")
if(NOT EXISTS ${RustyChahaArchivePath})
  file(DOWNLOAD
    "https://github.com/brookman/rusty_chacha/releases/download/${rustyChachaVersion}/EmbeddedRustyChacha.tar.gz"
    ${RustyChahaArchivePath}
    TLS_VERIFY ON
  )
endif()

# Extract the binaries, overriding any already present.
file(REMOVE_RECURSE ${RustyChahaLibRoot})
file(MAKE_DIRECTORY ${RustyChahaLibRoot})
execute_process(
  COMMAND ${CMAKE_COMMAND} -E tar xzf ${RustyChahaArchivePath}
  WORKING_DIRECTORY ${RustyChahaLibRoot}
)

# List of absolute paths to libraries that should be bundled with the plugin.
# This list could contain prebuilt libraries, or libraries created by an
# external build triggered from this build file.
set(rusty_chacha_bundled_libraries
  "${RustyChahaLibRoot}/${FLUTTER_TARGET_PLATFORM}/libembedded_rusty_chacha.so"
  PARENT_SCOPE
)
