# Rust study

## インストール
### インストール
```
$ curl https://sh.rustup.rs -sSf | sh
# 1を選択する。
```

### アンインストール
```
$ rust self uninstall
```

### アップデート
```
$ rustup update
```


## プロジェクト作成
```
$ cargo new hello_cargo --bin
```


## ビルド

### ビルド-実行
```
$ cargo build
$ ./target/debug/hello_cargo
# or
$ cargo run
```

### チェック
迅速にコードを確認し、コードをコンパイルできることを確かめるが、
実行ファイルは生成しない。`cargo build`よりも速い。
```
$ cargo check
```

### リリースビルド
```
$ cargo build --release
$ ./target/release/hello_cargo
```

### ドキュメントの生成
以下を実行すると、現在依存しているクレートを含むドキュメントを生成し、
ウェブブラウザが起動する。
```
$ cargo doc --open 
```


## 開発環境
* [LanguageClient-neovim](https://github.com/autozimu/LanguageClient-neovim)
* [rls](https://github.com/rust-lang/rls)


## メモ
### 構造体でデバッグ用の情報を出力する
```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

let rect1 = Rectangle { width: 30, height: 50 };
println!("rect1 is {:?}", rect1);
```
