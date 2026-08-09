import { useRef, useState } from 'react';
import type { Meta, StoryObj } from '@storybook/react-vite';
import EditorPane from './EditorPane';

const SAMPLE_MARKDOWN = `# Heading 1
## Heading 2

Normal paragraph text.

### Heading 3

More content below.`;

const EditorPanePreview = ({ initialMarkdown }: { initialMarkdown: string }) => {
    const lineNumRef = useRef<HTMLDivElement>(null);
    const preRef = useRef<HTMLPreElement>(null);
    const textareaRef = useRef<HTMLTextAreaElement>(null);
    const [markdown, setMarkdown] = useState(initialMarkdown);
    const [lineWraps, setLineWraps] = useState<number[]>([]);

    const handleScroll = () => {
        const scrollTop = textareaRef.current?.scrollTop ?? 0;
        if (preRef.current) preRef.current.scrollTop = scrollTop;
        if (lineNumRef.current) lineNumRef.current.scrollTop = scrollTop;
    };

    return (
        <EditorPane
            lineNumRef={lineNumRef}
            preRef={preRef}
            textareaRef={textareaRef}
            markdown={markdown}
            lineCount={Math.max(markdown.split('\n').length, 1)}
            lineWraps={lineWraps}
            onChange={e => setMarkdown(e.target.value)}
            onScroll={handleScroll}
            onInsert={(newMarkdown, cursorPos) => {
                setMarkdown(newMarkdown);
                setTimeout(() => {
                    if (textareaRef.current) {
                        textareaRef.current.selectionStart = cursorPos;
                        textareaRef.current.selectionEnd = cursorPos;
                    }
                }, 0);
            }}
            onLineWrapsChange={setLineWraps}
        />
    );
};

const meta = {
    title: 'MarkdownEditor/parts/EditorPane',
    component: EditorPanePreview,
} satisfies Meta<typeof EditorPanePreview>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
    args: { initialMarkdown: SAMPLE_MARKDOWN },
};

export const Empty: Story = {
    args: { initialMarkdown: '' },
};
