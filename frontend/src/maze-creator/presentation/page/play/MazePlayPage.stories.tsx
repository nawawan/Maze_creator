import type { Meta, StoryObj } from "@storybook/react-vite";

import MazePlayPage from "./MazePlayPage";

const meta = {
  title: "MazePlayPage",
  component: MazePlayPage,
} satisfies Meta<typeof MazePlayPage>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Playing: Story = {
  args: {
    hasMaze: true,
    isCleared: false,
    canvasRef: { current: null },
    onMove: () => {},
    onPlayAgain: () => {},
    onBackToCreator: () => {},
  },
};

export const Cleared: Story = {
  args: {
    hasMaze: true,
    isCleared: true,
    canvasRef: { current: null },
    onMove: () => {},
    onPlayAgain: () => {},
    onBackToCreator: () => {},
  },
};

export const NoMaze: Story = {
  args: {
    hasMaze: false,
    isCleared: false,
    canvasRef: { current: null },
    onMove: () => {},
    onPlayAgain: () => {},
    onBackToCreator: () => {},
  },
};
