import MazeCreatorPage from "./maze-creator/container/MazeCreatorPage";
import MazePlayContainer from "./maze-creator/container/page/play/Container";
import HomePageContainer from "./home/container/page/Container";
import { BlogTopContainer } from "./blog/container/page/blogs/Container";
import BlogContainer from "./blog/container/page/blog_id/Container";
import AdminHomeContainer from "./admin/container/page/Container";
import EditBlogContainer from "./admin/container/page/blogs_id_edit/Container";
import { RequireAuth } from "./auth/RequireAuth";
import TrajectryPageContainer from "./trajectry/container/page/Container";
import 'highlight.js/styles/github.css'; 

import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import "./App.css";

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<HomePageContainer />} />
        <Route path="/blogs" >
          <Route index element={<BlogTopContainer />} />
          <Route path=":blogId" element={<BlogContainer />} />
        </Route>
        <Route path="/maze" element={<MazeCreatorPage />} />
        <Route path="/maze/play" element={<MazePlayContainer />} />
        <Route path="/trajectry" element={<TrajectryPageContainer />} />

        <Route element={<RequireAuth />}>
          <Route path="/admin">
            <Route index element={<AdminHomeContainer />} />
            <Route path="blogs/:blogId/edit"  element={<EditBlogContainer />}/>
          </Route>
        </Route>
      </Routes>
    </Router>
  );
}

export default App;
