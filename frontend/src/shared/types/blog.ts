export const BLOG_TAGS = ["TRAVEL", "RETROSPECTIVE", "DIARY", "TECH"] as const;
export type BlogTagValue = typeof BLOG_TAGS[number];

export type BlogResponse = {
    id: string;
    title: string;
    content_key: string;
    slug: string;
    status: string;
    published_at: string | null;
    tag?: string;
};

export type BlogDetails = {
    id: string;
    title: string;
    slug?: string;
    content_html: string;
    status: string;
    tag?: string;
}