import { Box, Button, ImageList, ImageListItem, Paper, Stack, Typography } from "@mui/material";
import { styled } from "@mui/material/styles";
import { elevationAt, formatKmAt, photoBackground } from "../../../domain/geo";
import type { TrajectryActivity } from "../../../domain/types";

type ImagePanelProps = {
  activity: TrajectryActivity;
  activePhotoId: string | null;
  here: number;
  onPhotoSelect: (photoId: string) => void;
};

const PanelRoot = styled(Stack)({
  flex: 1,
  overflowY: "auto",
  padding: 20,
});

const Note = styled(Paper)({
  background: "var(--paper-2)",
  borderLeft: "3px solid var(--ink-faint)",
  color: "var(--ink-soft)",
  fontSize: 13,
  lineHeight: 1.6,
  padding: "10px 14px",
});

const StatGrid = styled(Box)({
  display: "grid",
  gap: 10,
  gridTemplateColumns: "1fr 1fr",
});

const StatTile = styled(Paper)({
  background: "var(--paper-2)",
  borderRadius: 12,
  padding: "10px 12px",
});

const StatLabel = styled(Typography)({
  color: "var(--ink-soft)",
  fontSize: 11,
  letterSpacing: 0.3,
  textTransform: "uppercase",
});

const StatValue = styled(Typography)({
  color: "var(--ink)",
  fontSize: 22,
  fontWeight: 700,
  marginTop: 2,
});

const GridTitle = styled(Stack)({
  color: "var(--ink)",
  fontSize: 13,
  fontWeight: 700,
  letterSpacing: 0.3,
  marginBottom: 10,
  textTransform: "uppercase",
});

const PhotoList = styled(ImageList)({
  margin: 0,
});

const PhotoButton = styled(Button)({
  aspectRatio: "1",
  borderRadius: 10,
  cursor: "pointer",
  overflow: "hidden",
  padding: 0,
  position: "relative",
  "& i": {
    background: "var(--pin)",
    border: "2px solid var(--paper)",
    borderRadius: "50%",
    height: 14,
    position: "absolute",
    right: 8,
    top: 8,
    width: 14,
    zIndex: 2,
  },
  "& span": {
    background: "rgba(26, 26, 26, 0.65)",
    borderRadius: 6,
    bottom: 8,
    color: "#fff",
    fontSize: 12,
    fontWeight: 700,
    left: 8,
    padding: "3px 8px",
    position: "absolute",
  },
});

const AddButton = styled(Button)({
  aspectRatio: "1",
  background: "transparent",
  border: "1.5px dashed var(--ink-faint)",
  borderRadius: 10,
  color: "var(--ink-soft)",
  cursor: "pointer",
  fontSize: 20,
  padding: 0,
});

const Stat = ({ label, value }: { label: string; value: string }) => (
  <StatTile elevation={0}>
    <StatLabel>{label}</StatLabel>
    <StatValue>{value}</StatValue>
  </StatTile>
);

export const ImagePanel = ({ activity, activePhotoId, here, onPhotoSelect }: ImagePanelProps) => {
  const hereElevation = Math.round(elevationAt(activity.elevation, here));
  const hereKm = formatKmAt(activity.km, here);

  return (
    <PanelRoot spacing={2.5}>
      <StatGrid>
        <Stat label="elev here" value={`${hereElevation}m`} />
        <Stat label="at" value={`${hereKm} / ${activity.km}km`} />
      </StatGrid>
      {activity.photos.length > 0 && (
        <Box>
          <GridTitle direction="row" justifyContent="space-between">
            <Typography component="span">PHOTOS</Typography>
            <Typography component="span" sx={{ color: "var(--ink-soft)", fontWeight: 400 }}>
              {activity.photos.length}
            </Typography>
          </GridTitle>
          <PhotoList cols={2} gap={12}>
            {activity.photos.map((photo) => (
              <ImageListItem key={photo.id}>
                <PhotoButton
                  aria-label={photo.caption}
                  disableRipple
                  onClick={() => onPhotoSelect(photo.id)}
                  style={{ background: photoBackground("red") }}
                  sx={{ border: photo.id === activePhotoId ? "2px solid var(--pin)" : "1px solid var(--rule)" }}
                  type="button"
                >
                  {photo.id === activePhotoId && <Box aria-hidden="true" component="i" />}
                  <Typography component="span">{photo.stamp}</Typography>
                </PhotoButton>
              </ImageListItem>
            ))}
            <ImageListItem>
              <AddButton disableRipple type="button">
                +
              </AddButton>
            </ImageListItem>
          </PhotoList>
        </Box>
      )}
      {activity.note && activity.note !== "—" && <Note elevation={0}>"{activity.note}"</Note>}
    </PanelRoot>
  );
};
