# csvmdへの貢献

csvmdへの貢献に興味を持っていただき、ありがとうございます！🎉 この文書は、開始に役立つガイドラインと情報を提供します。

## 目次

- [はじめに](#はじめに)
- [開発環境](#開発環境)
- [プロジェクトアーキテクチャ](#プロジェクトアーキテクチャ)
- [開発ワークフロー](#開発ワークフロー)
- [コードスタイルと品質](#コードスタイルと品質)
- [テスト](#テスト)
- [変更の提出](#変更の提出)
- [リリースプロセス](#リリースプロセス)
- [ヘルプの取得](#ヘルプの取得)

## はじめに

csvmdは、CSVファイルをMarkdownテーブルに変換するRust CLIツールです。速度と効率性を重視して設計されており、大きなファイルを処理するための標準モードとストリーミングモードの両方をサポートしています。

### 前提条件

- [Rust](https://www.rust-lang.org/tools/install)（最新の安定版を推奨）
- Git
- Rustサポート付きのテキストエディタまたはIDE

### クイックセットアップ

1. **リポジトリをフォークしてクローン:**
   ```bash
   git clone https://github.com/あなたのユーザー名/csvmd.git
   cd csvmd
   ```

2. **プロジェクトをビルド:**
   ```bash
   cargo build
   ```

3. **すべてが動作することを確認するためにテストを実行:**
   ```bash
   cargo test
   ```

4. **CLIツールを試してみる:**
   ```bash
   # サンプルCSVを作成
   echo "名前,年齢\n太郎,25\n花子,30" > sample.csv
   
   # Markdownに変換
   cargo run -- sample.csv
   ```

## 開発環境

### 推奨ツール

- **Rust Analyzer**: IDEサポート用
- **cargo-watch**: 継続的ビルド/テスト用
  ```bash
  cargo install cargo-watch
  cargo watch -x test
  ```

### 必須コマンド

```bash
# プロジェクトをビルド
cargo build

# 最適化されたリリース版をビルド
cargo build --release

# CLIツールを実行
cargo run -- [オプション] [ファイル]

# コードをフォーマット（コミット前に必ず実行）
cargo fmt

# フォーマットをチェック（コードが正しくフォーマットされていることを確認）
cargo fmt --check

# コードをリント
cargo clippy

# すべてのテストを実行（ユニット + 統合）
cargo test

# ユニットテストのみ実行（src/lib.rs内）
cargo test --lib

# 統合テストのみ実行
cargo test --test integration_tests

# 特定のテストを実行
cargo test test_csv_with_pipes

# 出力付きで実行
cargo test -- --nocapture
```

**重要**: 一貫したコードフォーマットを確保するため、コミット前に必ず`cargo fmt`を実行してください。

## プロジェクトアーキテクチャ

csvmdは標準的なRustライブラリ + バイナリパターンに従っています：

```
src/
├── lib.rs          # 変換ロジック付きコアライブラリ
├── main.rs         # clapを使用したCLIインターフェース
└── error.rs        # thiserrorを使用したカスタムエラータイプ

tests/
├── integration_tests.rs  # 完全なCLI機能テスト
└── edge_cases.rs         # エッジケースとエラー条件テスト
```

### 主要コンポーネント

- **コアライブラリ** (`src/lib.rs`): 2つの主要な関数が含まれます：
  - `csv_to_markdown()`: CSV全体をメモリに読み込む、小さなファイルに適している
  - `csv_to_markdown_streaming()`: 大きなファイル用の2パスストリーミングアプローチ
  
- **CLIインターフェース** (`src/main.rs`): 引数解析にclapを使用し、入出力を処理

- **エラーハンドリング** (`src/error.rs`): CSV解析、IO、フォーマットエラー用のカスタムエラータイプ

### 主要な設計決定

- 不均等な列数を処理するための柔軟な解析を持つcsvクレートを使用
- Markdown特殊文字をエスケープ: `|` → `\|`, `\n` → `<br>`
- 推定出力サイズに基づいて文字列容量を事前割り当て
- ストリーミングモードは正しいテーブルフォーマットを確保するために2パスアプローチを使用

## 開発ワークフロー

### 変更を行う

1. **機能ブランチを作成:**
   ```bash
   git checkout -b feature/あなたの機能名
   ```

2. **コーディング標準に従って変更を行う**

3. **変更をテスト:**
   ```bash
   # フォーマットを実行
   cargo fmt
   
   # リントを実行
   cargo clippy
   
   # すべてのテストを実行
   cargo test
   ```

4. **CLIを手動でテスト:**
   ```bash
   # 基本機能をテスト
   echo "名前,年齢\n太郎,25" | cargo run
   
   # ファイルでテスト
   cargo run -- test_data.csv
   
   # ストリーミングモードをテスト
   cargo run -- --stream large_file.csv
   
   # 異なる整列をテスト
   cargo run -- --align center data.csv
   ```

### コード変更ガイドライン

- 最小限の変更を行う - 可能な限り少ない行数を変更
- 絶対に必要でない限り、動作するコードを削除/除去しない
- 変更が既存の動作を壊さないことを常に検証
- 変更に直接関連する場合はドキュメントを更新

## コードスタイルと品質

### フォーマット

- **コミット前に必ず`cargo fmt`を実行**
- デフォルトのrustfmt設定を使用
- CIで`cargo fmt --check`を通過する必要があります

### リント

- コードは警告なしで`cargo clippy`を通過する必要があります
- Rustの命名規則とイディオムに従う
- 意味のある変数名と関数名を使用

### ドキュメント

- パブリック関数とタイプにdocstringを追加
- 役立つ場合はドキュメントに例を含める
- 新機能の追加やCLIインターフェースの変更時はREADME.mdを更新

## テスト

csvmdには包括的なテスト戦略があります：

### ユニットテスト

`src/lib.rs`に配置され、コア変換ロジックをテストします：

```bash
cargo test --lib
```

カバレッジには以下が含まれます：
- 基本的なCSV変換
- エッジケース（空のセル、特殊文字、Unicode）
- ヘッダー整列オプション
- カスタム区切り文字
- エラー条件

### 統合テスト

`tests/integration_tests.rs`に配置され、完全なCLI機能をテストします：

```bash
cargo test --test integration_tests
```

カバレッジには以下が含まれます：
- コマンドライン引数解析
- ファイル入出力
- stdin/stdout処理
- エラーハンドリングとレポート
- クロスプラットフォーム互換性

### エッジケーステスト

`tests/edge_cases.rs`に配置され、異常な入力をテストします：

```bash
cargo test --test edge_cases
```

### 新しいテストの追加

機能を追加する際：

1. **ユニットテストを追加** `src/lib.rs`のコアロジック用
2. **統合テストを追加** `tests/integration_tests.rs`のCLI動作用
3. **エッジケースを考慮**し、`tests/edge_cases.rs`にテストを追加

ユニットテストの例：
```rust
#[test]
fn test_your_feature() {
    let csv_data = "名前,年齢\n太郎,25";
    let input = Cursor::new(csv_data);
    let config = Config::default();
    let result = csv_to_markdown(input, config).unwrap();
    
    let expected = "| 名前 | 年齢 |\n| --- | --- |\n| 太郎 | 25 |\n";
    assert_eq!(result, expected);
}
```

## 変更の提出

### プルリクエストガイドライン

1. **すべてのテストが通ることを確認:**
   ```bash
   cargo test
   cargo fmt --check
   cargo clippy
   ```

2. **明確なPR説明を書く:**
   - 何を変更し、なぜ変更したかを説明
   - 関連するissueを参照
   - 新機能を追加する場合は例を含める

3. **PRを焦点を絞って保つ:**
   - PR当たり1つの機能または修正
   - 関連のない変更を混在させない

4. **ドキュメントを更新** 変更が以下に影響する場合：
   - CLIインターフェース
   - パブリックAPI
   - インストールまたは使用方法の指示

### コミットメッセージ

- 明確で説明的なコミットメッセージを使用
- 現在時制の動詞で開始（"Add"、"Fix"、"Update"）
- 該当する場合はissueを参照（"Fixes #123"）

### CI要件

あなたのPRはすべてのCIチェックを通過する必要があります：

- ✅ すべてのプラットフォーム（Linux、macOS、Windows）でテストが通る
- ✅ コードが正しくフォーマットされている（`cargo fmt --check`）
- ✅ リント警告がない（`cargo clippy`）
- ✅ リリースモードで正常にビルドされる

## リリースプロセス

csvmdは自動化されたリリースプロセスを使用します：

1. **バージョンタグ付け**: `vX.Y.Z`形式のタグをプッシュすることでリリースがトリガーされます
2. **クロスプラットフォームビルド**: CIが自動的に複数のプラットフォーム用にビルドします
3. **コード署名**: macOSバイナリは署名され、公証されます
4. **公開**: リリースはGitHubリリースとcrates.ioの両方に公開されます

貢献者はリリースについて心配する必要はありません - メンテナーがこのプロセスを処理します。

## ヘルプの取得

### ドキュメント

- [README.md](README.md) - 使用方法と例
- [The Rust Book](https://doc.rust-lang.org/book/) - Rustを学ぶ
- [Clapドキュメント](https://docs.rs/clap/) - CLI引数解析
- [CSVクレートドキュメント](https://docs.rs/csv/) - CSV解析

### コミュニケーション

- **Issues**: バグレポートと機能リクエストにはGitHub issueを使用
- **Discussions**: 質問と一般的な議論にはGitHub discussionsを使用
- **セキュリティ**: セキュリティissueについては、責任ある開示に従ってください

### よくある問題

**ビルドの問題:**
```bash
# クリーンして再ビルド
cargo clean
cargo build
```

**テストの失敗:**
```bash
# 出力付きで特定のテストを実行
cargo test test_name -- --nocapture

# テストを一つずつ実行
cargo test -- --test-threads=1
```

**フォーマットの問題:**
```bash
# フォーマットを自動修正
cargo fmt

# 何が変更されるかをチェック
cargo fmt -- --check
```

---

csvmdへの貢献をありがとうございます！あなたの貢献により、すべての人にとってCSV-to-Markdown変換がより良いものになります。🚀