import { useEffect, useRef, useState } from "react";
import { useLocation, useNavigate } from "react-router-dom";

import {
  MAZE_PLAY_CANVAS_ID,
  type MazePlayLocationState,
  type MazePlayPageProps,
  type MoveDirection,
} from "../../../presentation/page/play/MazePlayPage";
import { Direction, MazeGame } from "../../../wasm/pkg/wasm.js";

const DIRECTION_MAP: Record<MoveDirection, Direction> = {
  up: Direction.Up,
  right: Direction.Right,
  down: Direction.Down,
  left: Direction.Left,
};

const KEY_TO_DIRECTION: Record<string, MoveDirection> = {
  ArrowUp: "up",
  ArrowRight: "right",
  ArrowDown: "down",
  ArrowLeft: "left",
};

export const useGenerateProps = (): MazePlayPageProps => {
  const location = useLocation();
  const navigate = useNavigate();
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const gameRef = useRef<MazeGame | null>(null);
  const [isCleared, setIsCleared] = useState(false);

  const mazeState = location.state as MazePlayLocationState | null;
  const hasMaze = Boolean(
    mazeState &&
      mazeState.rows > 0 &&
      mazeState.cols > 0 &&
      mazeState.walls.length === mazeState.rows * mazeState.cols,
  );

  const startGame = () => {
    if (!mazeState || !canvasRef.current) return;
    const { walls, rows, cols, cellSize } = mazeState;

    canvasRef.current.width = cols * cellSize;
    canvasRef.current.height = rows * cellSize;

    gameRef.current?.free();
    gameRef.current = new MazeGame(
      MAZE_PLAY_CANVAS_ID,
      Uint8Array.from(walls),
      rows,
      cols,
      cellSize,
    );
    setIsCleared(false);
  };

  useEffect(() => {
    if (!hasMaze) return;
    startGame();
    return () => {
      gameRef.current?.free();
      gameRef.current = null;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [hasMaze]);

  const onMove = (direction: MoveDirection) => {
    const game = gameRef.current;
    if (!game) return;
    game.try_move(DIRECTION_MAP[direction]);
    setIsCleared(game.is_cleared());
  };

  useEffect(() => {
    if (!hasMaze) return;
    const onKeyDown = (event: KeyboardEvent) => {
      const direction = KEY_TO_DIRECTION[event.key];
      if (!direction) return;
      event.preventDefault();
      onMove(direction);
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [hasMaze]);

  return {
    hasMaze,
    isCleared,
    canvasRef,
    onMove,
    onPlayAgain: startGame,
    onBackToCreator: () => navigate("/maze"),
  };
};

export default useGenerateProps;
