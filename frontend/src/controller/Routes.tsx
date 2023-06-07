import React from 'react';
import { NavigationRoute } from '../common/navigation/models';
import Layout from './Layout';
import { AccountRole } from '../common/models/account';
import Redirect from '../common/components/Redirect';
import ControllerRuleList from './components/ControllerRuleList';
import ControllerRuleCreate from './components/ControllerRuleCreate';
import ControllerRuleEdit from './components/ControllerRuleEdit';

const ControllerRoutes: NavigationRoute = {
  name: 'Rules',
  path: 'rules',
  roles: [AccountRole.OWNER],
  navbarVisible: true,
  element: <Layout />,
  subRoutes: [
    {
      name: 'Home',
      path: '',
      roles: [AccountRole.OWNER],
      navbarVisible: false,
      element: <Redirect ownerRoute="/rules/list" />,
    },
    {
      name: 'List',
      path: 'list',
      roles: [AccountRole.OWNER],
      navbarVisible: true,
      element: <ControllerRuleList />,
    },
    {
      name: 'Create',
      path: 'create',
      roles: [AccountRole.OWNER],
      navbarVisible: true,
      element: <ControllerRuleCreate />,
    },
    {
      name: 'Edit',
      path: 'edit',
      roles: [AccountRole.OWNER],
      navbarVisible: false,
      element: <Redirect ownerRoute="/rules/create" />,
    },
    {
      name: 'Edit',
      path: 'edit/:id',
      roles: [AccountRole.OWNER],
      navbarVisible: false,
      element: <ControllerRuleEdit />,
    },
  ],
};

export default ControllerRoutes;
