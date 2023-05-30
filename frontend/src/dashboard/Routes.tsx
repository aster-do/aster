import React from 'react';
import Hello from './components/hello';
import { NavigationRoute } from '../common/navigation/models';
import Layout from './Layout';

const DashboardRoutes: NavigationRoute = {
  name: 'Dashboard',
  path: 'dashboard',
  element: <Layout />,
  subRoutes: [
    {
      name: 'Hello',
      path: '',
      element: <Hello />,
    },
  ],
};

export default DashboardRoutes;
