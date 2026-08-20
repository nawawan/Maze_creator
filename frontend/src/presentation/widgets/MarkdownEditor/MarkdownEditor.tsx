import { useState, useRef, useCallback, useEffect } from 'react';
import { Marked } from 'marked';
import { markedHighlight } from 'marked-highlight';
import hljs from 'highlight.js';
import sanitizeHtml from 'sanitize-html';
import { Box, Button, CircularProgress, MenuItem, Select, type SelectChangeEvent, Stack, styled } from '@mui/material';
import EditorPane from './parts/EditorPane/EditorPane';
import MarkdownPreview from './parts/MarkdownPreview/MarkdownPreview';
import InlineEditableField from '../../primitive/InlineEditableField/InlineEditableField';
import Snackbar from '../../primitive/Snackbar/Snackbar';
import { MarkdownEditorSchema } from './validation/SubmitBlogValidation';
import { BLOG_TAGS } from '../../../shared/types/blog';

const marked = new Marked(
    markedHighlight({
        emptyLangClass: 'hljs',
        langPrefix: 'hljs language-',
        highlight(code, lang) {
            const language = hljs.getLanguage(lang) ? lang : 'plaintext';
            try {
                return hljs.highlight(code, { language }).value;
            } catch {
                return code;
            }
        },
    })
);

const StyledButton = styled(Button)({
    backgroundColor: '#4f46e5',
    borderRadius: '5px',
    color: '#fff',
    '&:hover': {
        backgroundColor: '#4338ca',
    },
});

export type MarkdownEditorProps = {
    id: string;
    title?: string;
    slug?: string;
    tag?: string;
    markdown?: string;
    isPublished?: boolean;
    onSave: (markdown: string, id: string, title?: string, slug?: string, status?: string, tag?: string) => Promise<void>;
    onSaveTemporary?: (markdown: string) => void;
};

