import { Box } from "@mui/material";
import { AttributionControl, LngLatBounds, Map as MapLibreMap, Marker, NavigationControl } from "maplibre-gl";
import "maplibre-gl/dist/maplibre-gl.css";
import type { StyleSpecification } from "@maplibre/maplibre-gl-style-spec";
import { useEffect, useRef, useState } from "react";
import { pointAt } from "../../../domain/geo";
import type { LngLat, MapStyleKey, TrajectryActivity } from "../../../domain/types";

type MapAreaProps = {
  activity: TrajectryActivity;
  activePhotoId: string | null;
  here: number;
  mapStyle: MapStyleKey;
  onPhotoSelect: (photoId: string) => void;
};

// Free-for-prototyping tile sources. OpenTopoMap/OpenStreetMap raster tiles
// have usage policies that make them unsuitable for real production traffic
// — swap for MapTiler/Stadia/self-hosted tiles before this app sees real load.
const rasterStyle = (tileUrl: string, attribution: string): StyleSpecification => ({
  version: 8,
  sources: {
    base: {
      type: "raster",
      tiles: [tileUrl],
      tileSize: 256,
      attribution,
    },
  },
  layers: [
    { id: "bg", type: "background", paint: { "background-color": "#f4f4f5" } },
    { id: "base", type: "raster", source: "base" },
  ],
});

const MAP_STYLE_SOURCES: Record<MapStyleKey, () => StyleSpecification | string> = {
  terrain: () =>
    rasterStyle("https://a.tile.opentopomap.org/{z}/{x}/{y}.png", "© OpenTopoMap (CC-BY-SA), © OpenStreetMap"),
  streets: () => rasterStyle("https://tile.openstreetmap.org/{z}/{x}/{y}.png", "© OpenStreetMap contributors"),
};

const TRACK_SOURCE_ID = "trajectry-track";

const addTrackLayers = (map: MapLibreMap, activity: TrajectryActivity) => {
  if (map.getLayer("track-glow")) map.removeLayer("track-glow");
  if (map.getLayer("track")) map.removeLayer("track");
  if (map.getSource(TRACK_SOURCE_ID)) map.removeSource(TRACK_SOURCE_ID);

  map.addSource(TRACK_SOURCE_ID, {
    type: "geojson",
    data: {
      type: "Feature",
      geometry: { type: "LineString", coordinates: activity.track },
      properties: {},
    },
  });
  map.addLayer({
    id: "track-glow",
    type: "line",
    source: TRACK_SOURCE_ID,
    paint: { "line-color": "red", "line-width": 10, "line-opacity": 0.18, "line-blur": 3 },
    layout: { "line-cap": "round", "line-join": "round" },
  });
  map.addLayer({
    id: "track",
    type: "line",
    source: TRACK_SOURCE_ID,
    paint: { "line-color": "red", "line-width": 4, "line-opacity": 0.95 },
    layout: { "line-cap": "round", "line-join": "round" },
  });
};

const boundsOf = (track: LngLat[]) =>
  track.reduce((bounds, point) => bounds.extend(point), new LngLatBounds(track[0], track[0]));

export const MapArea = ({ activity, activePhotoId, here, mapStyle, onPhotoSelect }: MapAreaProps) => {
  const containerRef = useRef<HTMLDivElement | null>(null);
  const mapRef = useRef<MapLibreMap | null>(null);
  const [ready, setReady] = useState(false);
  const hereMarkerRef = useRef<Marker | null>(null);
  const startEndMarkersRef = useRef<Marker[]>([]);
  const photoMarkersRef = useRef<{ id: string; element: HTMLDivElement; marker: Marker }[]>([]);
  const onPhotoSelectRef = useRef(onPhotoSelect);
  onPhotoSelectRef.current = onPhotoSelect;
  const mapStyleRef = useRef(mapStyle);
  mapStyleRef.current = mapStyle;

  useEffect(() => {
    if (!containerRef.current || mapRef.current) return;

    const map = new MapLibreMap({
      container: containerRef.current,
      style: MAP_STYLE_SOURCES[mapStyleRef.current](),
      center: activity.center,
      zoom: activity.zoom,
      attributionControl: false,
      pitchWithRotate: false,
      dragRotate: false,
    });
    map.addControl(new AttributionControl({ compact: true }), "bottom-right");
    map.addControl(new NavigationControl({ showCompass: false }), "bottom-right");
    map.on("load", () => setReady(true));
    mapRef.current = map;

    return () => {
      photoMarkersRef.current.forEach(({ marker }) => marker.remove());
      photoMarkersRef.current = [];
      startEndMarkersRef.current.forEach((marker) => marker.remove());
      startEndMarkersRef.current = [];
      hereMarkerRef.current?.remove();
      hereMarkerRef.current = null;
      map.remove();
      mapRef.current = null;
      setReady(false);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    const map = mapRef.current;
    if (!map || !ready) return;

    map.setStyle(MAP_STYLE_SOURCES[mapStyle]());
    map.once("style.load", () => addTrackLayers(map, activity));
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [mapStyle]);

  useEffect(() => {
    const map = mapRef.current;
    if (!map || !ready) return;

    addTrackLayers(map, activity);
    map.fitBounds(boundsOf(activity.track), { padding: 80, duration: 900 });

    startEndMarkersRef.current.forEach((marker) => marker.remove());
    const startEl = document.createElement("div");
    startEl.className = "trajectry-map-area__marker";
    const endEl = document.createElement("div");
    endEl.className = "trajectry-map-area__marker";
    startEndMarkersRef.current = [
      new Marker({ element: startEl }).setLngLat(activity.track[0]).addTo(map),
      new Marker({ element: endEl }).setLngLat(activity.track[activity.track.length - 1]).addTo(map),
    ];

    photoMarkersRef.current.forEach(({ marker }) => marker.remove());
    photoMarkersRef.current = activity.photos.map((photo) => {
      const el = document.createElement("div");
      el.className = "trajectry-map-area__photo-marker";
      el.setAttribute("aria-label", photo.caption);
      el.addEventListener("click", (event) => {
        event.stopPropagation();
        onPhotoSelectRef.current(photo.id);
      });
      const marker = new Marker({ element: el }).setLngLat(pointAt(activity.track, photo.at)).addTo(map);
      return { id: photo.id, element: el, marker };
    });
  }, [activity, ready]);

  useEffect(() => {
    const map = mapRef.current;
    if (!map || !ready) return;

    const lngLat = pointAt(activity.track, here);
    if (!hereMarkerRef.current) {
      const el = document.createElement("div");
      el.className = "trajectry-map-area__here";
      hereMarkerRef.current = new Marker({ element: el }).setLngLat(lngLat).addTo(map);
    } else {
      hereMarkerRef.current.setLngLat(lngLat);
    }
  }, [activity, here, ready]);

  useEffect(() => {
    photoMarkersRef.current.forEach(({ id, element }) => {
      element.classList.toggle("is-active", id === activePhotoId);
    });
  }, [activePhotoId]);

  return <Box className="trajectry-map-area" component="section" aria-label="route map" ref={containerRef} />;
};
