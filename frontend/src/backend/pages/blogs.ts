import { Hono } from 'hono';
import { BlogService } from '../service/BlogService';
import { z } from 'zod';
import { zValidator } from '@hono/zod-validator';


import { type BlogDetails, type BlogResponse } from '../../shared/types/blog';

type Env = {
    API_URL: string;
    BLOG_BUCKET: R2Bucket;
    TEAM_DOMAIN: string;
    AUD: string;
    PUBLIC_URL: string;
};


const blogs = new Hono<{ Bindings: Env }>();

blogs.get('/', async (c) => {
    const apiUrl = c.env.API_URL;
    const status = c.req.query('status');
    const tag = c.req.query('tag');

    const resp: BlogResponse[] = await BlogService.getBlogs(apiUrl, status, tag);
    return c.json(resp);
});

blogs.get('/:id',
    zValidator('param', z.object({ id: z.string() })),
    async (c) => {
    const apiUrl = c.env.API_URL;
    const { id } = c.req.valid('param');

    const blog: BlogResponse = await BlogService.getBlogById(apiUrl, id)
        .catch((e) => {
            throw new Error("Failed to fetch blog by id: " + (e instanceof Error ? e.message : String(e)));
        });
    const content = await BlogService.getBlogContent(c.env.BLOG_BUCKET, blog.content_key)
        .catch((e) => {
            throw new Error("Failed to fetch blog content: " + (e instanceof Error ? e.message : String(e)));
        });

    const blogWithContent: BlogDetails = {
        id: blog.id,
        title: blog.title,
        slug: blog.slug === "" ? undefined : blog.slug,
        content_html: content,
        status: blog.status,
        tag: blog.tag,
    }
    return c.json(blogWithContent);
});

blogs.get('/:id/md', 
    zValidator('param', z.object({ id: z.string() })),
    async (c) => {

    const { id } = c.req.valid('param');
    const markdown = await BlogService.getBlogDraft(c.env.BLOG_BUCKET, id);
    console.log(id + ": " + markdown);

    return c.json(markdown);
});

export default blogs;