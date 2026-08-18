import { Hono } from 'hono';
import { StaticRouter } from 'react-router-dom';
import { renderToString } from 'react-dom/server';
import * as Sentry from '@sentry/cloudflare';
import { BlogTopContainer } from './blog/container/page/blogs/Container';
import BlogContainer from './blog/container/page/blog_id/Container';

import blogs from './backend/pages/blogs';
import admin from './backend/pages/admin';
import { BlogService } from './backend/service/BlogService';
import { escapeHtml, serializeInitialData, toExcerpt } from './backend/util/html';
import type { BlogDetails, BlogResponse } from './shared/types/blog';

type Bindings = {
  ASSETS: {
    fetch: typeof fetch;
  };
  API_URL: string;
  BLOG_BUCKET: R2Bucket;
  DSN: string;
};

const SITE_NAME = "nawa's blog";

const renderBlogPage = (opts: {
  bodyHtml: string;
  title: string;
  description: string;
  canonicalUrl: string;
  ogType: 'website' | 'article';
  initialDataScript: string;
}) => `
      <html lang="ja">
        <head>
          <meta charSet="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1" />
          <title>${escapeHtml(opts.title)}</title>
          <meta name="description" content="${escapeHtml(opts.description)}" />
          <link rel="canonical" href="${escapeHtml(opts.canonicalUrl)}" />
          <meta property="og:site_name" content="${escapeHtml(SITE_NAME)}" />
          <meta property="og:type" content="${opts.ogType}" />
          <meta property="og:title" content="${escapeHtml(opts.title)}" />
          <meta property="og:description" content="${escapeHtml(opts.description)}" />
          <meta property="og:url" content="${escapeHtml(opts.canonicalUrl)}" />
          <meta name="twitter:card" content="summary" />
        </head>
        <body>
          <div id="root">
            ${opts.bodyHtml}
          </div>
          ${opts.initialDataScript}
          <script type="module" src="/assets/blogClient.js"></script>
        </body>
      </html>
      `;

const app = new Hono<{ Bindings: Bindings }>();

app.get('/assets/*', (c) => {
  return c.env.ASSETS.fetch(c.req.raw);
});

app.get('/robots.txt', (c) => {
  const origin = new URL(c.req.url).origin;
  return c.text(`User-agent: *\nAllow: /\nSitemap: ${origin}/sitemap.xml\n`);
});

app.get('/sitemap.xml', async (c) => {
  const origin = new URL(c.req.url).origin;
  const publishedBlogs: BlogResponse[] = await BlogService.getBlogs(c.env.API_URL, 'PUBLISHED').catch(() => []);

  const urls: { loc: string; lastmod?: string }[] = [
    { loc: `${origin}/` },
    { loc: `${origin}/blogs` },
    ...publishedBlogs.map((blog) => ({
      loc: `${origin}/blogs/${blog.id}`,
      lastmod: blog.published_at ?? undefined,
    })),
  ];

  const urlEntries = urls
    .map((entry) => `  <url>\n    <loc>${escapeHtml(entry.loc)}</loc>${entry.lastmod ? `\n    <lastmod>${escapeHtml(entry.lastmod)}</lastmod>` : ''}\n  </url>`)
    .join('\n');

  c.header('Content-Type', 'application/xml');
  return c.body(`<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n${urlEntries}\n</urlset>\n`);
});

app.get('/blogs', async (c) => {
  const origin = new URL(c.req.url).origin;
  const publishedBlogs: BlogResponse[] = await BlogService.getBlogs(c.env.API_URL, 'PUBLISHED').catch(() => []);

  const htmlContent = renderToString(
    <StaticRouter location={c.req.path}>
      <BlogTopContainer initialBlogs={publishedBlogs} />
    </StaticRouter>);

  return c.html(renderBlogPage({
    bodyHtml: htmlContent,
    title: `ブログ一覧 | ${SITE_NAME}`,
    description: "nawaのブログ記事一覧です。技術、旅行、日記などについて書いています。",
    canonicalUrl: `${origin}/blogs`,
    ogType: 'website',
    initialDataScript: `<script>window.__BLOG_LIST_INITIAL_DATA__ = ${serializeInitialData(publishedBlogs)};</script>`,
  }));
})

app.get('/blogs/:id', async (c) => {
  const origin = new URL(c.req.url).origin;
  const id = c.req.param('id');
  const blog: BlogDetails | null = await BlogService.getBlogWithContent(c.env.API_URL, c.env.BLOG_BUCKET, id).catch(() => null);
  const isPublished = blog !== null && blog.status === 'PUBLISHED';

  if (!isPublished) {
    const htmlContent = renderToString(
      <StaticRouter location={c.req.path}>
        <BlogContainer />
      </StaticRouter>);

    return c.html(renderBlogPage({
      bodyHtml: htmlContent,
      title: `この記事は公開されていません | ${SITE_NAME}`,
      description: "この記事は公開されていません。",
      canonicalUrl: `${origin}/blogs/${id}`,
      ogType: 'article',
      initialDataScript: '',
    }), 404);
  }

  const htmlContent = renderToString(
    <StaticRouter location={c.req.path}>
      <BlogContainer initialBlog={blog} />
    </StaticRouter>);

  return c.html(renderBlogPage({
    bodyHtml: htmlContent,
    title: `${blog.title} | ${SITE_NAME}`,
    description: toExcerpt(blog.content_html),
    canonicalUrl: `${origin}/blogs/${id}`,
    ogType: 'article',
    initialDataScript: `<script>window.__BLOG_INITIAL_DATA__ = ${serializeInitialData(blog)};</script>`,
  }));
})

app.route('/api/blogs', blogs);
app.route('/api/admin', admin);

app.get('*', (c) => {
  const url = new URL('/', c.req.url);
  return c.env.ASSETS.fetch(new Request(url.toString()));
});

const appWithSentry = Sentry.withSentry(
  (env: Bindings) => ({
    dsn: env.DSN,
    tracesSampleRate: 1.0,
  }),
  app
);

export default appWithSentry;
