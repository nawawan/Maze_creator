import type { CSSProperties } from 'react';

export const FONT_SIZE = 15;
export const LINE_HEIGHT = 1.65;
export const PADDING = 12;
export const FONT_FAMILY = "ui-monospace, 'SFMono-Regular', Menlo, Consolas, 'Liberation Mono', monospace";
// Unitless line-height is a pure multiplier of font-size, so this is the exact
// pixel height of a single visual row in both the textarea/pre and the gutter.
export const ROW_HEIGHT_PX = FONT_SIZE * LINE_HEIGHT;

export const sharedEditorStyle: CSSProperties = {
    fontSize: FONT_SIZE,
    lineHeight: LINE_HEIGHT,
    fontFamily: FONT_FAMILY,
    padding: PADDING,
    margin: 0,
    whiteSpace: 'pre-wrap',
    overflowWrap: 'break-word',
    boxSizing: 'border-box',
    // Reserve the scrollbar's width on all three overlapping layers (measure
    // div, pre, textarea) regardless of whether a scrollbar is actually shown.
    // Without this, the textarea (overflow: auto) narrows once it grows a real
    // scrollbar while the pre/measure layers (overflow: hidden/visible) don't,
    // so long lines wrap at a different column and the caret drifts from the
    // highlighted text underneath it.
    scrollbarGutter: 'stable',
};
