import React, { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAccountContext } from '../contexts/AccountContext';
import { AccountRole } from '../models/account';
import LoadingWrapper from '../components/LoadingWrapper';

export default function DashboardHome() {
  const { account } = useAccountContext();
  const navigate = useNavigate();

  useEffect(() => {
    switch (account.role) {
      case AccountRole.OWNER:
      case AccountRole.USER:
        navigate('/dashboard');
        break;
      case AccountRole.MANAGER:
        navigate('/accounts');
        break;
      default:
        navigate('/');
    }
  }, [account.role, navigate]);

  return <LoadingWrapper loading />;
}
