import { Link, useLocation } from 'react-router-dom';
import {
  List,
  ListItemButton,
  MenuItem,
  Select,
  SelectChangeEvent,
  Stack,
  useTheme,
} from '@mui/material';
import React from 'react';
import { NavigationRoute } from '../navigation/models';
import { useAccountContext } from '../contexts/AccountContext';
import { AccountRole } from '../models/account';

interface NavbarProps {
  routes: NavigationRoute[];
}

export default function Navbar({ routes }: NavbarProps) {
  const { pathname } = useLocation();
  const { palette } = useTheme();
  const { account, setRole } = useAccountContext();

  const handleRoleChange = (event: SelectChangeEvent<AccountRole>) => {
    setRole(event.target.value as AccountRole);
  };

  return (
    <Stack
      direction="column"
      spacing={4}
      sx={{
        height: '100%',
        minHeight: '100vh',
        px: 2,
        py: 3,
        minWidth: 200,
        maxWidth: 220,
      }}
      alignItems="center"
    >
      <Link to="/" style={{ width: '80%' }}>
        <img src="/logo.png" alt="Aster logo" style={{ width: '100%' }} />
      </Link>
      <Stack direction="column" spacing={1} sx={{ width: '100%' }}>
        <Select
          value={account.role}
          onChange={handleRoleChange}
          sx={{ width: '100%', borderRadius: '5px' }}
        >
          <MenuItem value={AccountRole.USER}>User</MenuItem>
          <MenuItem value={AccountRole.OWNER}>Owner</MenuItem>
          <MenuItem value={AccountRole.MANAGER}>Aster Manager</MenuItem>
        </Select>
        <List sx={{ width: '100%' }}>
          {routes
            .filter(
              (route) =>
                route.roles.includes(account.role) && route.navbarVisible
            )
            .map((route) => (
              <Stack direction="column" spacing={0} sx={{ my: 1 }}>
                <Link
                  to={`/${route.path}`}
                  style={{
                    textDecoration: 'none',
                    color: pathname.startsWith(`/${route.path}`)
                      ? palette.primary.contrastText
                      : palette.text.primary,
                  }}
                >
                  <ListItemButton
                    sx={{
                      width: '100%',
                      px: 2,
                      py: 1,
                      borderRadius: '5px',
                      backgroundColor: pathname.startsWith(`/${route.path}`)
                        ? 'primary.light'
                        : 'transparent',
                      ':hover': {
                        backgroundColor: pathname.startsWith(`/${route.path}`)
                          ? 'primary.main'
                          : 'secondary.main',
                      },
                    }}
                  >
                    {route.name}
                  </ListItemButton>
                </Link>
                {route.subRoutes && route.subRoutes?.length > 0 && (
                  <List sx={{ width: '100%', mt: 0, pt: 0 }}>
                    {route.subRoutes
                      ?.filter(
                        (subRoute) =>
                          subRoute.roles.includes(account.role) &&
                          subRoute.navbarVisible
                      )
                      .map((subRoute) => (
                        <Link
                          to={`/${route.path}/${subRoute.path}`}
                          style={{
                            textDecoration: 'none',
                            color: palette.text.primary,
                          }}
                        >
                          <ListItemButton
                            sx={{
                              width: '100%',
                              my: 1,
                              pl: 4,
                              pr: 2,
                              py: 1,
                              borderRadius: '5px',
                            }}
                          >
                            {subRoute.name}
                          </ListItemButton>
                        </Link>
                      ))}
                  </List>
                )}
              </Stack>
            ))}
        </List>
      </Stack>
    </Stack>
  );
}
