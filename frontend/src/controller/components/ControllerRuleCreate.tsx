import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { enqueueSnackbar } from 'notistack';
import { Stack, Typography } from '@mui/material';
import { Rule, createRule, defaultRule } from '../model/rule';
import RuleEditor from './RuleEditor';
import LoadingWrapper from '../../common/components/LoadingWrapper';

export default function ControllerRuleCreate() {
  const [rule, setRule] = useState<Rule>(defaultRule);
  const [loading, setLoading] = useState<boolean>(false);
  const navigate = useNavigate();

  const handleSave = (r: Rule) => {
    setLoading(true);
    createRule(r)
      .then(() => {
        enqueueSnackbar('Rule created', { variant: 'success' });
      })
      .catch(() =>
        enqueueSnackbar("Couldn't create rule", { variant: 'error' })
      )
      .finally(() => {
        setLoading(false);
        navigate('/rules/list');
      });
  };

  return (
    <Stack direction="column" spacing={8}>
      <Typography variant="h5" textAlign="center">
        Create Rule
      </Typography>
      <LoadingWrapper loading={loading}>
        <RuleEditor rule={rule} onChange={setRule} onSave={handleSave} />
      </LoadingWrapper>
    </Stack>
  );
}
