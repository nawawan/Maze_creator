export type ActivityType = "bike" | "run";

export type LngLat = [number, number];

export type TrajectryPhoto = {
  id: string;
  at: number;
  caption: string;
  stamp: string;
};

export type TrajectryActivity = {
  id: string;
  title: string;
  subtitle: string;
  type: ActivityType;
  date: string;
  km: number;
  gain: number;
  duration: string;
  note: string;
  center: LngLat;
  zoom: number;
  track: LngLat[];
  elevation: number[];
  photos: TrajectryPhoto[];
};

export type MapStyleKey = "terrain" | "streets";
