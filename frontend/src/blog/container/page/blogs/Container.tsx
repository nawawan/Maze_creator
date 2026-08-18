
import { BlogTop } from "../../../presentation/page/blogs/BlogTop";
import { type BlogResponse } from "../../../../shared/types/blog";

import { useGenerateProps } from "./useGenerateProps";

type Props = {
    initialBlogs?: BlogResponse[];
};

export const BlogTopContainer = (props: Props) => {
    const generatedProps = useGenerateProps(props.initialBlogs);
    return <BlogTop {...generatedProps} />;
}