import React from 'react';
import { AccountRole } from '../models/account';
import Layout from './Layout';
import { NavigationRoute } from './models';
import Redirect from '../components/Redirect';

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
      element: (
        <Redirect
          userRoute="/dashboard"
          ownerRoute="/dashboard"
          managerRoute="/account"
        />
      ),
    },
  ],
};

export default DashboardRoutes;
