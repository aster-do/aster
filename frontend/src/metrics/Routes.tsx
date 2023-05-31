import React from 'react';
import { NavigationRoute } from '../common/navigation/models';
import Layout from './Layout';
import { AccountRole } from '../common/models/account';

const MetricsRoutes: NavigationRoute = {
  name: 'Metrics',
  path: 'metrics',
  roles: [AccountRole.OWNER],
  navbarVisible: true,
  element: <Layout />,
  subRoutes: [],
};

export default MetricsRoutes;
