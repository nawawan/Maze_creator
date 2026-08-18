import type { BlogDetails, BlogResponse } from "./blog";

declare global {
    interface Window {
        __BLOG_INITIAL_DATA__?: BlogDetails;
        __BLOG_LIST_INITIAL_DATA__?: BlogResponse[];
    }
}

export {};
