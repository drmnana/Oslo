---
title: Installation
---

This guide is for reference only, please check the latest guides on getting starting with 
[the Oslo network](https://github.com/drmnana/Oslo/blob/main/docs).

This page will guide you through the **2 steps** needed to prepare a computer for development on **Substrate**
networks like Oslo. Since Substrate is built with [the Rust programming language](https://www.rust-lang.org/), 
the first thing you will need to do is prepare the computer for Rust development - these steps will vary based
on the computer's operating system. Once Rust is configured, you will use its toolchains to interact
with Rust projects; the commands for Rust's toolchains will be the same for all supported,
Unix-based operating systems.

## Build dependencies

Substrate development is easiest on Unix-based operating systems like macOS or Linux. The examples
in the [Polkadot Docs](https://wiki.polkadot.network/) use Unix-style terminals to demonstrate how to
interact with Substrate from the command line.

### Ubuntu/Debian

Use a terminal shell to execute the following commands:

```bash
sudo apt update
# May prompt for location information
sudo apt install -y git clang curl libssl-dev llvm libudev-dev
```

### Arch Linux

Run these commands from a terminal:

```bash
pacman -Syu --needed --noconfirm curl git clang
```

### Fedora

Run these commands from a terminal:

```bash
sudo dnf update
sudo dnf install clang curl git openssl-devel
```

### OpenSUSE

Run these commands from a terminal:

```bash
sudo zypper install clang curl git openssl-devel llvm-devel libudev-devel
```

### macOS

> **Apple M1 ARM**
> If you have an Apple M1 ARM system on a chip, make sure that you have Apple Rosetta 2
> installed through `softwareupdate --install-rosetta`. This is only needed to run the
> `protoc` tool during the build. The build itself and the target binaries would remain native.

Open the Terminal application and execute the following commands:

```bash
# Install Homebrew if necessary https://brew.sh/
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh)"

# Make sure Homebrew is up-to-date, install openssl
brew update
brew install openssl
```

### Windows

**_PLEASE NOTE:_** Native Windows development of Substrate is _not_ very well supported! It is _highly_
recommend to use [Windows Subsystem Linux](https://learn.microsoft.com/en-us/windows/wsl/install)
(WSL) and follow the instructions for [Ubuntu/Debian](#ubuntudebian).

## Rust developer environment

This guide uses <https://rustup.rs> installer and the `rustup` tool to manage the Rust toolchain.
First install and configure `rustup`:

```bash
# Install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Configure
source ~/.cargo/env
```

Configure the Rust toolchain to default to the latest stable version and the wasm target:

```bash
rustup default stable
rustup update
rustup target add wasm32-unknown-unknown
```

## Test your set-up

Now the best way to ensure that you have successfully prepared a computer for running an Oslo node
is to follow the steps in the [Oslo setup instructions](https://github.com/drmnana/Oslo/docs/Oslo-setup-instructions.txt).

## Troubleshooting Substrate builds

Sometimes you can't get the Substrate node template
to compile out of the box. Here are some tips to help you work through that.

### Rust configuration check

To see what Rust toolchain you are presently using, run:

```bash
rustup show
```

This will show something like this (Ubuntu example) output:

```text
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/user/.rustup

installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu (default)

installed targets for active toolchain
--------------------------------------

wasm32-unknown-unknown
x86_64-unknown-linux-gnu

active toolchain
----------------

stable-x86_64-unknown-linux-gnu (default)
rustc 1.85.0 (cb75ad5db 2021-02-10)
```

As you can see above, the default toolchain is stable and its `wasm32-unknown-unknown` target is installed.

### WebAssembly compilation

Substrate uses [WebAssembly](https://webassembly.org) (Wasm) to produce portable blockchain
runtimes. 

> NOTE: It may be necessary to occasionally rerun `rustup update` if a change in the upstream Substrate
> codebase depends on a new feature of the Rust compiler. When you do this, both your nightly
> and stable toolchains will be pulled to the most recent release.