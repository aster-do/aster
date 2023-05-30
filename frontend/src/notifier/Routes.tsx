import React from 'react';
import { NavigationRoute } from '../common/navigation/models';
import Layout from './Layout';

const NotifierRoutes: NavigationRoute = {
  name: 'Notifier',
  path: 'notifier',
  element: <Layout />,
  subRoutes: [],
};

export default NotifierRoutes;
