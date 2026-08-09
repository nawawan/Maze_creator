import type { Meta, StoryObj } from '@storybook/react-vite';

import InlineEditableField from './InlineEditableField';

const meta = {
    title: 'InlineEditableField',
    component: InlineEditableField,
} satisfies Meta<typeof InlineEditableField>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
    args: {
        value: 'サンプルタイトル',
        placeholder: 'タイトルを入力',
        fontWeight: 700,
        onChange: () => {},
    },
};

export const WithAdornmentAndError: Story = {
    args: {
        value: 'invalid slug',
        placeholder: 'slugを入力',
        startAdornment: '/',
        error: '英小文字・数字・ハイフンのみ使用できます',
        onChange: () => {},
    },
};
