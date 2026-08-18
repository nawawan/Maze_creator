import { Box, CircularProgress } from "@mui/material";
import { Blog } from "../../../presentation/page/blog_id/Blog";
import useGenerateProps from "./useGenerateProps";
import type { BlogDetails, BlogNotFoundReason } from "../../../../shared/types/blog";

type Props = {
    initialBlog?: BlogDetails;
    notFoundReason?: BlogNotFoundReason;
};

const BlogContainer = (props: Props) => {
    const { isLoading, ...generatedProps } = useGenerateProps(props.initialBlog, props.notFoundReason);
    if (isLoading) {
        return (
            <Box sx={{ display: "flex", justifyContent: "center", p: 4 }}>
                <CircularProgress />
            </Box>
        );
    }
    return <Blog {...generatedProps} />;
}

export default BlogContainer;