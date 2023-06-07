import React from 'react';
import { Button, Stack, Typography } from '@mui/material';
import { Rule } from '../model/rule';
import RuleItem from './RuleIteam';

export interface RuleListProps {
  rules: Rule[];
  onClickEdit: (rule: Rule) => void;
  onClickDelete: (rule: Rule) => void;
  onClickAdd: () => void;
}

export default function RuleList({
  rules,
  onClickAdd,
  onClickDelete,
  onClickEdit,
}: RuleListProps) {
  return rules.length > 0 ? (
    <Stack
      direction="column"
      spacing={2}
      alignItems="stretch"
      sx={{ width: '100%' }}
    >
      {rules.map((rule) => (
        <RuleItem
          key={rule.id}
          rule={rule}
          onEdit={onClickEdit}
          onDelete={onClickDelete}
        />
      ))}
    </Stack>
  ) : (
    <Stack
      direction="column"
      sx={{ width: '100%' }}
      justifyContent="center"
      spacing={2}
    >
      <Typography variant="h5" textAlign="center">
        No rules found
      </Typography>
      <Button variant="contained" color="primary" onClick={onClickAdd}>
        Add Rule
      </Button>
    </Stack>
  );
}
