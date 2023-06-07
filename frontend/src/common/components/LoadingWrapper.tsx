import { Box, CircularProgress } from '@mui/material';
import React from 'react';

export interface LoadingWrapperProps {
  children?: React.ReactNode;
  loading?: boolean;
}

export default function LoadingWrapper({
  children,
  loading = false,
}: LoadingWrapperProps) {
  return (
    <Box
      sx={{
        width: '100%',
        height: '100%',
        p: 4,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
      }}
    >
      {loading ? <CircularProgress /> : children}
    </Box>
  );
}
