import { type BlogResponse } from "../../shared/types/blog";

export const BlogService = {
    async getBlogs(apiUrl: string, status?: string, tag?: string) : Promise<BlogResponse[]> {
        const url = new URL(`${apiUrl}/api/blogs`);
        if (status) url.searchParams.set('status', status);
        if (tag) url.searchParams.set('tag', tag);
        const response = await fetch(url.toString());
        if (!response.ok) {
            throw new Error('Failed to fetch blogs');
        }
        const json = await response.json<{ data: { blogs: BlogResponse[] } }>();
        return json.data.blogs;
    },

    async createBlog(apiUrl: string, jwt: string, id: string, content: string, title?: string, slug?: string, status?: string, tag?: string) : Promise<BlogResponse> {
        const response = await fetch(`${apiUrl}/api/blogs`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Cf-Access-Jwt-Assertion': jwt,
            },
            body: JSON.stringify({ id, title, slug, content, status, tag }),
        });
        if (!response.ok) {
            throw new Error('Failed to create blog');
        }
        const json = await response.json<{ blog: BlogResponse  }>();
        return json.blog;
    },

    async getBlogById(apiUrl: string, id: string): Promise<BlogResponse> {
        const response = await fetch(`${apiUrl}/api/blogs/${id}`);
        if (!response.ok) {
            throw new Error('Failed to fetch blog');
        }
        const json = await response.json<{ data: { blog: BlogResponse } }>();
        return json.data.blog;
    },

    async getBlogContent(bucket: R2Bucket, contentKey: string): Promise<string> {
        const object = await bucket.get(contentKey);
        if (!object) {
            throw new Error('Failed to get blog content');
        }
        return await object.text();
    },

    async createBlogId(apiUrl: string, jwt: string) : Promise<string> {
        const response = await fetch(`${apiUrl}/api/blogs/drafts`, {
            method: 'POST',
            headers: {
                'Cf-Access-Jwt-Assertion': jwt,
            }
        });
        if (!response.ok) {
            throw new Error('Failed to fetch blog');
        }
        const id = await response.json<string>();
        return id;
    },

    async getBlogDraft(bucket: R2Bucket, id: string): Promise<string> {
        const object = await bucket.get(`uploads/drafts/${id}.md`);

        if (!object){
            throw new Error('Failed to get blog draft');
        }
        return await object.text();
    },

    async uploadBlogDraft(bucket: R2Bucket, id: string, file: ReadableStream<Uint8Array>): Promise<void> {
        const key = `uploads/drafts/${id}.md`;

        try {
            const response = await bucket.put(key, file);
            if (!response) {
                throw new Error('Failed to upload blog draft');
            }
        } catch (e) {
            throw new Error("Failed to update blog image: " + (e instanceof Error ? e.message : String(e)));
        }
    },

    async updateBlogImage(bucket: R2Bucket, imageFile: ReadableStream<Uint8Array>): Promise<string> {
        const key = `_uploads/${crypto.randomUUID()}`;

        try {
            const response = await bucket.put(key, imageFile);
            if (!response) {
                throw new Error('Failed to upload image');
            }
            return key;
        } catch (e) {
            throw new Error("Failed to update blog image: " + (e instanceof Error ? e.message : String(e)));
        }
    }
}
