import { LinearProgress, Paper, Stack, Tab, Tabs, Typography } from "@mui/material";
import { Link } from "react-router-dom";
import { BLOG_TAGS } from "../../../shared/types/blog";

export type EntryCardProps = {
    posts: {
        id: string,
        title: string,
        outline: string | undefined,
        publishedAtLabel: string | undefined,
        tag?: string,
    }[],
    onClick: (id: string) => void,
    selectedTag: string,
    onTagFilterChange?: (tag: string) => void,
    isFetching?: boolean,
};

export const EntryCard = (props: EntryCardProps) => {
    const { posts, selectedTag, onTagFilterChange, isFetching } = props;

    return (
        <Stack spacing={2}>
            <Stack>
                <Tabs
                    value={selectedTag}
                    onChange={(_, value: string) => onTagFilterChange?.(value)}
                    sx={{ borderBottom: 1, borderColor: "divider" }}
                >
                    <Tab label="一覧" value="" />
                    {BLOG_TAGS.map((tag) => (
                        <Tab key={tag} label={tag} value={tag} />
                    ))}
                </Tabs>
                {isFetching && <LinearProgress />}
            </Stack>
            <Stack spacing={2}>
                {posts.map((post) => (
                <Paper
                    key={post.id}
                    component={Link}
                    to={`/blogs/${post.id}`}
                    elevation={0}
                    sx={{
                      display: "block",
                      bgcolor: "grey.300",
                      px: 2,
                      py: 2,
                      color: "inherit",
                      textDecoration: "none",
                    }}
                    >
                    <Typography variant="subtitle1" fontWeight={600}>
                        {post.title}
                    </Typography>
                    <Typography variant="body2" color="text.secondary" sx={{ pl: 2 }}>
                      {post.outline}
                    </Typography>
                    {post.publishedAtLabel && (
                        <Typography variant="body2" color="text.secondary">
                            {post.publishedAtLabel}
                        </Typography>
                    )}
                </Paper>
                ))}
            </Stack>
        </Stack>
    );
}

export default EntryCard;
