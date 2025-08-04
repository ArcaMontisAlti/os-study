# Chapter 2: Development Environment Setup  
**macOS (M3 Pro) 上で Rust 開発環境を整える**

## p. 34 ~ 36 のところ, 私と筆者の開発環境が異なる.

適宜 homebrew を使ったりした.
インストールの

`$ curl --proto...`
とか

`$ . "$HOME/.cargo/env"`

`$ rustup -- version`

とかはコマンド全く同じ!

最後, *ビルドに必要ないくつかのコマンドをパッケージマネージャー経由でインストールする*というところで, 私は

`xcode-select --install`

でうまくいった.

私の場合, 最初
🚀 make --version
GNU Make 3.81
Copyright (C) 2006  Free Software Foundation, Inc.
This is free software; see the source for copying conditions.
There is NO warranty; not even for MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE.

と筆者のものよりかなり古い 'GNU Make' だったので,

'$ brew install make'

からの

🚀 gmake --version
GNU Make 4.4.1
Built for aarch64-apple-darwin24.0.0
Copyright (C) 1988-2023 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

で, gmake を make にするために

🚀 echo 'alias make=gmake' >> ~/.zshrc
🚀 source ~/.zshrc

とした. そうすると

🚀 make --version
GNU Make 4.4.1
Built for aarch64-apple-darwin24.0.0
Copyright (C) 1988-2023 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

と無事最新版に更新できた.