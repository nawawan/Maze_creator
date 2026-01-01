import * as React from "react";

import { Button, Collapse, Box, Typography, Stack } from "@mui/material";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import ExpandLessIcon from "@mui/icons-material/ExpandLess";

type ToggleButtonProps = {
    displayText?: string
    additionalContents?: string[]
};

const expandedContentId = "additional-contents-collapse";

function ToggleButton(props: ToggleButtonProps) {
    const [isExpanded, setIsExpanded] = React.useState(false);
    const { displayText, additionalContents } = props;

    const handleToggle = () => {
        setIsExpanded(isExpanded => !isExpanded);
    };
    return (
        <Box>
            <Button
                onClick={handleToggle}
                sx = {{ width: '100%'}}
                startIcon={isExpanded ? <ExpandLessIcon /> : <ExpandMoreIcon />}
                aria-expanded={isExpanded}
                aria-controls={expandedContentId}
            >
                {isExpanded ? "閉じる" : (displayText ?? "もっと見る")}
            </Button>
            <Collapse id={expandedContentId} in={isExpanded} timeout="auto" unmountOnExit>
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