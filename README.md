# sudachi-wasm

> WebAssembly distribution of sudachi.rs.

This distribution supports both of browser and Node.js.

## Requirements

- [wasm-pack](https://github.com/rustwasm/wasm-pack)
- [zx](https://github.com/google/zx)

## Build

```bash
wasm-pack build --dev --target web && cd pkg && zx ../wasm-pack-inline.mjs && cd ..
```

## Test

### Browser

```bash
npx http-server
```

Then, access to the [local server](http://127.0.0.1:8080/test/browser.html).

### Node.js

```bash
node test/node.mjs
```

## Disclaimer

The following creations are included in this product:

- [WorksApplications/sudachi.rs](https://github.com/WorksApplications/sudachi.rs/blob/develop/LICENSE)
- [WorksApplications/SudachiDict](https://github.com/WorksApplications/SudachiDict#licenses)
  - [UniDic](https://unidic.ninjal.ac.jp/)
  - [neologd/mecab-ipadic-neologd](https://github.com/neologd/mecab-ipadic-neologd)

```text
Copyright (c) 2011-2013, The UniDic Consortium
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are
met:

 * Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.

 * Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the
   distribution.

 * Neither the name of the UniDic Consortium nor the names of its
   contributors may be used to endorse or promote products derived
   from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
"AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.


- src/main/text/core_lex.csv and src/main/text/notcore_lex.csv contain
a part of NEologd (https://github.com/neologd/mecab-unidic-neologd).

Copyright (C) 2015-2019 Toshinori Sato (@overlast)

      https://github.com/neologd/mecab-unidic-neologd

    i. 本データは、株式会社はてなが提供するはてなキーワード一覧ファイル
       中の表記、及び、読み仮名の大半を使用している。

       はてなキーワード一覧ファイルの著作権は、株式会社はてなにある。

       はてなキーワード一覧ファイルの使用条件に基づき、また、
       データ使用の許可を頂いたことに対する感謝の意を込めて、
       以下に株式会社はてなおよびはてなキーワードへの参照をURLで示す。

       株式会社はてな : http://hatenacorp.jp/information/outline

       はてなキーワード :
       http://developer.hatena.ne.jp/ja/documents/keyword/misc/catalog

   ii. 本データは、日本郵便株式会社が提供する郵便番号データ中の表記、
       及び、読み仮名を使用している。

       日本郵便株式会社は、郵便番号データに限っては著作権を主張しないと
       述べている。

       日本郵便株式会社の郵便番号データに対する感謝の意を込めて、
       以下に日本郵便株式会社および郵便番号データへの参照をURLで示す。

       日本郵便株式会社 :
         http://www.post.japanpost.jp/about/profile.html

       郵便番号データ :
         http://www.post.japanpost.jp/zipcode/dl/readme.html

  iii. 本データは、スナフキん氏が提供する日本全国駅名一覧中の表記、及び
       読み仮名を使用している。

       日本全国駅名一覧の著作権は、スナフキん氏にある。

       スナフキん氏は 「このデータを利用されるのは自由ですが、その際に
       不利益を被ったりした場合でも、スナフキんは一切責任は負えません
       ことをご承知おき下さい」と述べている。

       スナフキん氏に対する感謝の意を込めて、
       以下に日本全国駅名一覧のコーナーへの参照をURLで示す。

       日本全国駅名一覧のコーナー :
         http://www5a.biglobe.ne.jp/~harako/data/station.htm

   iv. 本データは、工藤拓氏が提供する人名(姓/名)エントリデータ中の、
       漢字表記の姓・名とそれに対応する読み仮名を使用している。

       人名(姓/名)エントリデータは被災者・安否不明者の人名の
       表記揺れ対策として、Mozcの人名辞書を活用できるという
       工藤氏の考えによって提供されている。

       工藤氏に対する感謝の意を込めて、
       以下にデータ本体と経緯が分かる情報への参照をURLで示す。

       人名(姓/名)エントリデータ :
         http://chasen.org/~taku/software/misc/personal_name.zip

       上記データが提供されることになった経緯
         http://togetter.com/li/111529

    v. 本データは、Web上からクロールした大量の文書データから抽出した
       表記とそれに対応する読み仮名のデータを含んでいる。

       抽出した表記とそれに対応する読み仮名の組は、上記の i. から iv.
       の言語資源の組み合わせによって得られる組のみを採録した。

       Web 上に文書データを公開して下さっている皆様に感謝いたします。

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

```

***

**2021-07-07 UPDATE: The official Sudachi team will take over this project** (cf. [日本語形態素解析器 SudachiPy の 現状と今後について - Speaker Deck](https://speakerdeck.com/waptech/ri-ben-yu-xing-tai-su-jie-xi-qi-sudachipy-false-xian-zhuang-tojin-hou-nituite?slide=28))


# sudachi.rs

<p align="center"><img width="100" src="logo.png" alt="sudachi.rs logo"></p>

An official [Sudachi](https://github.com/WorksApplications/Sudachi) clone in Rust 🦀

[日本語 README](#sudachirs---日本語readme)


## Caution

This is my hobby project to try out Rust, and the implementation is incomplete; One fatal problem is that it will throw an error when there is an Out-of-Vocabulary word (i.e., when there is no lattice path from the beginning to the end).

```sh
$ echo "あ" | sudachi
あ      感動詞,フィラー,*,*,*,* あー
EOS

$ echo "阿" | sudachi
thread 'main' panicked at 'EOS isn't connected to BOS', src/lattice.rs:70:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Please also have a look at an alternative by another person, [Yasu-umi/sudachiclone-rs](https://github.com/Yasu-umi/sudachiclone-rs).


## Example

Multi-granular Tokenization

```
$ echo 選挙管理委員会 | sudachi
選挙管理委員会	名詞,固有名詞,一般,*,*,*	選挙管理委員会
EOS

$ echo 選挙管理委員会 | sudachi --mode A
選挙	名詞,普通名詞,サ変可能,*,*,*	選挙
管理	名詞,普通名詞,サ変可能,*,*,*	管理
委員	名詞,普通名詞,一般,*,*,*	委員
会	名詞,普通名詞,一般,*,*,*	会
EOS
```

Normalized Form

```
$ echo 打込む かつ丼 附属 vintage | sudachi
打込む	動詞,一般,*,*,五段-マ行,終止形-一般	打ち込む
 	空白,*,*,*,*,*
かつ丼	名詞,普通名詞,一般,*,*,*	カツ丼
 	空白,*,*,*,*,*
附属	名詞,普通名詞,サ変可能,*,*,*	付属
 	空白,*,*,*,*,*
vintage	名詞,普通名詞,一般,*,*,*	ビンテージ
```

Wakati (space-delimited surface form) Output

```
$ cat lemon.txt
えたいの知れない不吉な塊が私の心を始終圧えつけていた。
焦躁と言おうか、嫌悪と言おうか――酒を飲んだあとに宿酔があるように、酒を毎日飲んでいると宿酔に相当した時期がやって来る。
それが来たのだ。これはちょっといけなかった。

$ sudachi --wakati lemon.txt
えたい の 知れ ない 不吉 な 塊 が 私 の 心 を 始終 圧え つけ て い た 。
焦躁 と 言おう か 、 嫌悪 と 言おう か ― ― 酒 を 飲ん だ あと に 宿酔 が ある よう に 、 酒 を 毎日 飲ん で いる と 宿酔 に 相当 し た 時期 が やっ て 来る 。
それ が 来 た の だ 。 これ は ちょっと いけ なかっ た 。
```

## Usage

```
$ sudachi -h
sudachi 0.1.0
A Japanese tokenizer

USAGE:
    sudachi [FLAGS] [OPTIONS] [file]

FLAGS:
    -d, --debug      Debug mode: Dumps lattice
    -h, --help       Prints help information
    -a, --all        Prints all fields
    -V, --version    Prints version information
    -w, --wakati     Outputs only surface form

OPTIONS:
    -m, --mode <mode>    Split unit: "A" (short), "B" (middle), or "C" (Named Entity) [default: C]

ARGS:
    <file>    Input text file: If not present, read from STDIN
```

## Setup

### 1. Get the source code

```
$ git clone https://github.com/sorami/sudachi.rs.git
```

### 2. Download a Sudachi Dictionary

You can download a dictionary zip file from [WorksApplications/SudachiDict](https://github.com/WorksApplications/SudachiDict) (choose one from `small`, `core`, or `full`), unzip it, and place the `system_*.dic` file to `src/resources/system.dic` (Note that the file name is changed to `system.dic`) .

Alternatively, you can use a quick shell script in the source code; This script will download the `core` dictionary and place it to `src/resources/system.dic`.

```
$ ./fetch_dictionary.sh
```

### 3. Build, Install

The built executable will **contain the dictionary binary**.

```
$ cargo build
```

or

```
sudachi.rs/ $ cargo install --path .

$ which sudachi
/Users/<USER>/.cargo/bin/sudachi

$ sudachi -h
sudachi 0.1.0
A Japanese tokenizer
...
```


## ToDo

- [ ] Out of Vocabulary handling
- [ ] Easy dictionary file install & management, [similar to SudachiPy](https://github.com/WorksApplications/SudachiPy/issues/73)
- [ ] Registration to crates.io


## References

### Sudachi

- [WorksApplications/Sudachi](https://github.com/WorksApplications/Sudachi)
- [WorksApplications/SudachiDict](https://github.com/WorksApplications/SudachiDict)
- [WorksApplications/SudachiPy](https://github.com/WorksApplications/SudachiPy)
- [msnoigrs/gosudachi](https://github.com/msnoigrs/gosudachi)


### Morphological Analyzers in Rust

- [agatan/yoin: A Japanese Morphological Analyzer written in pure Rust](https://github.com/agatan/yoin)
- [wareya/notmecab-rs: notmecab-rs is a very basic mecab clone, designed only to do parsing, not training.](https://github.com/wareya/notmecab-rs)

### Logo

- [Sudachi Logo](https://github.com/WorksApplications/Sudachi/blob/develop/docs/Sudachi.png)
- Crab illustration: [Pixabay](https://pixabay.com/ja/vectors/%E5%8B%95%E7%89%A9-%E3%82%AB%E3%83%8B-%E7%94%B2%E6%AE%BB%E9%A1%9E-%E6%B5%B7-2029728/)


***


# sudachi.rs - 日本語README

<p align="center"><img width="100" src="logo.png" alt="sudachi.rs logo"></p>

形態素解析器 [Sudachi](https://github.com/WorksApplications/Sudachi)  - 公式 Rust 🦀 クローン

[English README](#sudachirs)

## 注意

これはRust勉強のための趣味実装で、実装が未完の部分があります。特に、未知語が存在するときにエラーが発生します（ラティスで最初から最後までパスが存在しない場合）。

```sh
$ echo "あ" | sudachi
あ      感動詞,フィラー,*,*,*,* あー
EOS

$ echo "阿" | sudachi
thread 'main' panicked at 'EOS isn't connected to BOS', src/lattice.rs:70:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

他の方によるRust実装も参照ください; [Yasu-umi/sudachiclone-rs](https://github.com/Yasu-umi/sudachiclone-rs)


## 利用例

複数粒度での分割

```
$ echo 選挙管理委員会 | sudachi
選挙管理委員会	名詞,固有名詞,一般,*,*,*	選挙管理委員会
EOS

$ echo 選挙管理委員会 | sudachi --mode A
選挙	名詞,普通名詞,サ変可能,*,*,*	選挙
管理	名詞,普通名詞,サ変可能,*,*,*	管理
委員	名詞,普通名詞,一般,*,*,*	委員
会	名詞,普通名詞,一般,*,*,*	会
EOS
```

正規化表記

```
$ echo 打込む かつ丼 附属 vintage | sudachi
打込む	動詞,一般,*,*,五段-マ行,終止形-一般	打ち込む
 	空白,*,*,*,*,*
かつ丼	名詞,普通名詞,一般,*,*,*	カツ丼
 	空白,*,*,*,*,*
附属	名詞,普通名詞,サ変可能,*,*,*	付属
 	空白,*,*,*,*,*
vintage	名詞,普通名詞,一般,*,*,*	ビンテージ
```

分かち書き出力

```
$ cat lemon.txt
えたいの知れない不吉な塊が私の心を始終圧えつけていた。
焦躁と言おうか、嫌悪と言おうか――酒を飲んだあとに宿酔があるように、酒を毎日飲んでいると宿酔に相当した時期がやって来る。
それが来たのだ。これはちょっといけなかった。

$ sudachi --wakati lemon.txt
えたい の 知れ ない 不吉 な 塊 が 私 の 心 を 始終 圧え つけ て い た 。
焦躁 と 言おう か 、 嫌悪 と 言おう か ― ― 酒 を 飲ん だ あと に 宿酔 が ある よう に 、 酒 を 毎日 飲ん で いる と 宿酔 に 相当 し た 時期 が やっ て 来る 。
それ が 来 た の だ 。 これ は ちょっと いけ なかっ た 。
```

## 利用方法

```
$ sudachi -h
sudachi 0.1.0
A Japanese tokenizer

USAGE:
    sudachi [FLAGS] [OPTIONS] [file]

FLAGS:
    -d, --debug      Debug mode: Dumps lattice
    -h, --help       Prints help information
    -a, --all        Prints all fields
    -V, --version    Prints version information
    -w, --wakati     Outputs only surface form

OPTIONS:
    -m, --mode <mode>    Split unit: "A" (short), "B" (middle), or "C" (Named Entity) [default: C]

ARGS:
    <file>    Input text file: If not present, read from STDIN
```

## セットアップ

### 1. ソースコードの取得

```
$ git clone https://github.com/sorami/sudachi.rs.git
```

### 2. Sudachi辞書のダウンロード 

[WorksApplications/SudachiDict](https://github.com/WorksApplications/SudachiDict)から辞書のzipファイル（ `small` 、 `core` 、 `full` から一つ選択）し、解凍して、中にある `system_*.dic` ファイルを `src/resources/system.dic` として置いてください （ファイル名が `system.dic` に変わっていることに注意）。

上記のように手動で設置する以外に、レポジトリにあるスクリプトを使って自動的に `core` 辞書をダウンロードし `src/resources/system.dic` として設置することもできます。

```
$ ./fetch_dictionary.sh
```

### 3. ビルド、インストール

ビルドされた実行ファイルは、**辞書バイナリを内包しています**。

```
$ cargo build
```

もしくは

```
sudachi.rs/ $ cargo install --path .

$ which sudachi
/Users/<USER>/.cargo/bin/sudachi

$ sudachi -h
sudachi 0.1.0
A Japanese tokenizer
...
```


## ToDo

- [ ] 未知語処理
- [ ] 簡単な辞書ファイルのインストール、管理（[SudachiPyでの方式を参考に](https://github.com/WorksApplications/SudachiPy/issues/73)）
- [ ] crates.io への登録


## リファレンス

### Sudachi

- [WorksApplications/Sudachi](https://github.com/WorksApplications/Sudachi)
- [WorksApplications/SudachiDict](https://github.com/WorksApplications/SudachiDict)
- [WorksApplications/SudachiPy](https://github.com/WorksApplications/SudachiPy)
- [msnoigrs/gosudachi](https://github.com/msnoigrs/gosudachi)

### Rustによる形態素解析器の実装

- [agatan/yoin: A Japanese Morphological Analyzer written in pure Rust](https://github.com/agatan/yoin)
- [wareya/notmecab-rs: notmecab-rs is a very basic mecab clone, designed only to do parsing, not training.](https://github.com/wareya/notmecab-rs)

### ロゴ

- [Sudachiのロゴ](https://github.com/WorksApplications/Sudachi/blob/develop/docs/Sudachi.png)
- カニのイラスト: [Pixabay](https://pixabay.com/ja/vectors/%E5%8B%95%E7%89%A9-%E3%82%AB%E3%83%8B-%E7%94%B2%E6%AE%BB%E9%A1%9E-%E6%B5%B7-2029728/)
