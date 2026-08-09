import MazePlayPage from "../../../presentation/page/play/MazePlayPage";
import { useGenerateProps } from "./useGenerateProps";

export const MazePlayContainer = () => {
  const generatedProps = useGenerateProps();
  return <MazePlayPage {...generatedProps} />;
};

export default MazePlayContainer;
