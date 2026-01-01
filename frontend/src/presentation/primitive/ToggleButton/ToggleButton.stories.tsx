import type { Meta, StoryObj } from '@storybook/react-vite';

import ToggleButton from './ToggleButton';

const meta = {
    title: "ToggleButton",
    component: ToggleButton,
} satisfies Meta<typeof ToggleButton>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
    args: {
        displayText: "過去のブログ",
        additionalContents: ["2022年の記事", "2021年の記事", "2020年の記事"],
    }
}