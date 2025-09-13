import { useEffect, useMemo, useRef, useState } from 'react';
import './App.css';
import {
  Box,
  Button,
  Chip,
  Container,
  CssBaseline,
  FormControlLabel,
  Paper,
  Slider,
  Stack,
  Switch,
  Tab,
  Tabs,
  TextField,
  Typography,
  createTheme,
  ThemeProvider,
} from '@mui/material';
import Grid from '@mui/material/Grid';
import PlayArrowIcon from '@mui/icons-material/PlayArrow';
import RestartAltIcon from '@mui/icons-material/RestartAlt';

import { draw_maze, draw_single_stroke_maze } from './wasm';

type GridParams = {
  cellSize: number; // 1マスの幅（px）
  cols: number; // 横方向のマス数
  rows: number; // 縦方向のマス数
};

const DEFAULT_PARAMS: GridParams = { cellSize: 20, cols: 15, rows: 10 };
type Mode = 'random' | 'single';

function App() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);

  const [params, setParams] = useState<GridParams>(DEFAULT_PARAMS);
  const [autoPreview, setAutoPreview] = useState<boolean>(true);
  const [mode, setMode] = useState<Mode>('random');
  const [themeMode, setThemeMode] = useState<'light' | 'dark'>('light');

  const { widthPx, heightPx } = useMemo(() => ({
    widthPx: params.cellSize * params.cols,
    heightPx: params.cellSize * params.rows,
  }), [params]);

  const validation = useMemo(() => {
    const errors: { cellSize?: string; cols?: string; rows?: string; mode?: string } = {};
    const { cellSize, cols, rows } = params;
    const isInt = (n: number) => Number.isFinite(n) && Math.floor(n) === n;
    if (!Number.isFinite(cellSize) || cellSize <= 0) errors.cellSize = '1以上の数値を入力してください';
    if (!Number.isFinite(cols) || cols <= 0 || !isInt(cols)) errors.cols = '1以上の整数を入力してください';
    if (!Number.isFinite(rows) || rows <= 0 || !isInt(rows)) errors.rows = '1以上の整数を入力してください';
    if (cellSize > 200) errors.cellSize = '大きすぎます (<= 200)';
    if (cols > 400) errors.cols = '大きすぎます (<= 400)';
    if (rows > 400) errors.rows = '大きすぎます (<= 400)';
    if (mode === 'single') {
      if (rows % 2 === 1 && cols % 2 === 1) {
        errors.mode = '一筆書きは縦横どちらかが偶数である必要があります';
      }
      if (rows <= 2 && cols <= 2) {
        errors.mode = 'サイズが小さすぎます (いずれかを3以上にしてください)';
      }
    }
    const valid = Object.keys(errors).length === 0;
    return { errors, valid };
  }, [params, mode]);

  const ensureCanvasAndDraw = ({ cellSize, cols, rows }: GridParams, m: Mode) => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    // const ctx = canvas.getContext('2d');
    // if (!ctx) return;

    // キャンバスサイズをグリッドに合わせる
    const width = cellSize * cols;
    const height = cellSize * rows;
    canvas.width = width;
    canvas.height = height;
    if (m === 'single') {
      draw_single_stroke_maze(0, 0, rows, cols, cellSize);
    } else {
      draw_maze(0, 0, rows, cols, cellSize);
    }

    // // 縦線
    // for (let x = 0; x <= cols; x++) {
    //   const px = Math.floor(x * cellSize) + 0.5; // crisp line
    //   ctx.beginPath();
    //   ctx.moveTo(px, 0);
    //   ctx.lineTo(px, height);
    //   ctx.stroke();
    // }

    // // 横線
    // for (let y = 0; y <= rows; y++) {
    //   const py = Math.floor(y * cellSize) + 0.5; // crisp line
    //   ctx.beginPath();
    //   ctx.moveTo(0, py);
    //   ctx.lineTo(width, py);
    //   ctx.stroke();
    // }
  };

  // 自動プレビュー（デバウンス）
  useEffect(() => {
    if (!autoPreview || !validation.valid) return;
    const t = setTimeout(() => ensureCanvasAndDraw(params, mode), 300);
    return () => clearTimeout(t);
  }, [autoPreview, validation.valid, params, mode]);

  const onSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!validation.valid) return;
    ensureCanvasAndDraw(params, mode);
  };

  const onReset = () => {
    setParams(DEFAULT_PARAMS);
    setMode('random');
    if (autoPreview) {
      ensureCanvasAndDraw(DEFAULT_PARAMS, 'random');
    }
  };

  const theme = useMemo(() => createTheme({
    palette: {
      mode: themeMode,
    },
    shape: { borderRadius: 10 },
  }), [themeMode]);

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Container maxWidth="md" sx={{ py: 4 }}>
        <Stack spacing={3}>
          <Paper elevation={2} sx={{ p: 3 }}>
            <Typography variant="h5" gutterBottom>
              迷路グリッドの設定
            </Typography>
            <Tabs value={mode} onChange={(_, v) => setMode(v)} sx={{ mb: 2 }}>
              <Tab value="random" label="Random" />
              <Tab value="single" label="Single-Stroke" />
            </Tabs>
            <Box component="form" onSubmit={onSubmit}>
              <Grid container spacing={2}>
                <Grid item xs={12} md={4}>
                  <TextField
                    fullWidth
                    label="グリッド幅 (px)"
                    type="number"
                    value={params.cellSize}
                    onChange={(e) =>
                      setParams((p) => ({ ...p, cellSize: Number(e.target.value) }))
                    }
                    slotProps={{ input: { min: 1 } }}
                    error={Boolean(validation.errors.cellSize)}
                    helperText={validation.errors.cellSize || '1以上の数値'}
                  />
                </Grid>
                <Grid item xs={12} md={4}>
                  <TextField
                    fullWidth
                    label="横のマス数"
                    type="number"
                    value={params.cols}
                    onChange={(e) => setParams((p) => ({ ...p, cols: Number(e.target.value) }))}
                    slotProps={{ input: { min: 1 } }}
                    error={Boolean(validation.errors.cols)}
                    helperText={validation.errors.cols || '1以上の整数'}
                  />
                </Grid>
                <Grid item xs={12} md={4}>
                  <TextField
                    fullWidth
                    label="縦のマス数"
                    type="number"
                    value={params.rows}
                    onChange={(e) => setParams((p) => ({ ...p, rows: Number(e.target.value) }))}
                    slotProps={{ input: { min: 1 } }}
                    error={Boolean(validation.errors.rows)}
                    helperText={validation.errors.rows || '1以上の整数'}
                  />
                </Grid>

                <Grid item xs={12} md={6}>
                  <Stack spacing={1} sx={{ px: 1 }}>
                    <Typography variant="body2" color="text.secondary">
                      セル幅クイック調整
                    </Typography>
                    <Slider
                      value={params.cellSize}
                      min={5}
                      max={50}
                      step={1}
                      onChange={(_, val) =>
                        setParams((p) => ({ ...p, cellSize: Array.isArray(val) ? val[0] : Number(val) }))
                      }
                    />
                  </Stack>
                </Grid>

                <Grid item xs={12} md={6}>
                  <Stack direction="row" spacing={1} alignItems="center" sx={{ height: '100%' }}>
                    <Typography variant="body2" color="text.secondary">
                      プリセット:
                    </Typography>
                    {[
                      { c: 10, r: 10 },
                      { c: 20, r: 15 },
                      { c: 30, r: 20 },
                    ].map((p) => (
                      <Chip
                        key={`${p.c}x${p.r}`}
                        label={`${p.c}x${p.r}`}
                        onClick={() => setParams((prev) => ({ ...prev, cols: p.c, rows: p.r }))}
                        variant="outlined"
                        size="small"
                      />
                    ))}
                  </Stack>
                </Grid>

                <Grid item xs={12}>
                  {mode === 'single' && validation.errors.mode && (
                    <Typography color="error" variant="body2" sx={{ mb: 1 }}>
                      {validation.errors.mode}
                    </Typography>
                  )}
                  <Stack direction="row" alignItems="center" justifyContent="space-between">
                    <Stack direction="row" spacing={2} alignItems="center">
                      <FormControlLabel
                        control={<Switch checked={autoPreview} onChange={(e) => setAutoPreview(e.target.checked)} />}
                        label="自動プレビュー"
                      />
                      <FormControlLabel
                        control={<Switch checked={themeMode === 'dark'} onChange={(e) => setThemeMode(e.target.checked ? 'dark' : 'light')} />}
                        label="ダークモード"
                      />
                    </Stack>
                    <Stack direction="row" spacing={1}>
                      <Button
                        type="submit"
                        variant="contained"
                        startIcon={<PlayArrowIcon />}
                        disabled={!validation.valid}
                      >
                        生成して表示
                      </Button>
                      <Button
                        type="button"
                        variant="outlined"
                        color="inherit"
                        startIcon={<RestartAltIcon />}
                        onClick={onReset}
                      >
                        リセット
                      </Button>
                    </Stack>
                  </Stack>
                </Grid>
              </Grid>
            </Box>
          </Paper>

          <Paper elevation={2} sx={{ p: 3 }}>
            <Stack spacing={1}>
              <Typography variant="subtitle1">プレビュー</Typography>
              <Typography variant="caption" color="text.secondary">
                {widthPx} x {heightPx} px
              </Typography>
              <Box sx={{ border: '1px solid', borderColor: 'divider', width: 'fit-content' }}>
                <canvas id="canvas" ref={canvasRef} />
              </Box>
            </Stack>
          </Paper>
        </Stack>
      </Container>
    </ThemeProvider>
  );
}

export default App;
