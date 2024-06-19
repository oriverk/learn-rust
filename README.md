# Learn Rust

```rust title=main.rs
fn main() {
    println!("Hello, world!");
}
```

## reference

- [The Rust Programming Language - The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Introduction - Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Introduction - The Cargo Book](https://doc.rust-lang.org/cargo/index.html)
- [Tour of Rust - Let's go on an adventure!](https://tourofrust.com/)
- [はじめに - AtCoderコンテストにRustで参加するためのガイドブック](https://doc.rust-jp.rs/atcoder-rust-resources/)

## post

- [Rust をやってみる | oriverk.dev](https://oriverk.dev/blog/20240609-learn-rust/)
- [Zenn.dev | Rustで作物列画像を二値化処理をする](https://zenn.dev/oriverk/articles/432f7c2f17b928)

## workspace

- [crop image processing with image crate](/crop_image/)

## how to manage workspace

### add workspace

```sh
cargo new --bin my_project
```

```toml title=Cargo.toml
[workspace]
members = [
  "my_project"
]
```

### run workspace

```sh
cargo run -p my_project
```
