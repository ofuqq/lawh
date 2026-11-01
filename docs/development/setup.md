# Developer Setup Guide

This guide walks you through setting up a local Rust development environment to build, test, and contribute to Ruqa.

---

## 1. Prerequisites

### Rust Toolchain
Ruqa requires the stable Rust toolchain (version 1.80.0 or newer). Install it via official `rustup`:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify your installation:
```bash
rustc --version
cargo --version
```

### Required Cargo Components & Tools
Install `rustfmt` (code formatter), `clippy` (linter), and `cargo-deny` (license & dependency auditor):
```bash
rustup component add rustfmt clippy
cargo install --locked cargo-deny
```

### Linux System Dependencies (for GUI windowing)
Building native desktop windows on Linux requires development headers for X11 / Wayland:
```bash
# Ubuntu / Debian
sudo apt-get update && sudo apt-get install -y \
    build-essential \
    libx11-dev \
    libxcursor-dev \
    libxrandr-dev \
    libxi-dev \
    libxkbcommon-dev

# Arch Linux / Manjaro
sudo pacman -S --needed base-devel libx11 libxcursor libxrandr libxi libxkbcommon
```

---

## 2. Cloning the Repository

```bash
git clone https://github.com/ofuqq/ruqa.git
cd ruqa
```

---

## 3. Verified Build, Test & Lint Commands

Run these exact commands to verify your workspace is healthy before opening a pull request:

```bash
# Check code formatting (Tested in CI)
cargo fmt --all --check

# Run Clippy with warnings treated as errors (Tested in CI)
cargo clippy --workspace --all-targets -- -D warnings

# Run all unit and integration tests (Tested in CI)
cargo test --workspace

# Verify licenses and bans (Tested in CI)
cargo deny check licenses bans sources
```

---

## 4. Running the Application

> [!NOTE]
> The desktop windowing runtime is under active construction in Milestone M1. Until the M1 harness lands, the command below is marked as **untested** in local environments:

```bash
# Launch experimental desktop runner (Untested until M1)
cargo run -p ruqa-app
```
