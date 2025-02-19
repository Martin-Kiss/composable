import { InfoOutlined } from "@mui/icons-material";
import {
  Box,
  Tooltip,
  Typography,
  TypographyProps,
  useTheme,
} from "@mui/material";
import { FC } from "react";

export const TextWithTooltip: FC<{
  tooltip: string;
  TypographyProps?: TypographyProps;
  disabled?: boolean;
}> = ({ tooltip, children, TypographyProps, disabled }) => {
  const theme = useTheme();
  return (
    <Box sx={{ display: "flex", gap: 1 }}>
      <Typography
        {...{
          variant: "body2",
          ...TypographyProps,
        }}
      >
        {children}
      </Typography>
      <Tooltip title={tooltip} arrow disableHoverListener={disabled}>
        <InfoOutlined
          sx={{
            color: theme.palette.primary.light,
            "&:hover": {
              color: theme.palette.secondary.main,
            },
          }}
        />
      </Tooltip>
    </Box>
  );
};
