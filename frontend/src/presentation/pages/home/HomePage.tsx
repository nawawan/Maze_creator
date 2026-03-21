import { Box, Button, ButtonBase, Divider, Typography } from "@mui/material";

type HomePageProps = {
  contents: {
    id: string;
    title: string;
    date: string;
  }[];
  onLoginClick?: () => void;
  onContentClick?: (id: string) => void;
};

function HomePage(props: HomePageProps) {
  const { contents, onLoginClick, onContentClick } = props;
  return (
    <Box sx={{ p: 4 }}>
      <Box
        sx={{
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
          mb: 4,
        }}
      >
        <Typography variant="h5">nawa&apos;s page</Typography>
        <Button variant="outlined" onClick={onLoginClick}>
          ログイン
        </Button>
      </Box>
      <Box sx={{ mb: 2 }}>
        <Typography variant="h6">Contents</Typography>
        <Divider sx={{ mt: 1 }} />
      </Box>
      <Box
        sx={{
          display: "grid",
          gridTemplateColumns: "repeat(2, 1fr)",
          gap: 2,
        }}
      >
        {contents.map((content) => (
          <ButtonBase
            key={content.id}
            onClick={() => onContentClick?.(content.id)}
            sx={{
              display: "block",
              width: "100%",
              textAlign: "left",
              bgcolor: "grey.300",
              px: 2,
              py: 2,
              borderRadius: 1,
            }}
          >
            <Typography variant="subtitle1" fontWeight={600}>
              {content.title}
            </Typography>
            <Typography variant="body2" color="text.secondary">
              {content.date}
            </Typography>
          </ButtonBase>
        ))}
      </Box>
    </Box>
  );
}

export default HomePage;
