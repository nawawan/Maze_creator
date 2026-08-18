export const escapeHtml = (text: string): string =>
    text
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;')
        .replace(/'/g, '&#39;');

export const toExcerpt = (html: string, maxLength = 140): string => {
    const text = html
        .replace(/<[^>]*>/g, ' ')
        .replace(/\s+/g, ' ')
        .trim();
    return text.length > maxLength ? `${text.slice(0, maxLength)}...` : text;
};

export const serializeInitialData = (data: unknown): string =>
    JSON.stringify(data).replace(/</g, '\\u003c');
