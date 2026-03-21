import { Box, Button, ButtonBase, Typography } from "@mui/material";
import EditIcon from "@mui/icons-material/Edit";

type Blog = {
  id: string;
  title: string;
  date: string;
};

type AdminHomeProps = {
  posts: Blog[];
  onWriteClick?: () => void;
  onPostClick?: (id: string) => void;
};

function AdminHome(props: AdminHomeProps) {
  const { posts, onWriteClick, onPostClick } = props;
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
          <ButtonBase
            key={post.id}
            onClick={() => onPostClick?.(post.id)}
            sx={{
              display: "block",
              width: "100%",
              textAlign: "left",
              bgcolor: "grey.300",
              px: 2,
              py: 2,
              borderRadius: 1,
            }}
          >
            <Typography variant="subtitle1" fontWeight={600}>
              {post.title}
            </Typography>
            <Typography variant="body2" color="text.secondary">
              {post.date}
            </Typography>
          </ButtonBase>
        ))}
      </Box>
    </Box>
  );
}

export default AdminHome;
