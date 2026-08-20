import { type TrajectryActivity } from "../../trajectry/domain/types";

export type LineString = {
  type: "LineString";
  coordinates: [number, number][];
}


export type ActivityResponse = {
    id: string;
    name: string;
    distance: number;
    duration: number;
    elevation_gain: number;
    start_time: string;
    trajectory:  LineString | null;
};

export const toTrajectoryActivity = (activity: ActivityResponse): TrajectryActivity => ({
    id: activity.id,
    title: activity.name,
    type: "bike",
    subtitle: "",
    date: activity.start_time,
    km: activity.distance,
    gain: activity.elevation_gain,
    duration: activity.duration.toString(),
    note: "",
    center: activity.trajectory?.coordinates[0] ?? [1, 1],
    zoom: 1,
    track: activity.trajectory?.coordinates ?? [],
    elevation: [],
    photos: [],
});