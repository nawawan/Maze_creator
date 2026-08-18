import { type BlogTopProps } from "../../../presentation/page/blogs/BlogTop";
import { type BlogResponse } from "../../../../shared/types/blog";

import EntryCardsContainer from "./widgets/EntryCards/Container";
import SidebarContainer from "./widgets/Sidebar/Container";

export const useGenerateProps = (initialBlogs?: BlogResponse[]) : BlogTopProps => {
    return {
        blogEntries: <EntryCardsContainer initialBlogs={initialBlogs} />,
        sidebar: <SidebarContainer />,
    }
}