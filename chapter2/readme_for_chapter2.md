# Chapter 2: Development Environment Setup  
**macOS (M3 Pro) 上で Rust 開発環境を整える**

---

## p. 34 – 36：開発環境の差分

筆者は Debian / x86_64 環境だが、私は **macOS (M3 Pro, arm64)** で作業する。  
Rust のインストール手順は筆者と同じコマンドで実行できた：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"
rustup --version

ビルドに必要なツールのインストール
筆者は Debian 系で

sudo apt install -y build-essential qemu-system-x86 netcat-openbsd
を実行しているが、macOS では次で代用できた。

# Xcode Command Line Tools（clang / make など）
xcode-select --install

# 以降必要に応じて Homebrew:
# brew install qemu        # x86_64 エミュレーション
# brew install nasm        # アセンブラ