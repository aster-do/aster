import React, { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAccountContext } from '../../common/contexts/AccountContext';
import { AccountRole } from '../../common/models/account';
import LoadingWrapper from '../../common/components/LoadingWrapper';

export default function DashboardHome() {
  const { account } = useAccountContext();
  const navigate = useNavigate();

  useEffect(() => {
    switch (account.role) {
      case AccountRole.OWNER:
        navigate('/dashboard/earnings');
        break;
      case AccountRole.USER:
        navigate('/dashboard/costs');
        break;
      default:
        navigate('/');
    }
  }, [account.role, navigate]);

  return <LoadingWrapper loading />;
}
