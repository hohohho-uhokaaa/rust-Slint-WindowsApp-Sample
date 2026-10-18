# zed-test-01（Slint Hello App）

[Slint](https://slint.dev/) 1.18.1 を使用した Windows 11 対応のシンプルな GUI アプリです。  
ボタンをクリックすると、テキストコントロールに **Hello, Slint!** が表示されます。

SlintでWindowsのデスクトップアプリをLLMで推論させて作ろうとしてドはまりしたのでサンプルにしてみます。  
使用したLLMはローカルでgpt-oss-20b、qwen3.8-27bの2つでどちらもSlintの学習はバージョンが古く、  
1.18.1のような最新版で作ってと指示するとドはまりします。  
geminiやcopilotにドはまりから脱出を指示するプロンプトを聞きながら推論させ、最終的に推論ルールで  
最初にどう指示しておけばドはまりしないかを、このREADME.mdの末尾「考察」部分に推論ルールとして  
ひとまずまとめることができました。尚、まとめはClineのfreeトライアルで使えるnemotron-3.5-super-viに  
仕上げてもらっています。

## 動作環境

- Windows 11
- Rust（stable、2024 エディション対応）

## 実行方法

```sh
cargo run
```

ウィンドウが開いたら **Click me** ボタンをクリックすると、上部のテキストに **Hello, Slint!** が表示されます。

## リリースビルド

```sh
cargo build --release
```

生成された実行ファイルは `target/release/zed-test-01.exe` です。

## プロジェクト構成

```
zed-test-01/
├── Cargo.toml      # パッケージ定義（slint 1.18.1 の依存を宣言）
├── Cargo.lock      # 依存関係のバージョンロック（自動生成）
├── .gitignore      # Git 管理除外設定（target/ を除外）
├── README.md       # このファイル
└── src/
    └── main.rs     # アプリ本体（UI 定義 + 起動処理）
```

## 実装のポイント

### 1. UI 定義（`.slint` 記述）

`slint::slint!` マクロにより UI 定義が `src/main.rs` にインラインで記述されています。

- **`MyApp`**：`Window` を継承したメインウィンドウコンポーネント
- **`in-out property <string> message`**：表示テキストを保持するプロパティ
  - コンポーネント内部から書き換えるため `in-out` 宣言が必須
  - 型は `string`（`text` という型名は存在しない）
- **`Button` の `clicked` コールバック**：`root.message = "Hello, Slint!"` を実行

### 2. 起動処理（Rust 側）

```rust
let ui = MyApp::new();
ui.run().expect("Failed to run the main window");
```

- `MyApp::new()` はインスタンス（`Self`）を直接返す（`Result` ではない）
- `ui.run()` がウィンドウ表示とイベントループの両方を行い、エラーは `Result` として返されるため `.expect()` で解包する（Slint 1.x の標準パターン）

### 3. Windows 向け設定

- ファイル先頭の `#![windows_subsystem = "windows"]` により、GUI アプリ起動時にコンソールウィンドウが表示されないようになっている

## 備考（Rust と UI の通信パターン）

今回はクリック処理がすべて `.slint` 側にあるため、Rust との通信は不要です。  
将来、Rust 側から UI を更新したりイベントを受け取りたい場合は、ルートコンポーネントに `in-out property` と `callback` を定義して、Rust 側では `as_weak()` / `upgrade()` を使った安全なクロージャ設計でハンドラを登録してください（要素 ID への直接アクセスはできません）。

## 考察：LLM による Slint コーディングにおける注意点とプロンプト例

### 1. Slint 最新版（1.x系）で LLM に推論させる際の注意事項

Slint は 0.x系 から 1.x系 へ大幅な仕様変更が行われています。LLM（特に Slint 0.x系 の学習データのみを持つモデル）にコード生成を依頼する場合、以下の点に注意してください。

- **`in-out property` の必須宣言**：1.x系 ではコンポーネント内部から書き換えるプロパティは必ず `in-out property` で宣言する必要があります。`property` や `in property` だけではコンパイルエラーになります。
- **型名は `string`**：Slint では `text` という型名は存在せず、文字列型は常に `string` です。
- **`slint::slint!` マクロ内の UI 定義**：UI は `.slint` ファイルではなく `slint::slint!` マクロ内にインラインで記述するのが 1.x系 の標準パターンです。
- **`MyApp::new()` はインスタンス（`Self`）を直接返す**：`Result` ではないため `.expect()` は不要。エラー処理は `ui.run()` の戻り値で行う。
- **`ui.run()` でイベントループを開始**：`show()` とイベントループを分けて呼ぶ旧バージョンのパターンとは異なり、1.x系 では `run()` が両方を担います。
- **要素 ID への直接アクセス不可**：Rust 側から `ui.hello_button()` のように ID で要素に直接アクセスするメソッドは生成されません。プロパティやコールバック経由で間接的に操作します。

### 2. 過去バージョン（0.x系）と最新バージョン（1.x系）の主な相違点

| 項目 | 0.x系（旧） | 1.x系（最新） |
|------|-------------|---------------|
| プロパティ宣言 | `property`, `in property`, `out property` | `in-out property`（双方向） |
| 文字列型 | `string` | `string`（`text` は存在しない） |
| ウィンドウ生成 | `MyApp::new()`（インスタンスを直接返す） | `MyApp::new()`（インスタンスを直接返す、`Result` ではない） |
| イベントループ | `show()` + 別途イベントループ | `ui.run()` が表示とイベントループを両方担う |
| UI 定義場所 | `.slint` ファイル分離が主流 | `slint::slint!` マクロ内インライン定義が標準 |
| 要素へのアクセス | ID で直接アクセス可能なケースがあった | 要素 ID への直接アクセス不可（プロパティ/コールバック経由） |
| `Text` 要素 | `std-widgets.slint` からインポートが必要な場合あり | ビルトイン要素（インポート不要） |
| `Button` 要素 | `std-widgets.slint` からインポート | `std-widgets.slint` からインポート（または `slint!` マクロ内で自動利用） |

### 3. Slint 過去バージョンのみ学習済みの LLM で推論させる際の推論ルール

LLM が Slint 0.x系 の知識に基づいてコードを生成しようとする場合、以下のルールをプロンプトやシステム指示で強制してください。

1. **Slint 1.x系 の構文を使用する**：`in-out property <string> 名前` を必ず使用し、`property` 単体や `in property` / `out property` は使用しない。
2. **型名は `string` のみ使用する**：`text` 型は存在しないため絶対に使用しない。
3. **`slint::slint!` マクロ内で UI を定義する**：`.slint` ファイル分離ではなく、Rust ソース内にインラインで記述する。
4. **`MyApp::new()` はインスタンスを直接返す**：`Result` ではないため `.expect()` や `match` は不要。エラーは `ui.run()` で処理する。
5. **`ui.run()` を呼び出す**：`show()` の後に別途イベントループを回すコードは生成しない。
6. **要素 ID による直接アクセスを禁止する**：`ui.<id>()` のようなアクセサメソッドは存在しない。`in-out property` と `callback` を介して Rust 側と通信する。
7. **`Text` はビルトイン要素である**：`import { Text } from "std-widgets.slint"` のような記述は不要（エラーになる）。
8. **`#![windows_subsystem = "windows"]` をファイル先頭に配置する**：Windows GUI アプリではコンソールウィンドウを非表示にするため必須。

### 4. プロンプトの例

#### 例A：Slint 1.x系 での Hello World アプリ生成

```
以下の要件を満たす Rust + Slint（最新版 1.x系）の GUI アプリコードを生成してください。

- Slint バージョン: 1.x系（slint 1.18.1 以降を想定）
- OS: Windows 11（`#![windows_subsystem = "windows"]` を使用）
- ウィンドウに「Click me」ボタンと、メッセージ表示用のテキストを配置
- ボタンクリックでテキストに「Hello, Slint!」と表示
- UI は `slint::slint!` マクロ内でインライン定義
- プロパティは `in-out property <string> message` で宣言
- Rust 側では `MyApp::new()` と `ui.run()` を使用（`new()` は `Result` を返さない）
- 要素 ID への直接アクセスは行わない
```

#### 例B：Slint 0.x系 コードから 1.x系 への移行

```
以下の Slint 0.x系 コードを、Slint 1.x系（slint 1.18.1）に移行してください。

【移行ルール】
- `property` → `in-out property <string>` に変更
- `MyApp::new()` はインスタンスを直接返す（`Result` ではない）、`.expect()` は不要
- `show()` の代わりに `ui.run()` を使用
- 要素 ID への直接アクセスを削除し、`in-out property` と `callback` で代替
- `Text` 型は使用せず、`string` を使用

【元コード】
<ここに 0.x系 の .slint / Rust コードを貼り付け>
```

#### 例C：Rust から Slint UI を制御するコード生成

```
Rust 側から Slint（1.x系）の UI を制御するコードを生成してください。

- ルートコンポーネントに `in-out property <string> message` を定義
- ルートコンポーネントに `callback button_clicked` を定義
- Rust 側で `as_weak()` / `upgrade()` を使った弱参照クロージャを実装
- `invoke()` でコールバックをトリガー
- `set_message()` でプロパティを更新
- 要素 ID (`ui.hello_button()` など) への直接アクセスは禁止
```

