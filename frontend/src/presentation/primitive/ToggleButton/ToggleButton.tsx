import * as React from "react";

import { Button, Collapse, Box, Typography, Stack } from "@mui/material";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import ExpandLessIcon from "@mui/icons-material/ExpandLess";

type ToggleButtonProps = {
    displayText?: string
    additionalContents?: string[]
};

function ToggleButton(props: ToggleButtonProps) {
    const [isExpanded, setIsExpanded] = React.useState(false);
    const { displayText, additionalContents } = props;

    const handleToggle = () => {
        setIsExpanded(!isExpanded);
    };
    return (
        <Box>
            <Button
                onClick={handleToggle}
                sx = {{ width: '100%'}}
                startIcon={isExpanded ? <ExpandLessIcon /> : <ExpandMoreIcon />}
            >
                {isExpanded ? "閉じる" : (displayText ?? "もっと見る")}
            </Button>
            <Collapse in={isExpanded} timeout="auto" unmountOnExit>
                <Stack component="ul" spacing={1} sx={{ m: 0, p: 0 }}>
                {additionalContents?.map((content) => (
                    <Box component="li" key={content} sx={{ listStyle: "none" }}>
                        <Typography variant="body1">{content}</Typography>
                    </Box>
                ))}
                </Stack>
            </Collapse>
        </Box>
    )
}

export default ToggleButton;