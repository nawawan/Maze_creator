import { useState } from 'react';
import { TextField, type SxProps, type Theme } from '@mui/material';

export type InlineEditableFieldProps = {
    value?: string;
    onChange: (value: string) => void;
    placeholder?: string;
    error?: string;
    startAdornment?: React.ReactNode;
    inputRef?: React.Ref<HTMLInputElement>;
    fontSize?: string;
    fontWeight?: number;
    color?: string;
    width?: number | string;
    align?: 'left' | 'center';
    sx?: SxProps<Theme>;
};

const InlineEditableField = (props: InlineEditableFieldProps) => {
    const [focused, setFocused] = useState(false);
    const align = props.align ?? 'center';

    return (
        <TextField
            inputRef={props.inputRef}
            variant="standard"
            value={props.value}
            onChange={e => props.onChange(e.target.value)}
            onFocus={() => setFocused(true)}
            onBlur={() => setFocused(false)}
            onKeyDown={e => { if (e.key === 'Enter') e.currentTarget.blur(); }}
            placeholder={props.placeholder}
            error={!!props.error}
            helperText={props.error}
            slotProps={{
                input: {
                    disableUnderline: !focused,
                    startAdornment: props.startAdornment,
                    sx: {
                        fontSize: props.fontSize ?? '1rem',
                        fontWeight: props.fontWeight ?? 400,
                        color: props.color,
                        textAlign: align,
                    },
                },
            }}
            sx={{
                width: props.width,
                '& input': { textAlign: align, cursor: 'text' },
                ...props.sx,
            }}
        />
    );
};

export default InlineEditableField;
