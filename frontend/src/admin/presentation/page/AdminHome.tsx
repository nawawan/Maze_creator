import { Box, Button, ButtonBase, Tab, Tabs, Typography } from "@mui/material";
import EditIcon from "@mui/icons-material/Edit";
import { BLOG_TAGS } from "../../../shared/types/blog";

export type Blog = {
  id: string;
  title: string;
  date: string;
  tag?: string;
};

export type AdminHomeProps = {
  posts: Blog[];
  selectedTag: string;
  onWriteClick?: () => void;
  onPostClick?: (id: string, title: string) => void;
  onTagFilterChange?: (tag: string) => void;
};

function AdminHome(props: AdminHomeProps) {
  const { posts, selectedTag, onWriteClick, onPostClick, onTagFilterChange } = props;

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
      <Tabs
        value={selectedTag}
        onChange={(_, value: string) => onTagFilterChange?.(value)}
        sx={{ mb: 3, borderBottom: 1, borderColor: "divider" }}
      >
        <Tab label="一覧" value="" />
        {BLOG_TAGS.map((tag) => (
          <Tab key={tag} label={tag} value={tag} />
        ))}
      </Tabs>
      <Box
        sx={{
          display: "grid",
          gridTemplateColumns: "repeat(4, 1fr)",
          gap: 2,
        }}
      >
        {posts.map((post) => (
          <ButtonBase
            key={post.id}
            onClick={() => onPostClick?.(post.id, post.title)}
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
