# eda3-ecs_wasm_game_soli_vanilla_codex_20250731

このリポジトリは、Rust と WebAssembly を用いた簡易的なソリティアゲームの実装例です。Rust 製の軽量 ECS（Entity Component System）を採用しており、ゲームロジックを部品化しやすくしています。JavaScript コードは必要最小限に抑え、Rust 側でほとんどの処理を行う構成です。

## セットアップ
1. [Rust](https://www.rust-lang.org/ja) のインストール。
2. `wasm32-unknown-unknown` ターゲットを追加します。
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
3. [wasm-bindgen-cli](https://github.com/rustwasm/wasm-bindgen) をインストールします。
   ```bash
   cargo install wasm-bindgen-cli
   ```

## ビルド方法
WebAssembly 用バイナリを生成するには以下を実行します。
```bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/eda3_ecs_wasm_game_soli_vanilla_codex_20250731.wasm --out-dir pkg --target web
```
生成された `pkg/` 以下のファイルをブラウザから読み込むことでゲームを動作させられます。

## テスト
基本的なユニットテストは `cargo test` で実行できます。

## 主要コンポーネント
- `src/ecs.rs`: 最小構成の ECS 実装。関数型スタイルでシンプルに書かれています。
- `src/game.rs`: トランプや山札に関するデータ構造を定義しています。
- `src/lib.rs`: WebAssembly から利用するラッパー。ゲームの初期化やメッセージ送信を提供します。
- `src/network.rs`: WebSocket を用いた通信をラップします。オプション機能のため、利用しなくてもプレイ可能です。

## 実行例
ブラウザ上で動作させる場合、HTML からロードする最小限の JavaScript が必要です。例:
```html
<script type="module">
import init, { SolitaireGame } from "./pkg/eda3_ecs_wasm_game_soli_vanilla_codex_20250731.js";

async function main() {
    await init();
    const game = new SolitaireGame();
    game.setup_board();
}
main();
</script>
```
上記のとおり、JavaScript の記述は読み込みと簡単な起動処理のみです。

## ライセンス
MIT ライセンスの下で公開されています。
