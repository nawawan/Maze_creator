import { useState, useEffect } from "react";
import { useParams } from "react-router-dom";
import * as Sentry from "@sentry/react";

import type { BlogProps } from "../../../presentation/page/blog_id/Blog";
import SidebarContainer from "../blogs/widgets/Sidebar/Container";
import MarkdownHtml from "../../../presentation/MarkdownHtml/MarkdownHtml";
import { type BlogDetails } from "../../../../shared/types/blog";

const useGenerateProps = (initialBlog?: BlogDetails): BlogProps & { isLoading: boolean } => {
    const { blogId } = useParams<{ blogId: string }>();
    const [blog, setBlog] = useState<BlogDetails | undefined>(() => {
        if (initialBlog) return initialBlog;
        if (typeof window !== "undefined" && window.__BLOG_INITIAL_DATA__?.id === blogId) {
            const seeded = window.__BLOG_INITIAL_DATA__;
            window.__BLOG_INITIAL_DATA__ = undefined;
            return seeded;
        }
        return undefined;
    });
    const [isLoading, setIsLoading] = useState(!blog);

    useEffect(() => {
        if (!blogId) {
            setIsLoading(false);
            return;
        }
        if (blog?.id === blogId) {
            setIsLoading(false);
            return;
        }

        const fetchBlog = async () => {
            try {
                const response = await fetch(`/api/blogs/${blogId}`);
                if (!response.ok) {
                    throw new Error("Failed to fetch blog");
                }
                const data = await response.json() as BlogDetails;
                setBlog(data);
            } catch (error) {
                Sentry.captureException(new Error("Failed to fetch blog: " + (error instanceof Error ? error.message : String(error))));
            } finally {
                setIsLoading(false);
            }
        };
        fetchBlog();
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [blogId]);

    if (!blog || !blog.content_html) {
        return {
            title: "No Content",
            content: "No content available for this blog post.",
            sidebar: <SidebarContainer />,
            isLoading,
        };
    }

    return {
        title: blog?.title || "No Title",
        content: <MarkdownHtml htmlBody={blog?.content_html} />,
        sidebar: <SidebarContainer />,
        isLoading,
    }
}

export default useGenerateProps;
