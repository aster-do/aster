import React from 'react';
import { Outlet, useLocation } from 'react-router-dom';
import { createTheme } from '@mui/material/styles';
import { ThemeProvider } from '@emotion/react';
import { Box, Typography } from '@mui/material';
import Navbar from './Navbar';
import themeOptions from './theme';
import { NavigationRoute } from '../navigation/models';
import { getRouteTitle } from '../navigation';

const theme = createTheme(themeOptions);

interface AppLayoutProps {
  routes: NavigationRoute[];
}

export default function AppLayout({ routes }: AppLayoutProps) {
  const { pathname } = useLocation();

  const title = getRouteTitle(pathname);

  return (
    <ThemeProvider theme={theme}>
      <Box sx={{ display: 'flex', flexFlow: 'row' }}>
        <Navbar routes={routes} />
        <Box
          sx={{ flex: 1, backgroundColor: theme.palette.secondary.light, p: 4 }}
        >
          <Typography variant="h4" sx={{ mb: 2 }}>
            {title}
          </Typography>
          <Outlet />
        </Box>
      </Box>
    </ThemeProvider>
  );
}
