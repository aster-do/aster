import React from 'react';
import { NavigationRoute } from '../common/navigation/models';
import Layout from './Layout';
import { AccountRole } from '../common/models/account';
import Earnings from './components/Earnings';
import Costs from './components/Costs';
import Redirect from '../common/components/Redirect';

const DashboardRoutes: NavigationRoute = {
  name: 'Dashboard',
  path: 'dashboard',
  roles: [AccountRole.USER, AccountRole.OWNER],
  navbarVisible: true,
  element: <Layout />,
  subRoutes: [
    {
      name: 'Home',
      path: '',
      roles: [AccountRole.USER, AccountRole.OWNER],
      navbarVisible: false,
      element: (
        <Redirect
          userRoute="/dashboard/earnings"
          ownerRoute="/dashboard/earnings"
        />
      ),
    },
    {
      name: 'Earnings',
      path: 'earnings',
      roles: [AccountRole.OWNER],
      navbarVisible: true,
      element: <Earnings />,
    },
    {
      name: 'Costs',
      path: 'costs',
      roles: [AccountRole.USER],
      navbarVisible: true,
      element: <Costs />,
    },
  ],
};

export default DashboardRoutes;
