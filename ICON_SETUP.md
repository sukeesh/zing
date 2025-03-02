# Application Icon Setup for Zing

This document explains how to set up and build Zing with a custom application icon.

## Creating Icon Files

1. Create or download a PNG image for your icon (recommended size: 256x256 pixels)
2. Save it as `assets/icon.png` in the project directory

### Platform-Specific Icons

#### macOS

For macOS, you'll need an `.icns` file:

```bash
# Install the required tool (on macOS)
brew install makeicns

# Convert your PNG to ICNS
makeicns -in assets/icon.png -out assets/icon.icns
```

#### Windows

For Windows, you'll need an `.ico` file:

```bash
# You can use online converters or tools like ImageMagick
convert assets/icon.png -define icon:auto-resize=256,128,64,48,32,16 assets/icon.ico
```

## Building with Icon

### macOS

To create a macOS application bundle with the icon:

```bash
# Install cargo-bundle
cargo install cargo-bundle

# Create the bundle
cargo bundle --release

# The bundled app will be in target/release/bundle/osx/
```

### Windows and Linux

The icon will be automatically used when running the application normally:

```bash
cargo run --release
```

## Troubleshooting

If the icon doesn't appear:

1. Make sure the icon file paths in `Cargo.toml` are correct
2. For macOS, ensure the `.icns` file is properly formatted
3. For Windows, ensure the `.ico` file contains multiple resolutions
4. Rebuild the application completely after adding the icon files 