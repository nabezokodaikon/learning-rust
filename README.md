# Rust study
[Rust by Example](https://doc.rust-jp.rs/rust-by-example-ja/)

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

### 開発環境構築
```
# 実行してツールチェインをアップデート
$ rustup update nightly

# 標準ライブラリのメタデータをダウンロード
$ rustup component add --toolchain=nightly rust-analysis

# rls本体をインストール
$ rustup component add --toolchain=nightly rls-preview

# rust本体のソースをダウンロード
$ rustup component add --toolchain=nightly rust-src
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

### モジュールファイルシステムの規則
* `foo`という名前ののモジュールにサブモジュールがなければ、`foo`の定義は`foo.rs`というファイルに書く。
* `foo`というモジュールにサブモジュールがあったら、`foo`の定義は`foo/mod.rs`というファイルに書く。
```
├── foo
│   ├── bar.rs (contains the declarations in `foo::bar`)
│   │          (`foo::bar`内の定義を含む)
│   └── mod.rs (contains the declarations in `foo`, including `mod bar`)
               (`mod bar`を含む、`foo`内の定義を含む)
```

### macOS版のRustでpanic時のスタックトレースに行番号を出す
```
$ RUST_BACKTRACE=1 cargo run
```

### 読み方
`&T` -> `ref T`と読む。
`&mut T` -> `ref mute T`と読む。
