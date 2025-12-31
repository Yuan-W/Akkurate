# Windows Build Instructions

Due to Linux-specific dependencies (D-Bus for clipboard), cross-compilation is not straightforward.

## Option 1: Build on Windows (Recommended)

### Prerequisites
1. Install [Rust](https://rustup.rs/)
2. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with C++ workload

### Build Steps
```powershell
# Clone or copy the project
git clone https://github.com/yourusername/akkurate.git
cd akkurate

# Build release
cargo build --release

# The executable will be at: target\release\akkurate.exe
```

### Create Distribution
```powershell
mkdir akkurate-release
copy target\release\akkurate.exe akkurate-release\
copy -r assets akkurate-release\
copy README.md akkurate-release\
Compress-Archive -Path akkurate-release -DestinationPath akkurate-windows.zip
```

## Option 2: GitHub Actions CI/CD

Add `.github/workflows/release.yml` for automated cross-platform builds:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build-linux:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --release
      - uses: actions/upload-artifact@v4
        with:
          name: akkurate-linux
          path: target/release/akkurate

  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --release
      - uses: actions/upload-artifact@v4
        with:
          name: akkurate-windows
          path: target/release/akkurate.exe
```

## Notes

- Windows version uses native Windows clipboard API (no wl-clipboard needed)
- The `-s` (selection) feature only works on Linux/Wayland
- On Windows, use the main GUI interface or `--check "text"` argument
