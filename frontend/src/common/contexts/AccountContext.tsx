import React, { createContext, useContext, useMemo, useState } from 'react';
import { Account, AccountRole, defaultAccount } from '../models/account';

interface AccountContextType {
  account: Account;
  setRole: (role: AccountRole) => void;
}

const defaultAccountContext: AccountContextType = {
  account: defaultAccount,
  setRole: () => {},
};

const AccountContext = createContext<AccountContextType>(defaultAccountContext);

export function AccountContextProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const [account, setAccount] = useState<Account>(defaultAccount);
  const accountContext: AccountContextType = useMemo(
    () => ({
      account,
      setRole: (role: AccountRole) => {
        setAccount({ ...account, role });
      },
    }),
    [account, setAccount]
  );

  return (
    <AccountContext.Provider value={accountContext}>
      {children}
    </AccountContext.Provider>
  );
}

export function useAccountContext() {
  return useContext(AccountContext);
}
