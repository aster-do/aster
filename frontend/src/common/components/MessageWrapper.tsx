import { Box, Typography } from '@mui/material';
import React from 'react';

export interface MessageWrapperProps {
  children: React.ReactElement;
  message: string;
  show?: boolean;
}

export default function MessageWrapper({
  children,
  message,
  show = false,
}: MessageWrapperProps): React.ReactElement {
  return show ? (
    <Box sx={{ width: '100%', height: '100%', p: 4 }}>
      <Typography variant="h5" align="center">
        {message}
      </Typography>
    </Box>
  ) : (
    children
  );
}
