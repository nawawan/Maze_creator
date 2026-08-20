import type { Meta, StoryObj } from '@storybook/react-vite';
import { Button } from '@mui/material';
import Snackbar, { type SnackbarSeverity } from './Snackbar';

const SnackbarPreview = ({ message, severity }: { message: string; severity: SnackbarSeverity }) => (
    <>
        <Button onClick={() => Snackbar.Notify({ message, severity })}>表示する</Button>
        <Snackbar />
    </>
);

const meta = {
    title: 'primitive/Snackbar',
    component: SnackbarPreview,
} satisfies Meta<typeof SnackbarPreview>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Success: Story = {
    args: { message: '保存しました', severity: 'success' },
};

export const Error: Story = {
    args: { message: '保存に失敗しました', severity: 'error' },
};
