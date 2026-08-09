import type { RefObject } from "react";
import {
  Box,
  Button,
  Container,
  Paper,
  Stack,
  Typography,
} from "@mui/material";
import KeyboardArrowUpIcon from "@mui/icons-material/KeyboardArrowUp";
import KeyboardArrowDownIcon from "@mui/icons-material/KeyboardArrowDown";
import KeyboardArrowLeftIcon from "@mui/icons-material/KeyboardArrowLeft";
import KeyboardArrowRightIcon from "@mui/icons-material/KeyboardArrowRight";
import EmojiEventsIcon from "@mui/icons-material/EmojiEvents";

export const MAZE_PLAY_CANVAS_ID = "maze-play-canvas";

export type MoveDirection = "up" | "right" | "down" | "left";

export type MazePlayLocationState = {
  walls: number[];
  rows: number;
  cols: number;
  cellSize: number;
};

export type MazePlayPageProps = {
  hasMaze: boolean;
  isCleared: boolean;
  canvasRef: RefObject<HTMLCanvasElement | null>;
  onMove: (direction: MoveDirection) => void;
  onPlayAgain: () => void;
  onBackToCreator: () => void;
};

export const MazePlayPage = (props: MazePlayPageProps) => {
  const { hasMaze, isCleared, canvasRef, onMove, onPlayAgain, onBackToCreator } =
    props;

  if (!hasMaze) {
    return (
      <Container maxWidth="sm" sx={{ py: 4 }}>
        <Paper elevation={2} sx={{ p: 3 }}>
          <Stack spacing={2} alignItems="flex-start">
            <Typography variant="h6">遊べる迷路がありません</Typography>
            <Typography variant="body2" color="text.secondary">
              迷路作成ページでランダム迷路を作ってから「この迷路で遊ぶ」を押してください。
            </Typography>
            <Button variant="contained" onClick={onBackToCreator}>
              迷路作成ページへ
            </Button>
          </Stack>
        </Paper>
      </Container>
    );
  }

  return (
    <Container maxWidth="sm" sx={{ py: 4 }}>
      <Stack spacing={3}>
        <Paper elevation={2} sx={{ p: 3 }}>
          <Stack spacing={2} alignItems="center">
            <Typography variant="h5">迷路で遊ぶ</Typography>

            {isCleared && (
              <Stack
                direction="row"
                spacing={1}
                alignItems="center"
                sx={{ color: "success.main" }}
              >
                <EmojiEventsIcon />
                <Typography variant="subtitle1">クリア！</Typography>
              </Stack>
            )}

            <Box
              sx={{
                border: "1px solid",
                borderColor: "divider",
                width: "fit-content",
              }}
            >
              <canvas
                id={MAZE_PLAY_CANVAS_ID}
                ref={canvasRef}
                style={{ display: "block" }}
              />
            </Box>

            <Box
              sx={{
                display: "grid",
                gridTemplateColumns: "repeat(3, 48px)",
                gridTemplateRows: "repeat(3, 48px)",
                gap: 1,
              }}
            >
              <Box />
              <Button
                variant="outlined"
                onClick={() => onMove("up")}
                sx={{ minWidth: 0 }}
              >
                <KeyboardArrowUpIcon />
              </Button>
              <Box />

              <Button
                variant="outlined"
                onClick={() => onMove("left")}
                sx={{ minWidth: 0 }}
              >
                <KeyboardArrowLeftIcon />
              </Button>
              <Box />
              <Button
                variant="outlined"
                onClick={() => onMove("right")}
                sx={{ minWidth: 0 }}
              >
                <KeyboardArrowRightIcon />
              </Button>

              <Box />
              <Button
                variant="outlined"
                onClick={() => onMove("down")}
                sx={{ minWidth: 0 }}
              >
                <KeyboardArrowDownIcon />
              </Button>
              <Box />
            </Box>

            <Typography variant="caption" color="text.secondary">
              矢印キーでも操作できます
            </Typography>

            <Stack direction="row" spacing={1}>
              <Button variant="text" onClick={onBackToCreator}>
                迷路作成ページへ戻る
              </Button>
              <Button variant="outlined" onClick={onPlayAgain}>
                もう一度遊ぶ
              </Button>
            </Stack>
          </Stack>
        </Paper>
      </Stack>
    </Container>
  );
};

export default MazePlayPage;
