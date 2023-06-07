import React, { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import LoadingWrapper from './LoadingWrapper';
import { AccountRole } from '../models/account';
import { useAccountContext } from '../contexts/AccountContext';

export interface RedirectProps {
  userRoute?: string;
  ownerRoute?: string;
  managerRoute?: string;
}

export default function Redirect({
  userRoute,
  ownerRoute,
  managerRoute,
}: RedirectProps) {
  const { account } = useAccountContext();
  const navigate = useNavigate();

  useEffect(() => {
    if (account.role === AccountRole.USER && userRoute) {
      navigate(userRoute);
    } else if (account.role === AccountRole.OWNER && ownerRoute) {
      navigate(ownerRoute);
    } else if (account.role === AccountRole.MANAGER && managerRoute) {
      navigate(managerRoute);
    } else {
      navigate('/');
    }
  }, [account.role, managerRoute, navigate, ownerRoute, userRoute]);

  return <LoadingWrapper loading />;
}
