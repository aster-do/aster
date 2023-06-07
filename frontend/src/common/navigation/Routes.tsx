import React from 'react';
import { AccountRole } from '../models/account';
import Layout from './Layout';
import { NavigationRoute } from './models';
import AsterHome from './AsterHome';

const DashboardRoutes: NavigationRoute = {
  name: 'AsterHome',
  path: '/',
  roles: [AccountRole.USER, AccountRole.OWNER],
  navbarVisible: false,
  element: <Layout />,
  subRoutes: [
    {
      name: 'Home',
      path: '',
      roles: [AccountRole.USER, AccountRole.OWNER, AccountRole.MANAGER],
      navbarVisible: false,
      element: <AsterHome />,
    },
  ],
};

export default DashboardRoutes;
