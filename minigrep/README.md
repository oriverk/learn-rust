# mingrep

## example

`CASE_INSENSITIVE` = 1 OR 0

```shell
# cargo run [word] [target_file]
cargo run the poem.txt
CASE_INSENSITIVE=1 cargo run to poem.txt
CASE_INSENSITIVE=0 cargo run to poem.txt > output.txt
```

## reference

- The Rust Programming Language 日本語版
  - [12章 入出力プロジェクト：コマンドラインプログラムを構築する](https://doc.rust-jp.rs/book-ja/ch12-00-an-io-project.html)
