# メモ

## p.34 ~ 36 開発環境の構築
私の開発環境は M3 Macbook Pro なので, 筆者の開発環境とは異なる.

p. 34 は, まず
$ xcode - select --install を行ったあと,
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

でインストールできた.

プラスして, 直後にRust関連のツールが実行できる状態にするため
$ . "$HOME/.cargo/env"
をすることを忘れずに！

バージョン表示は、
🚀 rustup --version
rustup 1.28.2 (e4f3ad6f8 2025-04-28)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.88.0 (6b00bc388 2025-06-23)`
🚀 cargo --version
cargo 1.88.0 (873a06493 2025-05-10)
🚀 rustc --version
rustc 1.88.0 (6b00bc388 2025-06-23)

こんな感じでうまく表示された.