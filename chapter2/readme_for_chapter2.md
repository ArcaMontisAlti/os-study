# Chapter 2: Development Environment Setup  
**macOS (M3 Pro) 上で Rust 開発環境を整える**

---

## p. 34 – 36：開発環境のセットアップの差異

筆者は Debian / x86_64 環境だが、私は **macOS (M3 Pro, arm64)** で作業する。  
Rust のインストール手順は筆者と同じコマンドで実行できた：

```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

$ . "$HOME/.cargo/env"

$ rustup --version
```

## ビルドに必要なツールのインストール
筆者は Debian 系で

sudo apt install -y build-essential qemu-system-x86 netcat-openbsd
を実行しているが、macOS では次で代用できた。

`$ xcode-select --install`

以降必要に応じて Homebrew:

`$ brew install qemu`   

`$ brew install nasm`       

## GNU Make を最新版へ更新

```bash
# 1. 現在のバージョン確認
make --version          # => GNU Make 3.81 …

# 2. Homebrew で最新版をインストール（インストール名は gmake）
brew install make
gmake --version         # => GNU Make 4.4.1 …

# 3. gmake を make として使うエイリアスを設定（zsh）
echo 'alias make=gmake' >> ~/.zshrc
source ~/.zshrc

# 4. 新しい make を確認
make --version          # => GNU Make 4.4.1 …
```

## p.39 のRustツールチェインのバージョンを固定する　の部分の差異

私の方は,

`$ rustup show active-toolchain`

をすると

`$ nightly-2024-01-01-aarch64-apple-darwin (overridden by '/Users/takayamanoa/os-study/repo/wasabi/rust-toolchain.toml')`

になっている. 特に問題はなかった.echo '/repo/wasabi' >> .gitignore