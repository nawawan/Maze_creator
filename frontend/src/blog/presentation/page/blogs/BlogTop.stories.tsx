import type { Meta, StoryObj } from '@storybook/react-vite';
import { MemoryRouter } from 'react-router-dom';

import BlogTop  from './BlogTop';
import * as EntryCardStories from "../../EntryCards/EntryCard.stories";
import * as SidebarStories from "../../Sidebar/Sidebar.stories";
import EntryCard from "../../EntryCards/EntryCard";
import Sidebar from '../../Sidebar/Sidebar';

const meta = {
    title : "BlogTop",
    component: BlogTop,
    decorators: [(Story) => <MemoryRouter><Story /></MemoryRouter>],
} satisfies Meta<typeof BlogTop>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
    args:{
        blogEntries: <EntryCard {...EntryCardStories.Default.args} />,
        sidebar: <Sidebar {...SidebarStories.Default.args} />,
    }
}