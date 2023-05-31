import React from 'react';
import { Outlet } from 'react-router-dom';
import { createTheme } from '@mui/material/styles';
import { ThemeProvider } from '@emotion/react';
import { Box } from '@mui/material';
import Navbar from './Navbar';
import themeOptions from './theme';
import { NavigationRoute } from '../navigation/models';

const theme = createTheme(themeOptions);

interface AppLayoutProps {
  routes: NavigationRoute[];
}

export default function AppLayout({ routes }: AppLayoutProps) {
  return (
    <ThemeProvider theme={theme}>
      <Box sx={{ display: 'flex', flexFlow: 'row' }}>
        {/* nav */}
        <Navbar routes={routes} />
        <Box sx={{ flex: 1 }}>
          <Outlet />
        </Box>
        {/* footer */}
      </Box>
    </ThemeProvider>
  );
}
