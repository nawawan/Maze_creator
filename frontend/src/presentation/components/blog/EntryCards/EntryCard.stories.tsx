import type { Meta, StoryObj } from '@storybook/react-vite';

import EntryCard  from './EntryCard';

const meta = {
    title : "EntryCard",
    component: EntryCard,
} satisfies Meta<typeof EntryCard>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
    args: {
        posts: [
            {
                id: "1",
                title: "title1",
                outline: "test用"
            },
            {
                id: "2",
                title: "title2",
                outline: "test2用"
            },
            {
                id: "3",
                title: "title3",
                outline: "test3用"
            },
        ]
    }
}