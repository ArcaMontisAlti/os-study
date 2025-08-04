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
