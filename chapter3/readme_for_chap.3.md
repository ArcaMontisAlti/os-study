# Chapter3: メモリ管理を実装しよう

## p.116 - p.138 main.rs のモジュール化

この部分で, 第2章時点で作成されている `src/main.rs` を

1. `src/lib.rs` (wasabiクレートをライブラリクレートとしてビルドする際に起点となるファイル)

2. `src/main.rs` (モジュール化後のmainファイル)

3. `src/graphics.rs` (画面描画関連のモジュール)

4. `src/uefi.rs` (UEFI関連のモジュール)

5. `src/result.rs` (Result型の定義部分)

を分けています. このまま `$ cargo run` すれば, 第2章時点での `src/main.rs` と変わらない挙動を示します.
