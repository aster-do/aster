import React from 'react';
import { Stack } from '@mui/material';
import { useNavigate } from 'react-router-dom';
import { Rule, deleteRule, useRules } from '../model/rule';
import ControllerRuleControl from './ControllerRuleControl';
import RuleList from './RuleList';

export default function ControllerRuleList() {
  const { data } = useRules();
  const navigate = useNavigate();

  const handleClickAdd = () => {
    navigate('/rules/create');
  };

  const handleClickEdit = (rule: Rule) => {
    navigate(`/rules/edit/${rule.id}`);
  };

  const handleDelete = (rule: Rule) => {
    deleteRule(rule.id).catch(() => console.error('Unable to delete rule'));
  };

  return (
    <Stack direction="column" sx={{ width: '100%' }}>
      {data && data.length > 0 && (
        <ControllerRuleControl onAdd={handleClickAdd} />
      )}
      <RuleList
        rules={data ?? []}
        onClickAdd={handleClickAdd}
        onClickEdit={handleClickEdit}
        onClickDelete={handleDelete}
      />
    </Stack>
  );
}
