import { AccountRole } from '../models/account';

export interface NavigationRoute {
  name: string;
  path: string;
  element: React.ReactNode;
  roles: AccountRole[];
  navbarVisible?: boolean;
  subRoutes?: NavigationRoute[];
}
