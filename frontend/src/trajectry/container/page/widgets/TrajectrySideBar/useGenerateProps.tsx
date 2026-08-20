import { useEffect } from "react";
import * as Sentry from "@sentry/react";
import { toTrajectoryActivity, type ActivityResponse } from "../../../../../shared/types/trajectory";
import { useTrajectryPageState } from "../../state/useTrajectryPageState";


export const useGenerateProps = () => {
    const { addActivities } = useTrajectryPageState();
    useEffect(() => {
        const initializeData = async() => {
        try {
                const res = await fetch('/api/admin/trajectries');
                const activityTrajectries = (await res.json()) as ActivityResponse[];
                addActivities(activityTrajectries.map(toTrajectoryActivity))
            } catch (e) {
                Sentry.captureException(new Error("Failed to fetch activities :" + + (e instanceof Error ? e.message : String(e))));
            }
        }
        initializeData();
    }, []);
};