import { Box, CircularProgress } from '@mui/material';
import React from 'react';

export interface LoadingWrapperProps {
  children?: React.ReactElement;
  loading?: boolean;
}

export default function LoadingWrapper({
  children,
  loading = false,
}: LoadingWrapperProps): React.ReactElement {
  return loading || !children ? (
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
      <CircularProgress />
    </Box>
  ) : (
    children
  );
}
