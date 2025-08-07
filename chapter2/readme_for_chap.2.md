# Chapter 2: OS のない世界でプログラムを書く 
**macOS (M3 Pro) 上で行う上で, 教科書通りに運ばなかった部分に対し私がどう対処したかを記載しています**

---

## p. 34 – 36：開発環境のセットアップ

筆者は Debian / x86_64 環境だが, 私は **macOS (M3 Pro, arm64)** で作業する.

Rust のインストール手順は筆者と同じコマンドで実行できた：

```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

$ . "$HOME/.cargo/env"

$ rustup --version
```

## ビルドに必要なツールのインストール
筆者は Debian 系で

`$ sudo apt install -y build-essential qemu-system-x86 netcat-openbsd`

を実行しているが, **私はできなかった.** その代わり, macOS では次で解決できた.

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

## p.39 Rustツールチェインのバージョンを固定する

私の方は,

`$ rustup show active-toolchain`

をすると

`$ nightly-2024-01-01-aarch64-apple-darwin(overridden by '/Users/takayamanoa/os-study/repo/wasabi/rust-toolchain.toml')`

になっている. 特に問題はなかった.

## p. 45 Rust ツールチェインバージョン固定

私の環境では以下のように出力されるが, 本の手順通りにバージョン固定が適用されている：

```bash
$ rustup show active-toolchain
nightly-2024-01-01-aarch64-apple-darwin (overridden by '/Users/takayamanoa/os-study/repo/wasabi/rust-toolchain.toml')
```

## p.59 \~ 67 フレームバッファに何か描く

```rust
#[panic_handler] // panic_handler を定義することで、パニック時の挙動を定義する
fn panic(_info: &PanicInfo) -> ! {
    loop {} // パニック時は無限ループに入る
}
```

> **私の環境（rust-analyzer導入済）では赤線が引かれてしまうのだが, コンパイル・実行には影響がなかった.**
> 消すとコンパイルエラーになるため, 必ず残して使う.

> 2025-08-08 追記: 本書を読み進めていくうちに, 上記のエラーがエラーでなくなった.