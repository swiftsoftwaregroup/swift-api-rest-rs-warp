# Setup for macOS

One time setup on macOS

> Scripts are `bash`

## Xcode / Compilers

Install Command Line Tools (CLT) for Xcode:

```bash
xcode-select --install
```

## Homebrew

Install Homebrew:

```bash
/usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
```

## bash 5

```bash
brew install bash
```

### Rust

Install `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Test:

```bash
source $HOME/.cargo/env
rustc --help
```

