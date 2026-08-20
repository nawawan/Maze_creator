import { useTrajectryPageState } from "../../state/useTrajectryPageState";
import { type UploadModalProps } from "../../../../presentation/components/upload_modal/UploadModal";
import { type ActivityResponse, toTrajectoryActivity } from "../../../../../shared/types/trajectory";

import * as Sentry from '@sentry/react';
import { useState } from "react";

export const useGenerateProps = (): UploadModalProps => {
    const { onCloseUpload, uploadOpen, addActivities } = useTrajectryPageState();
    const [isLoading, setIsLoading] = useState(false);

    return {
        onClose: onCloseUpload,
        open: uploadOpen,
        isLoading: isLoading,
        onImport: async (file) => {
            setIsLoading(true);
            try {
                const res = await fetch('/api/admin/trajectories', {
                    method: "POST",
                    body: file,
                    headers: { 'Content-Type': file.type }
                });
                const activity = (await res.json()) as ActivityResponse;
                addActivities([toTrajectoryActivity(activity)]);
            } catch (e) {
                Sentry.captureException(new Error("Failed to fetch blogs: " + (e instanceof Error ? e.message : String(e))));
            }
            finally{
                setIsLoading(false);
            }
        }
    }
};