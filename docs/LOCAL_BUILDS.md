# Building Release Snapshots Locally with GoReleaser

This guide explains how to build release snapshots locally using GoReleaser on different operating systems.

## Prerequisites

### All Platforms
- **Go** (1.21 or later) - [Install Go](https://golang.org/doc/install)
- **GoReleaser** - Install via:
  ```bash
  go install github.com/goreleaser/goreleaser/v2@latest
  ```
- **Rust** (stable toolchain) - [Install Rust](https://rustup.rs/)
- **Git** (repository must be initialized)

### Platform-Specific Requirements

#### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install -y musl-tools
```

#### macOS
```bash
# Xcode Command Line Tools (if not already installed)
xcode-select --install

# Add target architectures
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

#### Windows
```bash
# Add target architectures
rustup target add x86_64-pc-windows-gnu
rustup target add i686-pc-windows-gnu

# Install MSYS2 or MinGW-w64 for GNU toolchain
# Download from: https://www.msys2.org/
```

### Install Rust Dependencies (All Platforms)
```bash
rustup default stable
cargo install --locked cargo-zigbuild
cargo fetch --locked
```

---

## Building Snapshots by Platform

Snapshots are local builds without publishing. They're perfect for testing before a real release.

### Linux Builds

From the repository root:

```bash
# Build Linux binaries locally
goreleaser release --snapshot --clean --skip=publish --config .goreleaser-linux.yaml
```

**Output location:** `dist/`

**What gets built:**
- x86_64-unknown-linux-gnu
- aarch64-unknown-linux-gnu
- i686-unknown-linux-gnu
- armv7-unknown-linux-gnueabihf
- arm-unknown-linux-gnueabihf
- `.deb`, `.rpm`, `.apk` packages

---

### macOS Builds

From the repository root:

```bash
# Build macOS binaries locally
goreleaser release --snapshot --clean --skip=publish --config .goreleaser-macos.yaml
```

**Output location:** `dist/`

**What gets built:**
- x86_64-apple-darwin
- aarch64-apple-darwin
- Universal binary (combined x86_64 + aarch64)

**Note:** You can only build macOS binaries on a macOS machine due to native linking requirements.

---

### Windows Builds

From the repository root:

```bash
# Build Windows binaries locally
goreleaser release --snapshot --clean --skip=publish --config .goreleaser-windows.yaml
```

**Output location:** `dist/`

**What gets built:**
- x86_64-pc-windows-gnu
- i686-pc-windows-gnu
- `.zip` archives

**Note:** You can only build Windows binaries on a Windows machine due to native linking requirements.

---

## Cross-Platform Testing Strategy

Since Rust cannot cross-compile between OS families for this project, use this approach:

### Option 1: Multi-Machine Setup
1. **Linux machine/VM:** Build Linux artifacts
2. **macOS machine:** Build macOS artifacts  
3. **Windows machine/VM:** Build Windows artifacts

### Option 2: Docker/CI-like Environment
```bash
# Linux (can use Docker on any OS)
docker run --rm -v "$PWD":/work -w /work \
  ghcr.io/goreleaser/goreleaser-cross:v1.22 \
  release --snapshot --clean --skip=publish --config .goreleaser-linux.yaml
```

### Option 3: GitHub Actions (Recommended)
Push to a test branch and let CI build everything:
```bash
git tag v0.0.0-test
git push origin v0.0.0-test

# Download artifacts from GitHub Actions after builds complete
# Delete tag when done: git tag -d v0.0.0-test && git push origin :refs/tags/v0.0.0-test
```

---

## Understanding the Output

After running a snapshot build, check the `dist/` directory:

```
dist/
├── wstunnel_linux_amd64/
├── wstunnel_linux_arm64/
├── wstunnel_*.tar.gz          # Archives
├── wstunnel_*.deb             # Linux packages
├── wstunnel_*.rpm
├── wstunnel_*.apk
├── checksums.txt              # SHA256 checksums
└── artifacts.json             # Build metadata
```

---

## Common Commands Reference

```bash
# Snapshot build (no publish, no version validation)
goreleaser release --snapshot --clean --skip=publish --config .goreleaser-<OS>.yaml

# Dry run (see what would happen without building)
goreleaser release --skip=publish --skip=validate --config .goreleaser-<OS>.yaml

# Check configuration validity
goreleaser check --config .goreleaser-<OS>.yaml

# Build only (skip archives/packages)
goreleaser build --snapshot --clean --config .goreleaser-<OS>.yaml

# Clean dist directory
rm -rf dist/
```

---

## Troubleshooting

### "cargo-zigbuild not found"
```bash
cargo install --locked cargo-zigbuild
```

### "target not found" error
```bash
rustup target add <target-triple>
# Example: rustup target add aarch64-apple-darwin
```

### Permission denied on Linux packages
```bash
# Building .deb/.rpm requires root in some cases
sudo goreleaser release --snapshot --clean --skip=publish --config .goreleaser-linux.yaml
```

### Windows: "linker 'cc' not found"
Install MinGW-w64 or MSYS2 and ensure it's in your PATH:
```bash
# MSYS2
pacman -S mingw-w64-x86_64-gcc

# Add to PATH: C:\msys64\mingw64\bin
```

### macOS: Universal binary fails
Ensure both targets are installed:
```bash
rustup target add x86_64-apple-darwin aarch64-apple-darwin
```

---

## CI/CD vs Local Builds

| Aspect | Local Snapshot | CI/CD Release |
|--------|----------------|---------------|
| Purpose | Testing, validation | Production release |
| Requires tag | No | Yes (v*) |
| Publishes to GitHub | No | Yes |
| Runs on | Your machine | GitHub runners |
| Speed | Single OS only | All OS in parallel |
| Use case | Development | Official releases |

---

## Best Practices

1. **Always clean before building**: Use `--clean` flag to avoid stale artifacts
2. **Test locally first**: Run snapshot builds before tagging releases
3. **Verify checksums**: Check `dist/checksums.txt` after builds
4. **Keep configs in sync**: If you modify build flags, update all three configs
5. **Use version control**: Test with `--snapshot` before pushing tags

---

## Quick Start Example

**On your native OS:**

```bash
# 1. Ensure prerequisites are installed
go version
rustup --version
goreleaser --version

# 2. Navigate to repository
cd /path/to/wstunnel

# 3. Build snapshot for your OS
# Linux:
goreleaser release --snapshot --clean --skip=publish --config .goreleaser-linux.yaml

# macOS:
goreleaser release --snapshot --clean --skip=publish --config .goreleaser-macos.yaml

# Windows:
goreleaser release --snapshot --clean --skip=publish --config .goreleaser-windows.yaml

# 4. Check output
ls -lh dist/
```

---

## Additional Resources

- [GoReleaser Documentation](https://goreleaser.com/)
- [Rust Cross-Compilation Guide](https://rust-lang.github.io/rustup/cross-compilation.html)
- [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild)
