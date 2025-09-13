# Maze Modes UI – Design

作成日: 2025-09-12

## 目的
- 迷路生成モード（ランダム / 一筆書き）を切り替えてプレビューできるUIを提供する。
- 入力（行/列/セル幅）を分かりやすく、制約は明示してガードする。
- 自動プレビューで素早く試行できる。

## 画面構成
- 設定カード（Paper）
  - タイトル + モード切替（Tabs: Random | Single-Stroke）
  - 入力: Rows, Cols, Cell Size(px)
  - 補助: Cell Sizeスライダー、プリセットChips、自動プレビューSwitch
  - アクション: Generate, Reset
- プレビューカード（Paper）
  - 現在のキャンバスサイズ表示（px）
  - `<canvas id="canvas">`（WASM側がこれを参照）

## バリデーション
- 共通: rows/cols >= 1 の整数、cellSize 1–200
- 一筆書き: rows と cols が同時に奇数だと不可（どちらかが偶数）
- 小さすぎるケース: rows <= 2 かつ cols <= 2 は非推奨（警告）。
- 無効時はGenerateをdisabled。Auto Previewは動かさない。

## 振る舞い
- モード変更: モード特有の検証を行い、Auto Preview有効かつ有効入力なら即描画。
- 入力変更: Auto PreviewがONで有効なら300msデバウンスで描画。
- Generate: 有効時にWASMを呼び出し。
- Reset: 既定値（Rows=15, Cols=15, Cell=20, Mode=Random）に戻し、Auto Previewなら描画。

## WASM連携
- Random: `draw_maze(0, 0, rows, cols, cellSize)`
- Single-Stroke: `draw_single_stroke_maze(0, 0, rows, cols, cellSize)`
- 描画前に`canvas.width/height`を `cols*cellSize`, `rows*cellSize` に設定して表示領域を調整。

## 状態
- `mode: 'random' | 'single'`
- `params: { rows: number; cols: number; cellSize: number }`
- `autoPreview: boolean`
- `validation: { errors: { rows?, cols?, cellSize?, mode? }, valid: boolean }`
- 派生: `canvasWidth`, `canvasHeight`

## アクセシビリティ/UX
- すべての入力にラベルとヘルパーテキスト。
- モード固有の制約はエラーメッセージで明示。
- ボタンはテキスト + アイコン、キーボード操作を阻害しない。

## 実装方針
- 既存のMUIベースUIにTabsとWASM呼出の分岐を追加。
- `ensureCanvasSize`でキャンバスの実ピクセルサイズを都度調整。
- Auto Previewは`useEffect` + 300msデバウンス。

