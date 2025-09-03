import { useRef, useState } from 'react';
import './App.css';

type GridParams = {
  cellSize: number; // 1マスの幅（px）
  cols: number;     // 横方向のマス数
  rows: number;     // 縦方向のマス数
};

function App() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);

  const [params, setParams] = useState<GridParams>({
    cellSize: 20,
    cols: 15,
    rows: 10,
  });
  const [error, setError] = useState<string | null>(null);

  const drawGrid = ({ cellSize, cols, rows }: GridParams) => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // キャンバスサイズをグリッドに合わせる
    const width = cellSize * cols;
    const height = cellSize * rows;
    canvas.width = width;
    canvas.height = height;

    // クリア
    ctx.clearRect(0, 0, width, height);

    // スタイル
    ctx.strokeStyle = '#aaa';
    ctx.lineWidth = 1;

    // 縦線
    for (let x = 0; x <= cols; x++) {
      const px = Math.floor(x * cellSize) + 0.5; // crisp line
      ctx.beginPath();
      ctx.moveTo(px, 0);
      ctx.lineTo(px, height);
      ctx.stroke();
    }

    // 横線
    for (let y = 0; y <= rows; y++) {
      const py = Math.floor(y * cellSize) + 0.5; // crisp line
      ctx.beginPath();
      ctx.moveTo(0, py);
      ctx.lineTo(width, py);
      ctx.stroke();
    }
  };

  const onSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);

    const { cellSize, cols, rows } = params;
    if (!Number.isFinite(cellSize) || !Number.isFinite(cols) || !Number.isFinite(rows)) {
      setError('数値を入力してください');
      return;
    }
    if (cellSize <= 0 || cols <= 0 || rows <= 0) {
      setError('すべて1以上の数値を入力してください');
      return;
    }
    if (cellSize > 200 || cols > 400 || rows > 400) {
      // 極端なサイズの誤入力を軽くガード
      setError('入力が大きすぎます');
      return;
    }

    drawGrid({ cellSize, cols, rows });
  };

  return (
    <>
      <h1>迷路グリッドの設定</h1>

      <form className="card" onSubmit={onSubmit} style={{ display: 'grid', gap: '12px', justifyItems: 'start', justifyContent: 'center' }}>
        <label>
          グリッド幅(px):
          <input
            type="number"
            min={1}
            value={params.cellSize}
            onChange={(e) => setParams((p) => ({ ...p, cellSize: Number(e.target.value) }))}
            style={{ marginLeft: 8 }}
          />
        </label>

        <label>
          横のマス数:
          <input
            type="number"
            min={1}
            value={params.cols}
            onChange={(e) => setParams((p) => ({ ...p, cols: Number(e.target.value) }))}
            style={{ marginLeft: 8 }}
          />
        </label>

        <label>
          縦のマス数:
          <input
            type="number"
            min={1}
            value={params.rows}
            onChange={(e) => setParams((p) => ({ ...p, rows: Number(e.target.value) }))}
            style={{ marginLeft: 8 }}
          />
        </label>

        <button type="submit">グリッドを表示</button>
        {error && <div style={{ color: 'tomato' }}>{error}</div>}
      </form>

      <canvas id="canvas" ref={canvasRef} style={{ border: '1px solid #ccc', marginTop: 16 }} />
    </>
  );
}

export default App;
