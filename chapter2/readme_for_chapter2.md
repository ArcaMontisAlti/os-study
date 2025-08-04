# Chapter 2: Development Environment Setup  
**macOS (M3 Pro) 上で Rust 開発環境を整えるメモ / Setup Rust toolchain on macOS**

## Summary / 概要
本書の筆者は Debian/x86_64 環境（`apt` 前提）で進めているが、ここでは macOS M3 Pro（arm64）上で同等の Rust 開発環境を作る。  
違いを押さえつつ、動作確認まで一貫して再現できるようにする。

## Environment Differences / 本と違う点
- 筆者：`Linux 6.6.32 … x86_64`, `Debian 12.4`, `apt` ベース  
- 自分：`Darwin 24.5.0 arm64`（Apple Silicon / M3 Pro）、`apt` や `/etc/debian_version` は存在しない。  
  → `rustup` と Homebrew 等を使う。

## Prerequisites / 前提条件
1. **Command Line Tools（ビルドツール類）を入れる**
   ```sh
   xcode-select --install
