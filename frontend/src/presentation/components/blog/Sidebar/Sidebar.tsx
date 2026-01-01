
import { Box, Divider, Stack, Typography } from "@mui/material";
import HomeOutlinedIcon from "@mui/icons-material/HomeOutlined";
import GitHubIcon from "@mui/icons-material/GitHub";
import XIcon from "@mui/icons-material/X";
import ToggleButton from "../../../primitive/ToggleButton/ToggleButton";

type SidebarProps = {
    years: string[],
    additionalYears? : string[],
};

function Sidebar(props: SidebarProps) {
    const { years, additionalYears } = props;

    return (
        <Stack spacing={2}>
            <Stack direction="row" spacing={1}>
                <HomeOutlinedIcon />
                <GitHubIcon />
                <XIcon />
            </Stack>
        
            <Divider />
        
            <Stack component="ul" spacing={1} sx={{ m: 0, p: 0 }}>
                {years.map((year) => (
                    <Box component="li" key={year} sx={{ listStyle: "none" }}>
                        <Typography variant="body1">{year}</Typography>
                    </Box>
                ))}
                {additionalYears && 
                    <ToggleButton additionalContents={additionalYears} />
                }
            </Stack>
        </Stack>
    )
};

export default Sidebar;