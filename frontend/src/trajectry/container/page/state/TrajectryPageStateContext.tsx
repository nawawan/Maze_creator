import { createContext } from "react";
import type { MapStyleKey, TrajectryActivity } from "../../../domain/types";

type TrajectryPageState = {
  activities: TrajectryActivity[];
  activeActivity: TrajectryActivity;
  activeId: string;
  activePhotoId: string | null;
  here: number;
  mapStyle: MapStyleKey;
  uploadOpen: boolean;
  onCloseUpload: () => void;
  onHereChange: (here: number) => void;
  onMapStyleChange: (style: MapStyleKey) => void;
  onOpenUpload: () => void;
  onSelectActivity: (activityId: string) => void;
  onSelectPhoto: (photoId: string) => void;
  addActivities: (activities: TrajectryActivity[]) => void;
};

export const TrajectryPageStateContext = createContext<TrajectryPageState | null>(null);
