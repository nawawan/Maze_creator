import type { ActivityResponse } from "../../shared/types/trajectory";

export const TrajectoryService = {
    async uploadActivity(apiUrl: string, jwt: string, file: ReadableStream<Uint8Array>): Promise<ActivityResponse> {
        const res = await fetch(`${apiUrl}/api/trajectory/upload`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/octet-stream',
                'Cf-Access-Jwt-Assertion': jwt,
            },
            body: file,
        });

        const json = await res.json<{ activity: ActivityResponse }>();
        return json.activity;
    },
    async listActivities(apiUrl: string): Promise<ActivityResponse[]> {
        const res = await fetch(`${apiUrl}/api/trajectry/`);
        const json = await res.json<{activities: ActivityResponse[]}>();
        return json.activities;
    },
};