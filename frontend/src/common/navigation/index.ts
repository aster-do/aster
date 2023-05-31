import { RouteObject } from 'react-router-dom';
import { NavigationRoute } from './models';
import DashboardRoutes from '../../dashboard/Routes';
import InvoiceRoutes from '../../invoice/Routes';
import MetricsRoutes from '../../metrics/Routes';
import NotifierRoutes from '../../notifier/Routes';

const navigationRoutes: NavigationRoute[] = [
  DashboardRoutes,
  InvoiceRoutes,
  MetricsRoutes,
  NotifierRoutes,
];

export default function getRoutes(): RouteObject[] {
  function getRoutesInternal(navRoutes: NavigationRoute[]): RouteObject[] {
    return navRoutes.map((navRoute) => ({
      path: navRoute.path,
      element: navRoute.element,
      children: getRoutesInternal(navRoute.subRoutes ?? []),
    }));
  }

  return getRoutesInternal(navigationRoutes);
}