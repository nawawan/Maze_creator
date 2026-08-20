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
                outline: "test用",
                publishedAtLabel: "2026-07-31",
                tag: "TECH",
            },
            {
                id: "2",
                title: "title2",
                outline: "test2用",
                publishedAtLabel: "2026-07-28",
                tag: "TRAVEL",
            },
            {
                id: "3",
                title: "title3",
                outline: "test3用",
                publishedAtLabel: "2024-05-10",
            },
        ],
        onClick: (id: string) => alert(`記事ID: ${id}がクリックされました`),
        selectedTag: "",
    }
}

export const TechTabSelected: Story = {
    args: {
        posts: [
            {
                id: "1",
                title: "title1",
                outline: "test用",
                publishedAtLabel: "2026-07-31",
                tag: "TECH",
            },
        ],
        onClick: (id: string) => alert(`記事ID: ${id}がクリックされました`),
        selectedTag: "TECH",
    }
}

export const FetchingNextTag: Story = {
    args: {
        posts: [
            {
                id: "1",
                title: "title1",
                outline: "test用",
                publishedAtLabel: "2026-07-31",
                tag: "TECH",
            },
        ],
        onClick: (id: string) => alert(`記事ID: ${id}がクリックされました`),
        selectedTag: "TRAVEL",
        isFetching: true,
    }
}
