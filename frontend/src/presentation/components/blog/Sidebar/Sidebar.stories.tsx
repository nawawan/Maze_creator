import type { Meta, StoryObj } from '@storybook/react-vite';

import Sidebar from './Sidebar';

const meta = {
    title: "Sidebar",
    component: Sidebar,
} satisfies Meta<typeof Sidebar>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
    args: {
        years: ["2025年の記事", "2024年の記事", "2023年の記事"],
        additionalYears: ["2022年の記事", "2021年の記事", "2020年の記事"],
    }
}