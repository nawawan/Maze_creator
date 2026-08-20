import { Box, CircularProgress } from "@mui/material";
import EntryCard from "../../../../../presentation/EntryCards/EntryCard";
import { useGenerateProps } from "./useGenerateProps";

export const EntryCardsContainer = () => {
    const { isLoading, ...generatedProps } = useGenerateProps();
    // Only the very first load (nothing to show yet) gets the full-screen
    // spinner. Later tag switches keep EntryCard mounted and show an inline
    // indicator via isFetching instead, so the tabs and list never disappear.
    if (isLoading) {
        return (
            <Box sx={{ display: "flex", justifyContent: "center", p: 4 }}>
                <CircularProgress />
            </Box>
        );
    }
    return <EntryCard {...generatedProps} />;
}

export default EntryCardsContainer;