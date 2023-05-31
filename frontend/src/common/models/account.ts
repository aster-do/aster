export enum AccountRole {
  MANAGER = 'MANAGER',
  OWNER = 'OWNER',
  USER = 'USER',
}

export interface Account {
  role: AccountRole;
}

export const defaultAccount: Account = {
  role: AccountRole.USER,
};
