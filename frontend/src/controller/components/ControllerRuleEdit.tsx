import React, { useEffect, useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { Stack, Typography } from '@mui/material';
import { Rule, createRule, defaultRule, getRule } from '../model/rule';
import RuleEditor from './RuleEditor';
import LoadingWrapper from '../../common/components/LoadingWrapper';

export default function ControllerRuleEdit() {
  const { id } = useParams();
  const [rule, setRule] = useState<Rule>(defaultRule);
  const [loading, setLoading] = useState<boolean>(false);
  const navigate = useNavigate();

  useEffect(() => {
    const ruleId = Number(id);

    if (!Number.isNaN(ruleId)) {
      setLoading(true);
      getRule(ruleId)
        .then((response) => {
          setRule(response.body ?? defaultRule);
        })
        .catch(() => console.error('Unable to fetch rule'))
        .finally(() => setLoading(false));
    }
  }, [id]);

  const handleSave = (r: Rule) => {
    setLoading(true);
    createRule(r)
      .catch(() => console.error('Unable to save rule'))
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
