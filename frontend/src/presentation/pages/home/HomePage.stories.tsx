import type { Meta, StoryObj } from "@storybook/react-vite";

import HomePage from "./HomePage";

const meta = {
  title: "HomePage",
  component: HomePage,
} satisfies Meta<typeof HomePage>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    contents: [
      { id: "1", title: "ブログ", date: "10日前" },
      { id: "2", title: "Maze Creator", date: "10日前" },
    ],
  },
};
