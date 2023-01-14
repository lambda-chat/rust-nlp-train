# NLP in Rust

お砂場。Sudachi.rs を CLI としてではなく Rust コードから動作させたかったもの。

## Usage

fetch_dictionary.sh をコピペしてリソースだけ取ってくれば submodule じゃなくてもいいんだけど一旦これで……。

```sh
git submodule update -i
cd sudachi && ./sudachi/fetch_dictionary.sh && cd ..
cd rust-nlp
cargo run 2> /dev/null
```

実行結果

```sh
❯ cargo run 2> /dev/null
外国    ["名詞", "普通名詞", "一般", "*", "*", "*"]
人      ["接尾辞", "名詞的", "一般", "*", "*", "*"]
参政    ["名詞", "普通名詞", "一般", "*", "*", "*"]
権      ["接尾辞", "名詞的", "一般", "*", "*", "*"]
```

具体的なコードは rust-nlp/src/main.rs を参照してください。
