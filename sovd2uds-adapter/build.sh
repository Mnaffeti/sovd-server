#!/bin/bash
# Build script for SOVD2UDS Adapter

set -e

echo "Building SOVD2UDS Adapter..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check environment variables
if [ -z "$UDS_INCLUDE_PATH" ]; then
    echo "Warning: UDS_INCLUDE_PATH not set. Using default: ../libudsclient/include"
    export UDS_INCLUDE_PATH="../libudsclient/include"
fi

if [ -z "$DOIP_INCLUDE_PATH" ]; then
    echo "Warning: DOIP_INCLUDE_PATH not set. Using default: ../libdoipclient/include"
    export DOIP_INCLUDE_PATH="../libdoipclient/include"
fi

echo "UDS_INCLUDE_PATH: $UDS_INCLUDE_PATH"
echo "DOIP_INCLUDE_PATH: $DOIP_INCLUDE_PATH"

# Build type (default: release)
BUILD_TYPE=${1:-release}

if [ "$BUILD_TYPE" = "debug" ]; then
    echo "Building in debug mode..."
    cargo build
    echo "Build complete! Binary: target/debug/sovd2uds-adapter"
else
    echo "Building in release mode..."
    cargo build --release
    echo "Build complete! Binary: target/release/sovd2uds-adapter"
fi

# Run tests
echo ""
echo "Running tests..."
cargo test

echo ""
echo "Build successful!"
