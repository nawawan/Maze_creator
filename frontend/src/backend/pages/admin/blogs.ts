import { Hono } from 'hono';
import { BlogService } from '../../service/BlogService';
import { z } from 'zod';
import { zValidator } from '@hono/zod-validator';
import * as Sentry from '@sentry/cloudflare';
import { accessAuth } from '../../middleware/auth';

import { type BlogResponse } from '../../../shared/types/blog';

const JWT_HEADER = "Cf-Access-Jwt-Assertion";

type Env = {
    API_URL: string;
    BLOG_BUCKET: R2Bucket;
    TEAM_DOMAIN: string;
    AUD: string;
    PUBLIC_URL: string;
};

const blogs = new Hono<{ Bindings: Env }>();

blogs.post('/', async (c) => {
    const apiUrl = c.env.API_URL;
    const { id, title, content, slug, status, tag } = await c.req.json();
    const jwt = c.req.header(JWT_HEADER) ?? "";

    const resp: BlogResponse = await BlogService.createBlog(apiUrl, jwt, id, content, title, slug, status, tag);
    return c.json(resp);
});

blogs.post('/drafts', async (c)  => {
    const apiUrl = c.env.API_URL;
    const jwt = c.req.header(JWT_HEADER) ?? "";

    const resp: string = await BlogService.createBlogId(apiUrl, jwt);
    return c.json(resp);
});

blogs.post('/:id/md', 
    zValidator('param', z.object({id: z.string()})),
    accessAuth, 
    async(c) => {
    
    if(!c.req.raw.body) {
        return c.notFound();
    }
    const { id } = c.req.valid('param');
    await BlogService.uploadBlogDraft(c.env.BLOG_BUCKET, id, c.req.raw.body);

    return c.json({ status: 'successs' });
});

blogs.put('/images', accessAuth, async (c)  => {
    if (!c.req.raw.body) {
        return c.json({ error: 'No image file provided' }, 400);
    }

    const uploadedImageKey = await BlogService.updateBlogImage(c.env.BLOG_BUCKET, c.req.raw.body)
        .catch((e) => {
            Sentry.captureException("Failed to update blog image: " + (e instanceof Error ? e.message : String(e)));
            throw new Error("Failed to update blog image: " + (e instanceof Error ? e.message : String(e)));
        });

    return c.json(`${c.env.PUBLIC_URL}/${uploadedImageKey}`);
});

export default blogs;