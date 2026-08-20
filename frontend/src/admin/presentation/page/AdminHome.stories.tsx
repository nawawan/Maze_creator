import type { Meta, StoryObj } from "@storybook/react-vite";

import AdminHome from "./AdminHome";

const meta = {
  title: "AdminHome",
  component: AdminHome,
} satisfies Meta<typeof AdminHome>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    posts: [
      { id: "1", title: "title", date: "10日前", tag: "TECH" },
      { id: "2", title: "title", date: "10日前", tag: "TRAVEL" },
      { id: "3", title: "title", date: "10日前", tag: "TECH" },
      { id: "4", title: "title", date: "10日前" },
      { id: "5", title: "title", date: "10日前", tag: "DIARY" },
      { id: "6", title: "title", date: "10日前" },
    ],
    selectedTag: "",
  },
};

export const TechTabSelected: Story = {
  args: {
    posts: [
      { id: "1", title: "title", date: "10日前", tag: "TECH" },
      { id: "3", title: "title", date: "10日前", tag: "TECH" },
    ],
    selectedTag: "TECH",
  },
};
