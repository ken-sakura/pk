# pk

`pk` は、ファイルの内容を標準出力に表示（cat）し、同時にクリップボードへコピーするCLIツールです。MarkdownファイルをHTMLへ変換する機能も備えています。

## 🚀 特徴

* **クイックcat & コピー**: ファイルを表示すると同時に、クリップボードにその内容を保持します。
* **HTML変換機能**: `-hp` オプションにより、Markdownファイルを即座にHTML形式へ変換して出力・コピーします。
* **シンプル設計**: 余計な依存を削ぎ落とした、高速で動作するCLIツールです。

## 📦 インストール方法

1. **Rustツールチェーンの準備**
[Rust公式サイト](https://www.rust-lang.org/)に従い、`cargo` が使用できる環境を構築してください。

2. **ビルド・インストール**
```bash
cargo install --path .
```

## 📖 使い方

### 基本操作 (cat & copy)
指定したファイルの内容を表示し、クリップボードにコピーします。
```bash
peek <ファイル名>
```

### HTML変換 (Markdown to HTML)
`-hp` オプションを使用すると、MarkdownファイルをHTML形式に変換して表示し、クリップボードにコピーします。
※ `.md` 拡張子のファイルのみ対応しています。
```bash
peek -hp <ファイル名.md>
```

## 🛠 使用ライブラリ

* [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark): 高速Markdownパース
* [arboard](https://github.com/1Password/arboard): クリップボード操作
