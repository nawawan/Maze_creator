import { useEffect, useState } from 'react';
import { Alert, Snackbar as MuiSnackbar } from '@mui/material';

export type SnackbarSeverity = 'success' | 'error' | 'info' | 'warning';

export type NotifyOptions = {
    message: string;
    severity?: SnackbarSeverity;
    autoHideDuration?: number;
};

type SnackbarState = {
    open: boolean;
    message: string;
    severity: SnackbarSeverity;
    autoHideDuration: number;
};

const DEFAULT_AUTO_HIDE_DURATION = 4000;

const initialState: SnackbarState = {
    open: false,
    message: '',
    severity: 'success',
    autoHideDuration: DEFAULT_AUTO_HIDE_DURATION,
};

let listener: ((state: SnackbarState) => void) | null = null;

const notify = ({ message, severity = 'success', autoHideDuration = DEFAULT_AUTO_HIDE_DURATION }: NotifyOptions) => {
    listener?.({ open: true, message, severity, autoHideDuration });
};

const Snackbar = () => {
    const [state, setState] = useState<SnackbarState>(initialState);

    useEffect(() => {
        listener = setState;
        return () => {
            listener = null;
        };
    }, []);

    const handleClose = () => setState(prev => ({ ...prev, open: false }));

    return (
        <MuiSnackbar
            open={state.open}
            autoHideDuration={state.autoHideDuration}
            onClose={handleClose}
            anchorOrigin={{ vertical: 'bottom', horizontal: 'center' }}
        >
            <Alert onClose={handleClose} severity={state.severity} variant="filled" sx={{ width: '100%' }}>
                {state.message}
            </Alert>
        </MuiSnackbar>
    );
};

Snackbar.Notify = notify;

export default Snackbar;
