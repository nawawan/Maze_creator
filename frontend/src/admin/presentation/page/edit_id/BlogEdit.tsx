import { Box } from '@mui/material';

export type EditBlogProps = {
  Editor: React.ReactNode;
};

const EditBlog = (props: EditBlogProps) => {
  const { Editor } = props;
  return (
    <Box
      sx={{
        display: 'flex',
        flexDirection: 'column',
        position: 'fixed',
        top: 0,
        right: 0,
        bottom: 0,
        left: 0,
      }}
    >
      {Editor}
    </Box>
  );
};

export default EditBlog;
