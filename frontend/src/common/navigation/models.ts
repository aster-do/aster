export interface NavigationRoute {
  name: string;
  path: string;
  element: React.ReactNode;
  subRoutes?: NavigationRoute[];
}