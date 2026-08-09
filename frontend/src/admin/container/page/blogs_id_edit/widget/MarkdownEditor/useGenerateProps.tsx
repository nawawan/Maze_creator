import { useCallback, useEffect, useState } from 'react';
import * as Sentry from '@sentry/react';
import type { MarkdownEditorProps } from '../../../../../../presentation/widgets/MarkdownEditor/MarkdownEditor';
import type { BlogDetails } from '../../../../../../shared/types/blog';
import { useDebouncedCallback } from 'use-debounce';

const useGenerateProps = (article_id: string): MarkdownEditorProps & { loaded: boolean } => {
    const [markdown, setMarkdown] = useState<string | null>(null);
    const [title, setTitle] = useState<string | undefined>(undefined);
    const [slug, setSlug] = useState<string | undefined>(undefined);
    const [tag, setTag] = useState<string | undefined>(undefined);
    const [isPublished, setIsPublished] = useState(false);
    const [statusLoaded, setStatusLoaded] = useState(false);

    useEffect(() => {
        const fetchMarkdown = async () => {
            const localDraft = localStorage.getItem(article_id);
            if (localDraft) {
                setMarkdown(localDraft);
                return;
            }
            try {
                const res = await fetch(`/api/blogs/${article_id}/md`);
                const draft: string = await res.json();
                setMarkdown(draft);
            } catch (e) {
                Sentry.captureException(e);
                setMarkdown('');
            }
        };

        const fetchStatus = async () => {
            try {
                const res = await fetch(`/api/blogs/${article_id}`);
                if (!res.ok) return;
                const blog: BlogDetails = await res.json();
                setIsPublished(blog.status === 'PUBLISHED');
                setTitle(blog.title);
                setSlug(blog.slug);
                setTag(blog.tag);
            } catch (e) {
                Sentry.captureException(e);
            } finally {
                setStatusLoaded(true);
            }
        };

        fetchMarkdown();
        fetchStatus();
    }, [article_id]);

    const handleTemporarySave = useDebouncedCallback((markdown: string) => {
        if(article_id == "") return;
        localStorage.setItem(article_id, markdown);
    }, 2000);

    const temporarySave = (inputText: string) => {
        handleTemporarySave(inputText);
    };

    const handleSave = useCallback(async (markdown: string, id: string, title?: string, slug?: string, status?: string, tag?: string) => {
        try {
            const blogRes = await fetch("/api/admin/blogs", {
                method: "POST",
                body: JSON.stringify({
                    id: id,
                    title: title,
                    slug: slug,
                    content: markdown,
                    status: status,
                    tag: tag,
                }),
            });
            if (!blogRes.ok) throw new Error(`Failed to save blog: ${blogRes.status}`);

            const mdRes = await fetch(`/api/admin/blogs/${article_id}/md`, {
                method: "POST",
                body: markdown,
            });
            if (!mdRes.ok) throw new Error(`Failed to save markdown: ${mdRes.status}`);

            if (status === 'PUBLISHED') setIsPublished(true);
        } catch (e) {
            Sentry.captureException(e);
            throw e;
        }
    }, [article_id]);

    return {
        id: article_id,
        markdown: markdown ?? '',
        loaded: markdown !== null && statusLoaded,
        title,
        slug,
        tag,
        isPublished,
        onSave: handleSave,
        onSaveTemporary: temporarySave
    };
};

export default useGenerateProps;