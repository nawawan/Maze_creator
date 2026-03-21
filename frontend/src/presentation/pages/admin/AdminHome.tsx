import { Box, Button, Paper, Typography } from "@mui/material";
import EditIcon from "@mui/icons-material/Edit";

type Blog = {
  id: string;
  title: string;
  date: string;
};

type AdminHomeProps = {
  posts: Blog[];
  onWriteClick?: () => void;
};

function AdminHome(props: AdminHomeProps) {
  const { posts, onWriteClick } = props;
  return (
    <Box sx={{ p: 4 }}>
      <Box
        sx={{
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
          mb: 4,
        }}
      >
        <Typography variant="h5" fontWeight={600}>
          ブログ一覧
        </Typography>
        <Button
          variant="outlined"
          startIcon={<EditIcon />}
          onClick={onWriteClick}
        >
          ブログを書く
        </Button>
      </Box>
      <Box
        sx={{
          display: "grid",
          gridTemplateColumns: "repeat(3, 1fr)",
          gap: 2,
        }}
      >
        {posts.map((post) => (
          <Paper
            key={post.id}
            elevation={0}
            sx={{ bgcolor: "grey.300", px: 2, py: 2 }}
          >
            <Typography variant="subtitle1" fontWeight={600}>
              {post.title}
            </Typography>
            <Typography variant="body2" color="text.secondary">
              {post.date}
            </Typography>
          </Paper>
        ))}
      </Box>
    </Box>
  );
}

export default AdminHome;
