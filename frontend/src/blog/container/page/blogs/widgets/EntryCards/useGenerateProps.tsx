import { useEffect, useRef, useState } from "react";
import { useNavigate } from "react-router-dom";
import * as Sentry from "@sentry/react";
import { type EntryCardProps } from "../../../../../presentation/EntryCards/EntryCard";
import { type BlogResponse } from "../../../../../../shared/types/blog";
import { formatPublishedDate } from "../../../../../../helper/FormatPublishedDate";

type Posts = EntryCardProps["posts"];

const toPosts = (data: BlogResponse[]): Posts =>
    data.map((blog) => ({
        id: blog.id,
        title: blog.title,
        outline: undefined,
        publishedAtLabel: blog.published_at
            ? formatPublishedDate(blog.published_at)
            : undefined,
        tag: blog.tag,
    }));

export const useGenerateProps = (): EntryCardProps & { isLoading: boolean } => {
    const navigate = useNavigate();
    const [posts, setPosts] = useState<Posts>([]);
    const [selectedTag, setSelectedTag] = useState("");
    const [isLoading, setIsLoading] = useState(true);
    const [isFetching, setIsFetching] = useState(false);

    // Keep the previously rendered posts on screen while a new tag is being
    // fetched, guarding against a superseded request resolving out of order.
    const requestIdRef = useRef(0);

    useEffect(() => {
        const requestId = ++requestIdRef.current;
        setIsFetching(true);

        const fetchData = async () => {
            try {
                const url = selectedTag
                    ? `/api/blogs?status=PUBLISHED&tag=${encodeURIComponent(selectedTag)}`
                    : "/api/blogs?status=PUBLISHED";
                const res = await fetch(url);
                if (!res.ok) throw new Error("Failed to fetch blogs");
                const data = (await res.json()) as BlogResponse[];
                const fetchedPosts = toPosts(data);

                // A newer tag switch superseded this request; ignore its result.
                if (requestId !== requestIdRef.current) return;

                setPosts(fetchedPosts);
            } catch (e) {
                Sentry.captureException(new Error("Failed to fetch blogs: " + (e instanceof Error ? e.message : String(e))));
                // Keep whatever was previously displayed rather than clearing it.
            } finally {
                if (requestId === requestIdRef.current) {
                    setIsLoading(false);
                    setIsFetching(false);
                }
            }
        };
        fetchData();
    }, [selectedTag]);

    return {
        posts,
        onClick: (id: string) => {
            navigate(`/blogs/${id}`);
        },
        selectedTag,
        onTagFilterChange: setSelectedTag,
        isLoading,
        isFetching,
    };
}
