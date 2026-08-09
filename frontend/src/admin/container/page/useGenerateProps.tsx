import { useState, useEffect } from 'react';
import * as Sentry from '@sentry/react';
import { type Blog, type AdminHomeProps } from "../../presentation/page/AdminHome";
import { useNavigate } from "react-router-dom";
import type { BlogResponse } from '../../../shared/types/blog';

const toBlog = (blog: BlogResponse): Blog => ({
    id: blog.id,
    title: blog.title,
    date: '2024-01-01',
    tag: blog.tag,
});

export const useGenerateProps = (): AdminHomeProps & { isLoading: boolean } => {
    const [blogs, setBlogs] = useState<Blog[]>([]);
    const [selectedTag, setSelectedTag] = useState('');
    const [isLoading, setIsLoading] = useState(true);
    const navigate = useNavigate();

    useEffect(() => {
        const fetchData = async () => {
            setIsLoading(true);
            try {
                const url = selectedTag ? `/api/blogs?tag=${encodeURIComponent(selectedTag)}` : "/api/blogs";
                const res = await fetch(url);
                if (!res.ok) throw new Error("Failed to fetch blogs");
                const data = (await res.json()) as BlogResponse[];
                setBlogs(data.map(toBlog));
            } catch (e) {
                Sentry.captureException(new Error("Failed to fetch blogs: " + (e instanceof Error ? e.message : String(e))));
                setBlogs([]);
            } finally {
                setIsLoading(false);
            }
        };
        fetchData();
    }, [selectedTag]);

    const onWriteClick = async () => {
        const res = await fetch("/api/admin/blogs/drafts", {
            method: 'POST',
        });
        const id: string = await res.json();
        navigate(`/admin/blogs/${id}/edit`);
    };

    return {
        posts: blogs,
        selectedTag,
        onWriteClick: onWriteClick,
        onPostClick: (id: string, title: string) => {
            navigate(`/admin/blogs/${id}/edit?title=${title}`);
        },
        onTagFilterChange: setSelectedTag,
        isLoading,
    }
};

export default useGenerateProps;
