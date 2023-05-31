import React from 'react';
import Hello from './components/hello';
import { NavigationRoute } from '../common/navigation/models';
import Layout from './Layout';
import { AccountRole } from '../common/models/account';

const DashboardRoutes: NavigationRoute = {
  name: 'Dashboard',
  path: 'dashboard',
  roles: [AccountRole.USER, AccountRole.OWNER],
  navbarVisible: true,
  element: <Layout />,
  subRoutes: [
    {
      name: 'Earnings',
      path: 'earnings',
      roles: [AccountRole.OWNER],
      navbarVisible: true,
      element: <Hello />,
    },
    {
      name: 'Costs',
      path: 'costs',
      roles: [AccountRole.USER, AccountRole.OWNER],
      navbarVisible: true,
      element: <Hello />,
    },
  ],
};

export default DashboardRoutes;
