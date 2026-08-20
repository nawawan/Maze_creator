import { useLayoutEffect, useRef } from 'react';
import { sharedEditorStyle, ROW_HEIGHT_PX } from '../constants';

const escapeHtml = (text: string): string =>
    text.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');

const highlightMarkdownSyntax = (text: string): string =>
    text
        .split('\n')
        .map(line => {
            const headingMatch = line.match(/^(#{1,6})(.*)$/);
            if (headingMatch) {
                return (
                    `<span style="color:#8b7ce1">${escapeHtml(headingMatch[1])}</span>` +
                    `<span style="color:#8b7ce1">${escapeHtml(headingMatch[2])}</span>`
                );
            }
            return `<span style="color:#e2e8f0">${escapeHtml(line)}</span>`;
        })
        .join('\n');

export type EditorOverlayProps = {
    preRef: React.RefObject<HTMLPreElement | null>;
    textareaRef: React.RefObject<HTMLTextAreaElement | null>;
    markdown: string;
    onChange: (e: React.ChangeEvent<HTMLTextAreaElement>) => void;
    onScroll: () => void;
    onInsert: (newMarkdown: string, cursorPos: number) => void;
    onLineWrapsChange?: (lineWraps: number[]) => void;
};

const EditorOverlay = ({ preRef, textareaRef, markdown, onChange, onScroll, onInsert, onLineWrapsChange }: EditorOverlayProps) => {
    const measureRef = useRef<HTMLDivElement>(null);
    const lineRefs = useRef<Map<number, HTMLDivElement>>(new Map());
    const lines = markdown.split('\n');

    // The visible <pre>/<textarea> wrap long lines (white-space: pre-wrap), so a
    // single logical line can span multiple visual rows. This hidden mirror
    // shares the exact same font/width/wrapping rules and lets us measure how
    // many visual rows each logical line actually occupies, so the line-number
    // gutter (LineNumbers) can grow to match instead of drifting out of sync.
    useLayoutEffect(() => {
        const measure = () => {
            const wraps = lines.map((_, i) => {
                const el = lineRefs.current.get(i);
                if (!el) return 1;
                return Math.max(1, Math.round(el.getBoundingClientRect().height / ROW_HEIGHT_PX));
            });
            onLineWrapsChange?.(wraps);
        };
        measure();
        const el = measureRef.current;
        if (!el || typeof ResizeObserver === 'undefined') return;
        const observer = new ResizeObserver(measure);
        observer.observe(el);
        return () => observer.disconnect();
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [markdown, onLineWrapsChange]);

    const handleDrop = async (e: React.DragEvent<HTMLDivElement>) => {
        e.preventDefault();
        const files = e.dataTransfer.files;
        if (files.length === 0) return;
        const file = files[0];
        if (!file.type.startsWith("image/")) return;

        const cursorPos = textareaRef.current?.selectionStart ?? markdown.length;
        const res = await fetch("/api/admin/blogs/images", {
            method: "PUT",
            body: file,
            headers: { 'Content-Type': file.type }
        });
        const url: string = await res.json();
        const inserted = `![${file.name}](${url})`;
        const newMarkdown = markdown.slice(0, cursorPos) + inserted + markdown.slice(cursorPos);
        onInsert(newMarkdown, cursorPos + inserted.length);
    };
    return (
        <div 
            style={{ position: 'relative', flex: 1, overflow: 'hidden' }}
            onDrop={handleDrop}
            onDragOver={(e) => e.preventDefault()}
        >
            <div
                ref={measureRef}
                aria-hidden="true"
                style={{
                    ...sharedEditorStyle,
                    position: 'absolute',
                    top: 0,
                    left: 0,
                    right: 0,
                    overflow: 'hidden',
                    visibility: 'hidden',
                    pointerEvents: 'none',
                }}
            >
                {lines.map((line, i) => (
                    <div
                        key={i}
                        ref={(el) => {
                            if (el) lineRefs.current.set(i, el);
                            else lineRefs.current.delete(i);
                        }}
                    >
                        {line.length > 0 ? line : '​'}
                    </div>
                ))}
            </div>
            <pre
                ref={preRef}
                aria-hidden="true"
                style={{
                    ...sharedEditorStyle,
                    position: 'absolute',
                    top: 0,
                    left: 0,
                    right: 0,
                    bottom: 0,
                    overflow: 'hidden',
                    pointerEvents: 'none',
                    backgroundColor: 'transparent',
                    color: '#e2e8f0',
                }}
                dangerouslySetInnerHTML={{ __html: highlightMarkdownSyntax(markdown) + '\n' }}
            />
            <textarea
                ref={textareaRef}
                value={markdown}
                onChange={onChange}
                onScroll={onScroll}
                placeholder="Enter markdown here..."
                spellCheck={false}
                style={{
                    ...sharedEditorStyle,
                    position: 'absolute',
                    top: 0,
                    left: 0,
                    width: '100%',
                    height: '100%',
                    resize: 'none',
                    border: 'none',
                    outline: 'none',
                    backgroundColor: 'transparent',
                    color: 'transparent',
                    caretColor: '#e2e8f0',
                    overflow: 'auto',
                }}
            />
        </div>
    );  
};

export default EditorOverlay;
