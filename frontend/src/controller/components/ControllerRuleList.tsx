import React, { useEffect, useState } from 'react';
import { Stack } from '@mui/material';
import { useNavigate } from 'react-router-dom';
import { enqueueSnackbar } from 'notistack';
import { Rule, deleteRule, getRules } from '../model/rule';
import ControllerRuleControl from './RuleControl';
import RuleList from './RuleList';

export default function ControllerRuleList() {
  const [rules, setRules] = useState<Rule[]>([]);
  const navigate = useNavigate();

  useEffect(() => {
    getRules()
      .then((response) => {
        if (response.status === 200) {
          setRules(response.body ?? []);
        }
      })
      .catch(() =>
        enqueueSnackbar("Couldn't load rules", { variant: 'error' })
      );
  }, []);

  const handleClickAdd = () => {
    navigate('/rules/create');
  };

  const handleClickEdit = (rule: Rule) => {
    navigate(`/rules/edit/${rule.id}`);
  };

  const handleDelete = (rule: Rule) => {
    deleteRule(rule.id)
      .then(() =>
        getRules()
          .then((response) => {
            if (response.status === 200) {
              setRules(response.body ?? []);
            }
          })
          .catch(() =>
            enqueueSnackbar("Couldn't load rules", { variant: 'error' })
          )
      )
      .catch(() =>
        enqueueSnackbar("Couldn't delete rule", { variant: 'error' })
      );
  };

  return (
    <Stack direction="column" sx={{ width: '100%' }} spacing={2}>
      {rules.length > 0 && <ControllerRuleControl onAdd={handleClickAdd} />}
      <RuleList
        rules={rules}
        onClickAdd={handleClickAdd}
        onClickEdit={handleClickEdit}
        onClickDelete={handleDelete}
      />
    </Stack>
  );
}
