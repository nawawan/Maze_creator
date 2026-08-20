import { FONT_FAMILY, FONT_SIZE, LINE_HEIGHT, PADDING, ROW_HEIGHT_PX } from '../constants';

export type LineNumbersProps = {
    scrollRef: React.RefObject<HTMLDivElement | null>;
    lineCount: number;
    // Number of visual rows each logical line occupies once wrapped (index-aligned
    // with the markdown's lines). Falls back to 1 row per line when not yet measured.
    lineWraps?: number[];
};

const LineNumbers = ({ scrollRef, lineCount, lineWraps }: LineNumbersProps) => (
    <div
        ref={scrollRef}
        style={{
            overflowY: 'hidden',
            backgroundColor: '#161b22',
            borderRight: '1px solid #30363d',
            color: '#484f58',
            textAlign: 'right',
            padding: `${PADDING}px 10px`,
            minWidth: '20px',
            fontSize: FONT_SIZE,
            lineHeight: LINE_HEIGHT,
            fontFamily: FONT_FAMILY,
            userSelect: 'none',
            flexShrink: 0,
            boxSizing: 'border-box',
        }}
    >
        {Array.from({ length: lineCount }, (_, i) => (
            <div key={i + 1} style={{ height: (lineWraps?.[i] ?? 1) * ROW_HEIGHT_PX }}>
                {i + 1}
            </div>
        ))}
    </div>
);

export default LineNumbers;
