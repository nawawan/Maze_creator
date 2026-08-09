import type { Meta, StoryObj } from '@storybook/react-vite';
import MarkdownEditor from './MarkdownEditor';

const meta = {
    title: 'MarkdownEditor',
    component: MarkdownEditor,
} satisfies Meta<typeof MarkdownEditor>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
    args: {
        id: "test",
        tag: "TECH",
        onSave: async (markdown: string, id: string) => { alert(markdown + id); },
    },
};