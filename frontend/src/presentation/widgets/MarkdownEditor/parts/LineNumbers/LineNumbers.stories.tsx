import { useRef } from 'react';
import type { Meta, StoryObj } from '@storybook/react-vite';
import LineNumbers from './LineNumbers';

const LineNumbersPreview = ({ lineCount, lineWraps }: { lineCount: number; lineWraps?: number[] }) => {
    const scrollRef = useRef<HTMLDivElement>(null);
    return <LineNumbers scrollRef={scrollRef} lineCount={lineCount} lineWraps={lineWraps} />;
};

const meta = {
    title: 'MarkdownEditor/parts/LineNumbers',
    component: LineNumbersPreview,
} satisfies Meta<typeof LineNumbersPreview>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
    args: { lineCount: 10 },
};

export const ManyLines: Story = {
    args: { lineCount: 50 },
};

// Line 3 wraps across 4 visual rows (e.g. a long unbroken line), so its gutter
// entry grows to match instead of every entry staying a single row tall.
export const WithWrappedLine: Story = {
    args: { lineCount: 5, lineWraps: [1, 1, 4, 1, 1] },
};
