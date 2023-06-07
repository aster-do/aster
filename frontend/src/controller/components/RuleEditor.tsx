import React from 'react';
import { Button, Stack, TextField } from '@mui/material';
import { Rule, RuleOperation } from '../model/rule';
import SimpleSelect from '../../common/components/SimpleSelect';

export interface RuleEditorProps {
  rule: Rule;
  onChange: (rule: Rule) => void;
  onSave: (rule: Rule) => void;
}

export default function RuleEditor({
  rule,
  onChange,
  onSave,
}: RuleEditorProps) {
  return (
    <Stack direction="column" spacing={6}>
      <TextField
        label="Name"
        value={rule.name}
        onChange={(e) => onChange({ ...rule, name: e.target.value })}
      />
      <SimpleSelect
        label="Operation"
        selected={rule.operation}
        options={[
          { value: RuleOperation.ADD, label: 'Add' },
          { value: RuleOperation.SUBTRACT, label: 'Subtract' },
          { value: RuleOperation.MULTIPLY, label: 'Multiply' },
          { value: RuleOperation.DIVIDE, label: 'Divide' },
        ]}
        onChange={(value) =>
          onChange({ ...rule, operation: value as RuleOperation })
        }
      />
      <TextField
        label="Number"
        type="number"
        value={rule.number}
        onChange={(e) => onChange({ ...rule, number: Number(e.target.value) })}
      />
      <Button variant="contained" color="primary" onClick={() => onSave(rule)}>
        Save
      </Button>
    </Stack>
  );
}
