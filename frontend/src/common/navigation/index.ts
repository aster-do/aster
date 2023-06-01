import { RouteObject } from 'react-router-dom';
import { NavigationRoute } from './models';
import DashboardRoutes from '../../dashboard/Routes';
import InvoiceRoutes from '../../invoice/Routes';
import MetricsRoutes from '../../metrics/Routes';
import NotifierRoutes from '../../notifier/Routes';

export const navigationRoutes: NavigationRoute[] = [
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

export function getRouteTitle(path: string): string {
  const explodedPath = path.split('/').slice(1);

  function getRouteTitleInternal(
    navRoutes: NavigationRoute[],
    pathRemaining: string[],
    currentTitles: string[]
  ): string[] {
    if (navRoutes.length === 0) {
      return [];
    }

    const currentNavRoute = navRoutes.find(
      (navRoute) => navRoute.path === pathRemaining[0]
    );

    if (!currentNavRoute) {
      return [];
    }

    return currentTitles
      .concat([currentNavRoute.name])
      .concat(
        getRouteTitleInternal(
          currentNavRoute.subRoutes ?? [],
          pathRemaining.slice(1),
          currentTitles
        )
      );
  }

  return getRouteTitleInternal(navigationRoutes, explodedPath, []).join(' - ');
}
