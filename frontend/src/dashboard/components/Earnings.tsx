import { Stack } from '@mui/material';
import React from 'react';
import BillableChart from './BillableChart';

export default function Earnings() {
  return (
    <Stack direction="column" sx={{ width: '100%' }} spacing={4}>
      <BillableChart title="Earnings" />
    </Stack>
  );
}
