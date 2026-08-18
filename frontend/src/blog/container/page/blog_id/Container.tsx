import { Box, CircularProgress } from "@mui/material";
import { Blog } from "../../../presentation/page/blog_id/Blog";
import useGenerateProps from "./useGenerateProps";
import type { BlogDetails } from "../../../../shared/types/blog";

type Props = {
    initialBlog?: BlogDetails;
};

const BlogContainer = (props: Props) => {
    const { isLoading, ...generatedProps } = useGenerateProps(props.initialBlog);
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