import React from 'react';
import Layout from './Layout';
import { NavigationRoute } from '../common/navigation/models';

const InvoiceRoutes: NavigationRoute = {
  name: 'Invoice',
  path: 'invoice',
  element: <Layout />,
  subRoutes: [],
};

export default InvoiceRoutes;
