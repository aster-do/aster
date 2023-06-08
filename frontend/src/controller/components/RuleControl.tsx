import { Button, Stack } from '@mui/material';
import React from 'react';

export interface ControllerRuleListProps {
  onAdd: () => void;
}

export default function ControllerRuleList({ onAdd }: ControllerRuleListProps) {
  const handleAdd = () => {
    onAdd();
  };

  return (
    <Stack direction="row" justifyContent="flex-end" sx={{ width: '100%' }}>
      <Button variant="contained" color="primary" onClick={handleAdd}>
        Add Rule
      </Button>
    </Stack>
  );
}
