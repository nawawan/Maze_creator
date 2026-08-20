import { useEffect, useMemo, useState } from "react";
import * as Sentry from "@sentry/react";

import { trajectryActivities } from "../../domain/mockData";
import type { MapStyleKey, TrajectryActivity } from "../../domain/types";
import { toTrajectoryActivity, type ActivityResponse } from "../../../shared/types/trajectory";

export const useGenerateTrajectryPageProps = () => {
  const [activeId, setActiveId] = useState(trajectryActivities[0].id);
  const [activities, setActivities] = useState(trajectryActivities);
  const activeActivity = useMemo(
    () => trajectryActivities.find((activity) => activity.id === activeId) ?? trajectryActivities[0],
    [activeId],
  );
  const [here, setHere] = useState(activeActivity.photos[2]?.at ?? activeActivity.photos[0]?.at ?? 0.5);
  const [activePhotoId, setActivePhotoId] = useState<string | null>(
    activeActivity.photos[2]?.id ?? activeActivity.photos[0]?.id ?? null,
  );
  const [mapStyle, setMapStyle] = useState<MapStyleKey>("terrain");
  const [uploadOpen, setUploadOpen] = useState(false);

  const selectActivity = (activityId: string) => {
    const nextActivity = trajectryActivities.find((activity) => activity.id === activityId);
    if (!nextActivity) return;

    setActiveId(activityId);
    const nextPhoto = nextActivity.photos[0] ?? null;
    setHere(nextPhoto?.at ?? 0.5);
    setActivePhotoId(nextPhoto?.id ?? null);
  };

  const selectPhoto = (photoId: string) => {
    const nextPhoto = activeActivity.photos.find((photo) => photo.id === photoId);
    setActivePhotoId(photoId);
    if (nextPhoto) setHere(nextPhoto.at);
  };

  const addActivity = (activity: TrajectryActivity[]) => {
    setActivities((prev) => [...activity, ...prev]);
  };

  return {
    activities: trajectryActivities,
    activeActivity,
    activeId,
    activePhotoId,
    here,
    mapStyle,
    uploadOpen,
    onCloseUpload: () => setUploadOpen(false),
    onHereChange: setHere,
    onMapStyleChange: setMapStyle,
    onOpenUpload: () => setUploadOpen(true),
    onSelectActivity: selectActivity,
    onSelectPhoto: selectPhoto,
    addActivities: addActivity,
  };
};
