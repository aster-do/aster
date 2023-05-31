import React from 'react';
import Layout from './Layout';
import { NavigationRoute } from '../common/navigation/models';
import { AccountRole } from '../common/models/account';

const InvoiceRoutes: NavigationRoute = {
  name: 'Invoices',
  path: 'invoices',
  roles: [AccountRole.USER, AccountRole.OWNER],
  navbarVisible: true,
  element: <Layout />,
  subRoutes: [],
};

export default InvoiceRoutes;
