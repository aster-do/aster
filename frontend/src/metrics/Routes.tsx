import React from 'react';
import { NavigationRoute } from '../common/navigation/models';
import Layout from './Layout';

const MetricsRoutes: NavigationRoute = {
  name: 'Metrics',
  path: 'metrics',
  element: <Layout />,
  subRoutes: [],
};

export default MetricsRoutes;
