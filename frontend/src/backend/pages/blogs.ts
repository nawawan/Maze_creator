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

    const blogWithContent: BlogDetails | null = await BlogService.getBlogWithContent(apiUrl, c.env.BLOG_BUCKET, id).catch(() => null);
    if (!blogWithContent) {
        return c.json({ error: 'Not Found', reason: 'missing' }, 404);
    }
    if (blogWithContent.status !== 'PUBLISHED') {
        return c.json({ error: 'Not Found', reason: 'unpublished' }, 404);
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