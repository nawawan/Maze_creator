# Maze UI Modernization – Design Doc

作成日: 2025-09-04

## 目的 / ゴール

- 迷路グリッド設定のUIをモダンかつ分かりやすくする。
- 入力（セル幅・横マス・縦マス）の操作性と視認性を改善。
- 自動プレビューやプリセットで試行錯誤を高速化。
- 今後のWASM連携（迷路生成）に自然に拡張できる構成にする。

## 非ゴール

- 迷路生成アルゴリズム（Kruskal等）そのものの実装・最適化は対象外。
- サーバー連携や永続化は対象外。

## 導入ライブラリ

- `@mui/material`, `@emotion/react`, `@emotion/styled`, `@mui/icons-material`

インストール（開発者ローカル実行）:

```bash
npm i @mui/material @emotion/react @emotion/styled @mui/icons-material
```

## 画面構成（情報設計）

- Container（中心寄せ、maxWidth）
  - Paper（設定フォーム、`elevation`あり）
    - Typography（タイトル）
    - Stack/Grid（入力群）
      - TextField: セル幅(px)
      - TextField: 横のマス数
      - TextField: 縦のマス数
      - Slider: セル幅のクイック調整（例: 5–50px、1刻み）
      - Chip群: プリセット（10x10 / 20x15 / 30x20 など）
      - FormControlLabel + Switch: 自動プレビューON/OFF
      - Stack: アクションボタン
        - Button（アイコン付き）: 描画（主要アクション）
        - Button（アイコン付き）: リセット（初期値に戻す）
  - Paper（プレビュー）
    - Typography: キャンバス情報（幅×高さ）
    - canvas: 枠線付き、サイズは入力から自動算出

## バリデーション / UX

- ルール: すべて1以上の整数。極端な値（例: `cellSize > 200`、`cols/rows > 400`）は警告。
- UI: `TextField`の`error`と`helperText`で可視フィードバック。
- 自動プレビューON時: 入力変更後300msデバウンスで描画を走らせる。
- 描画不可（バリデーションNG）の場合は「描画」ボタンを`disabled`。

## アクセシビリティ

- すべての入力に`label`、適切な`aria-*`属性付与。
- キーボード操作: `Slider`は矢印キーで微調整可能、`Enter`で描画。

## 状態管理 / データモデル

- Reactローカルステートで完結。
- 型: `GridParams { cellSize: number; cols: number; rows: number; }`
- 付随状態: `autoPreview: boolean`、`error: string | null`。
- 派生値: `canvasWidth = cellSize * cols`、`canvasHeight = cellSize * rows`。

## アーキテクチャ / 実装方針

- `src/App.tsx` をMUIベースに置換（既存の`drawGrid`ロジックは継承しつつUIを刷新）。
- `ThemeProvider` + `CssBaseline` を適用（`App.tsx`内で簡易導入）。
- `drawGrid(params)` は既存同様にキャンバスへ縦横罫線描画。
  - crisp lineのため 0.5ピクセルオフセットを継続使用。
  - キャンバス`width/height`は都度`params`から再計算。
- 自動プレビュー: `useEffect` + `setTimeout`で300msデバウンス。
  - `autoPreview === true` かつバリデーションOKのときのみ自動描画。
- 互換性: `id="canvas"` を維持してWASM側の将来呼出に対応。

## コンポーネントと主なProps

- `TextField`
  - `type="number"`, `label`, `value`, `onChange`, `error`, `helperText`, `inputProps={ min: 1 }`
- `Slider`
  - `min=5`, `max=50`, `step=1`, `value`, `onChange`
- `Chip`
  - `label="10x10"` など、クリックで`params`に適用
- `Switch`
  - `checked={autoPreview}` / `onChange`
- `Button`
  - `variant="contained"`（描画）/ `variant="outlined"`（リセット）
  - `startIcon={<PlayArrow/>}` / `startIcon={<RestartAlt/>}`

## エラーハンドリング

- 数値未入力や0以下は即エラー表示、描画をスキップ。
- 上限超過は警告表示（helperText）＋描画ボタン無効化。

## パフォーマンス配慮

- デバウンスで過剰な再描画を防止。
- 罫線本数は `cols + rows` に比例（入力上限で大規模化を回避）。

## 変更ファイル

- 追加: 依存パッケージ（`package.json`）
- 更新: `src/App.tsx`（MUIコンポーネント適用、状態と描画の整理）
- 既存の`wasm`配下は変更なし（将来拡張ポイント）

## ロールアウト手順

1. 依存追加: `npm i @mui/material @emotion/react @emotion/styled @mui/icons-material`
2. `App.tsx` をMUI UIへ置換（既存描画ロジックを移植）
3. 手動動作確認（入力/プリセット/自動プレビュー/バリデーション/キャンバスサイズ）
4. ブランチ: `feature/create-maze-UI` にコミット/プッシュ
5. PR作成（Base: `main` 予定。リポジトリのデフォルトブランチに合わせて調整）

## 将来拡張

- WASM（Kruskal等）の結果（壁の有無）に応じた線分描画へ拡張。
- ダークモード切替（`createTheme` でモード切替）。
- プリセットの保存（`localStorage` など）。
- Canvas解像度と表示倍率の分離（HiDPI対応）。