const MarkdownEditor = (props: MarkdownEditorProps) => {
    const { onSaveTemporary } = props;
    const [markdown, setMarkdown] = useState(props.markdown ?? "");
    const [html, setHtml] = useState('');
    const [title, setTitle] = useState(props.title);
    const [slug, setSlug] = useState(props.slug);
    const [tag, setTag] = useState(props.tag);
    const [published, setPublished] = useState(props.isPublished ?? false);
    const [errors, setErrors] = useState<{ title?: string; slug?: string }>({});
    const [lineWraps, setLineWraps] = useState<number[]>([]);
    const [isSaving, setIsSaving] = useState(false);
    const textareaRef = useRef<HTMLTextAreaElement>(null);
    const preRef = useRef<HTMLPreElement>(null);
    const lineNumRef = useRef<HTMLDivElement>(null);
    const titleInputRef = useRef<HTMLInputElement>(null);
    const slugInputRef = useRef<HTMLInputElement>(null);

    useEffect(() => {
        // Render the initial preview once on mount; subsequent edits call
        // updateMarkdown directly via handleChange/handleInsert instead.
        updateMarkdown(markdown);
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);

    const updateMarkdown = useCallback(async (text: string) => {
        setMarkdown(text);
        const rawHtml = await marked.parse(text);
        const sanitizedHtml = sanitizeHtml(rawHtml, {
            allowedTags: sanitizeHtml.defaults.allowedTags.concat(['pre', 'code', 'span', 'img']),
            disallowedTagsMode: 'recursiveEscape',
            allowedAttributes: {
                ...sanitizeHtml.defaults.allowedAttributes,
                code: ['class'],
                span: ['class'],
                img: ['src', 'alt', 'title', 'width', 'height'],
            },
        });
        onSaveTemporary?.(text);
        setHtml(sanitizedHtml);
    }, [onSaveTemporary]);

    const handleChange = async (e: React.ChangeEvent<HTMLTextAreaElement>) => {
        await updateMarkdown(e.target.value);
    };

    const handleInsert = useCallback(async (newMarkdown: string, cursorPos: number) => {
        await updateMarkdown(newMarkdown);
        requestAnimationFrame(() => {
            if (textareaRef.current) {
                textareaRef.current.selectionStart = cursorPos;
                textareaRef.current.selectionEnd = cursorPos;
            }
        });
    }, [updateMarkdown]);

    const handleLineWrapsChange = useCallback((wraps: number[]) => {
        setLineWraps(wraps);
    }, []);

    const handleTagChange = (e: SelectChangeEvent) => {
        setTag(e.target.value === '' ? undefined : e.target.value);
    };

    const handleScroll = () => {
        const scrollTop = textareaRef.current?.scrollTop ?? 0;
        if (preRef.current) preRef.current.scrollTop = scrollTop;
        if (lineNumRef.current) lineNumRef.current.scrollTop = scrollTop;
    };

    const lineCount = Math.max(markdown.split('\n').length, 1);

    const validateFields = (): boolean => {
        const result = MarkdownEditorSchema.safeParse({ title, slug });
        if (result.success) {
            setErrors({});
            return true;
        }
        const newErrors: { title?: string; slug?: string } = {};
        for (const issue of result.error.issues) {
            const field = issue.path[0];
            if (field === 'title' || field === 'slug') {
                newErrors[field] = issue.message;
            }
        }
        setErrors(newErrors);
        if (newErrors.title) titleInputRef.current?.focus();
        else if (newErrors.slug) slugInputRef.current?.focus();
        return false;
    };

    const handleSave = async () => {
        if (!validateFields()) return;
        setIsSaving(true);
        try {
            await props.onSave(markdown, props.id, title, slug, published ? 'PUBLISHED' : 'DRAFT', tag);
            Snackbar.Notify({ message: '保存しました', severity: 'success' });
        } catch {
            Snackbar.Notify({ message: '保存に失敗しました', severity: 'error' });
        } finally {
            setIsSaving(false);
        }
    };

    const handlePublish = async () => {
        if (!validateFields()) return;
        setIsSaving(true);
        try {
            await props.onSave(markdown, props.id, title, slug, 'PUBLISHED', tag);
            setPublished(true);
            Snackbar.Notify({ message: '公開しました', severity: 'success' });
        } catch {
            Snackbar.Notify({ message: '公開に失敗しました', severity: 'error' });
        } finally {
            setIsSaving(false);
        }
    };

    return (
        <Stack spacing={2} sx={{ height: '100%', overflow: 'hidden', position: 'relative', pb: '80px' }}>
            <Stack direction="row" sx={{ justifyContent: 'center'}}>
                <InlineEditableField
                    inputRef={titleInputRef}
                    value={title}
                    onChange={setTitle}
                    placeholder="タイトルを入力"
                    error={errors.title}
                    fontWeight={700}
                />
                <InlineEditableField
                    inputRef={slugInputRef}
                    value={slug}
                    onChange={setSlug}
                    placeholder="slugを入力"
                    error={errors.slug}
                    startAdornment="/"
                    color="text.secondary"
                />
            </Stack>
            <Stack direction="row" sx={{ justifyContent: 'center' }}>
                <Select
                    value={tag ?? ''}
                    onChange={handleTagChange}
                    displayEmpty
                    variant="standard"
                    disableUnderline
                    sx={{ fontSize: '0.875rem', color: 'text.secondary', minWidth: 140 }}
                >
                    <MenuItem value="">タグ未設定</MenuItem>
                    {BLOG_TAGS.map((option) => (
                        <MenuItem key={option} value={option}>{option}</MenuItem>
                    ))}
                </Select>
            </Stack>
            <Stack direction="row"  sx={{ display: 'flex', flex: 1, minHeight: 0 }}>
                <EditorPane
                    lineNumRef={lineNumRef}
                    preRef={preRef}
                    textareaRef={textareaRef}
                    markdown={markdown}
                    lineCount={lineCount}
                    lineWraps={lineWraps}
                    onChange={handleChange}
                    onScroll={handleScroll}
                    onInsert={handleInsert}
                    onLineWrapsChange={handleLineWrapsChange}
                />
                <MarkdownPreview html={html} />
            </Stack>
            <Box
                sx={{
                    position: 'absolute',
                    right: 0,
                    bottom: 0,
                    display: 'flex',
                    justifyContent: 'flex-end',
                    gap: 1.5,
                    px: 3,
                    py: 2,
                    bgcolor: 'rgba(255, 255, 255, 0.7)',
                    backdropFilter: 'blur(8px)',
                    borderTop: '1px solid rgba(0, 0, 0, 0.08)',
                }}
            >
                <Button variant="outlined" onClick={handleSave} disabled={isSaving} startIcon={isSaving ? <CircularProgress size={16} /> : undefined}>
                    SAVE
                </Button>
                {!published && (
                    <StyledButton onClick={handlePublish} disabled={isSaving} startIcon={isSaving ? <CircularProgress size={16} sx={{ color: '#fff' }} /> : undefined}>
                        公開する
                    </StyledButton>
                )}
            </Box>
            <Snackbar />
        </Stack>
    );
};

export default MarkdownEditor;
