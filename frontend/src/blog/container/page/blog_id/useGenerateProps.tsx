import { useState, useEffect, useRef } from "react";
import { useParams } from "react-router-dom";
import * as Sentry from "@sentry/react";

import type { BlogProps } from "../../../presentation/page/blog_id/Blog";
import SidebarContainer from "../blogs/widgets/Sidebar/Container";
import MarkdownHtml from "../../../presentation/MarkdownHtml/MarkdownHtml";
import { type BlogDetails, type BlogNotFoundReason } from "../../../../shared/types/blog";

const useGenerateProps = (initialBlog?: BlogDetails, initialNotFoundReason?: BlogNotFoundReason): BlogProps & { isLoading: boolean } => {
    const { blogId } = useParams<{ blogId: string }>();
    // Reading window here is a pure read (no mutation), so it's safe to do
    // during render. The global is only cleared inside the effect below,
    // after React has committed the value into state.
    const seededBlog = initialBlog ?? (typeof window !== "undefined" && window.__BLOG_INITIAL_DATA__?.id === blogId
        ? window.__BLOG_INITIAL_DATA__
        : undefined);

    const [blog, setBlog] = useState<BlogDetails | undefined>(seededBlog);
    const [notFoundReason, setNotFoundReason] = useState<BlogNotFoundReason | undefined>(initialNotFoundReason);
    // Server already told us whether this id exists/is published; skip the
    // redundant initial fetch. Later blogId changes (client-side nav) fetch normally.
    const hasServerAnswerRef = useRef(!!(seededBlog || initialNotFoundReason));
    const [isLoading, setIsLoading] = useState(!hasServerAnswerRef.current);

    useEffect(() => {
        if (typeof window !== "undefined") {
            window.__BLOG_INITIAL_DATA__ = undefined;
        }
        if (!blogId) {
            setIsLoading(false);
            return;
        }
        if (hasServerAnswerRef.current) {
            hasServerAnswerRef.current = false;
            setIsLoading(false);
            return;
        }

        const fetchBlog = async () => {
            try {
                const response = await fetch(`/api/blogs/${blogId}`);
                if (!response.ok) {
                    const body = await response.json().catch(() => null) as { reason?: BlogNotFoundReason } | null;
                    setBlog(undefined);
                    setNotFoundReason(body?.reason === "unpublished" ? "unpublished" : "missing");
                    return;
                }
                const data = await response.json() as BlogDetails;
                setBlog(data);
                setNotFoundReason(undefined);
            } catch (error) {
                Sentry.captureException(new Error("Failed to fetch blog: " + (error instanceof Error ? error.message : String(error))));
                setNotFoundReason("missing");
            } finally {
                setIsLoading(false);
            }
        };
        fetchBlog();
    }, [blogId]);

    if (!blog || !blog.content_html) {
        const isUnpublished = notFoundReason === "unpublished";
        return {
            title: isUnpublished ? "この記事は公開されていません" : "記事が見つかりません",
            content: isUnpublished ? "この記事は公開されていません。" : "指定された記事は見つかりませんでした。",
            sidebar: <SidebarContainer />,
            isLoading,
        };
    }

    return {
        title: blog.title || "No Title",
        content: <MarkdownHtml htmlBody={blog.content_html} />,
        sidebar: <SidebarContainer />,
        isLoading,
    }
}

export default useGenerateProps;
