import MazeCreatorPage from "./presentation/pages/maze-creator/MazeCreatorPage";

import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import "./App.css";

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<MazeCreatorPage />} />
      </Routes>
    </Router>
  );
}

export default App;
