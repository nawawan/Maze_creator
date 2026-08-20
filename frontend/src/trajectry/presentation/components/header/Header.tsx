import { Button, Stack, Typography } from "@mui/material";
import { styled } from "@mui/material/styles";

type HeaderProps = {
  onUpload: () => void;
};

const HeaderRoot = styled(Stack)({
  background: "var(--paper)",
  borderBottom: "1px solid var(--rule)",
  flexShrink: 0,
  height: 56,
  minWidth: 0,
  padding: "0 20px",
});

const Logo = styled(Typography)({
  flexShrink: 0,
  fontSize: 20,
  fontWeight: 700,
  letterSpacing: -0.2,
  whiteSpace: "nowrap",
});

const Spacer = styled(Stack)({
  flex: 1,
  minWidth: 0,
});

const UploadButton = styled(Button)({
  background: "var(--accent)",
  borderRadius: 999,
  color: "#fff",
  cursor: "pointer",
  flexShrink: 0,
  fontSize: 13,
  fontWeight: 700,
  padding: "8px 16px",
  whiteSpace: "nowrap",
});

const Avatar = styled(Typography)({
  alignItems: "center",
  background: "var(--paper)",
  border: "1px solid var(--rule)",
  borderRadius: "50%",
  display: "flex",
  flexShrink: 0,
  fontSize: 15,
  height: 36,
  justifyContent: "center",
  width: 36,
});

export const Header = ({ onUpload }: HeaderProps) => (
  <HeaderRoot alignItems="center" direction="row" spacing={2}>
    <Logo>trajectry</Logo>
    <Spacer />
    <UploadButton disableRipple type="button" onClick={onUpload}>
      + Upload GPX
    </UploadButton>
    {/* <Avatar>あ</Avatar> */}
  </HeaderRoot>
);
