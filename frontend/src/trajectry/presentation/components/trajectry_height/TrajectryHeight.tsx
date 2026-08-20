import { Box, Button, Stack, Typography } from "@mui/material";
import { styled } from "@mui/material/styles";
import { clamp, elevationAt, formatKmAt } from "../../../domain/geo";
import type { TrajectryActivity } from "../../../domain/types";

type TrajectryHeightProps = {
  activity: TrajectryActivity;
  activePhotoId: string | null;
  here: number;
  onHereChange: (here: number) => void;
  onPhotoSelect: (photoId: string) => void;
};

const SVG_WIDTH = 100;
const SVG_HEIGHT = 30;
const PADDING = 2;

const HeightRoot = styled(Stack)({
  background: "var(--paper)",
  borderTop: "1px solid var(--rule)",
  flexShrink: 0,
  padding: "10px 14px 12px",
});

const HeaderRow = styled(Stack)({
  alignItems: "flex-start",
  justifyContent: "space-between",
});

const Label = styled(Typography)({
  color: "var(--ink-soft)",
  fontSize: 12,
});

const StatGroup = styled(Stack)({
  alignItems: "flex-end",
});

const StatCaption = styled(Typography)({
  color: "var(--ink-soft)",
  fontSize: 11,
});

const StatNumber = styled(Typography)({
  color: "var(--ink)",
  fontSize: 13,
  fontWeight: 700,
});

export const TrajectryHeight = ({
  activity,
  activePhotoId,
  here,
  onHereChange,
  onPhotoSelect,
}: TrajectryHeightProps) => {
  const minElevation = Math.min(...activity.elevation);
  const maxElevation = Math.max(...activity.elevation);
  const points = activity.elevation.map((elevation, index) => {
    const x = (index / (activity.elevation.length - 1)) * SVG_WIDTH;
    const y =
      SVG_HEIGHT -
      PADDING -
      ((elevation - minElevation) / (maxElevation - minElevation || 1)) * (SVG_HEIGHT - PADDING * 2);
    return [x, y] as const;
  });
  const linePath = `M ${points.map(([x, y]) => `${x},${y}`).join(" L ")}`;
  const fillPath = `${linePath} L ${SVG_WIDTH},${SVG_HEIGHT} L 0,${SVG_HEIGHT} Z`;
  const hereElevation = Math.round(elevationAt(activity.elevation, here));
  const hereKm = formatKmAt(activity.km, here);

  const scrub = (event: React.PointerEvent<HTMLDivElement>) => {
    const rect = event.currentTarget.getBoundingClientRect();
    onHereChange(clamp((event.clientX - rect.left) / rect.width));
  };

  return (
    <HeightRoot spacing={0.75}>
      <HeaderRow direction="row">
        <Label>ELEVATION PROFILE · drag or hover to scrub</Label>
        <Stack direction="row" spacing={2.5}>
          <StatGroup>
            <StatNumber>{minElevation}m</StatNumber>
          </StatGroup>
          <StatGroup>
            <StatCaption>max</StatCaption>
            <StatNumber>{Math.round(maxElevation)}m</StatNumber>
          </StatGroup>
          <StatGroup>
            <StatNumber>{activity.km}</StatNumber>
            <StatCaption>km</StatCaption>
          </StatGroup>
        </Stack>
      </HeaderRow>
      <Box
        onPointerDown={scrub}
        onPointerMove={scrub}
        sx={{
          cursor: "crosshair",
          height: 92,
          position: "relative",
          userSelect: "none",
          "& svg": { height: "100%", inset: 0, position: "absolute", width: "100%" },
        }}
      >
        <svg viewBox={`0 0 ${SVG_WIDTH} ${SVG_HEIGHT}`} preserveAspectRatio="none">
          <defs>
            <linearGradient id={`elevation-fill-${activity.id}`} x1="0" x2="0" y1="0" y2="1">
              <stop offset="0%" stopColor="red" stopOpacity="0.35" />
              <stop offset="100%" stopColor="red" stopOpacity="0.02" />
            </linearGradient>
          </defs>
          {[0.25, 0.5, 0.75].map((grid) => (
            <line
              key={grid}
              stroke="rgba(0, 0, 0, 0.06)"
              strokeWidth="0.2"
              vectorEffect="non-scaling-stroke"
              x1="0"
              x2={SVG_WIDTH}
              y1={SVG_HEIGHT * grid}
              y2={SVG_HEIGHT * grid}
            />
          ))}
          <path d={fillPath} fill={`url(#elevation-fill-${activity.id})`} />
          <path
            d={linePath}
            fill="none"
            stroke="red"
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth="1.4"
            vectorEffect="non-scaling-stroke"
          />
        </svg>
        {activity.photos.map((photo) => (
          <Button
            aria-label={photo.caption}
            className="trajectry-height__photo"
            disableRipple
            key={photo.id}
            onClick={(event) => {
              event.stopPropagation();
              onPhotoSelect(photo.id);
            }}
            style={{ left: `${photo.at * 100}%` }}
            sx={activePhotoId === photo.id ? { background: "var(--pin)" } : undefined}
            type="button"
          />
        ))}
        <Box
          style={{ left: `${here * 100}%` }}
          sx={{ borderLeft: "1.5px solid var(--pin)", bottom: 0, pointerEvents: "none", position: "absolute", top: 0 }}
        />
        <Typography
          component="div"
          style={{ left: `${here * 100}%` }}
          sx={{
            background: "var(--paper)",
            border: "1px solid var(--rule)",
            borderRadius: "6px",
            color: "var(--ink)",
            fontSize: 11,
            padding: "3px 8px",
            pointerEvents: "none",
            position: "absolute",
            top: -2,
            transform: "translateX(8px)",
            whiteSpace: "nowrap",
          }}
        >
          {hereElevation}m · {hereKm}km
        </Typography>
      </Box>
    </HeightRoot>
  );
};
