import { UploadModal } from "../../../../presentation/components/upload_modal/UploadModal";
import { useGenerateProps } from "./useGenerateProps";

export const UploadModalContainer = () => {
  const props = useGenerateProps();

  return <UploadModal {...props} />;
};
