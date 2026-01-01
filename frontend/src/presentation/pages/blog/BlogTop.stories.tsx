import type { Meta, StoryObj } from '@storybook/react-vite';

import BlogTop  from './BlogTop';
import * as EntryCardStories from "../../components/blog/EntryCards/EntryCard.stories";
import * as SidebarStories from "../../components/blog/Sidebar/Sidebar.stories";
import EntryCard from "../../components/blog/EntryCards/EntryCard";
import Sidebar from '../../components/blog/Sidebar/Sidebar';

const meta = {
    title : "BlogTop",
    component: BlogTop,
} satisfies Meta<typeof BlogTop>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
    args:{
        blogEntries: <EntryCard {...EntryCardStories.Default.args} />,
        sidebar: <Sidebar {...SidebarStories.Default.args} />,
    }
}