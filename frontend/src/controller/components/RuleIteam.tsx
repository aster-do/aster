import React from 'react';
import {
  Button,
  Card,
  CardActionArea,
  CardActions,
  CardContent,
  CardHeader,
  Typography,
} from '@mui/material';
import { Rule, getRuleOperationDisplay } from '../model/rule';

export interface RuleItemProps {
  rule: Rule;
  onEdit: (rule: Rule) => void;
  onDelete: (rule: Rule) => void;
}

export default function RuleItem({ rule, onEdit, onDelete }: RuleItemProps) {
  const handleEdit = () => {
    onEdit(rule);
  };

  const handleDelete = () => {
    onDelete(rule);
  };

  return (
    <Card>
      <CardActionArea onClick={handleEdit}>
        <CardHeader title={rule.name} />
        <CardContent>
          <Typography variant="body1">{`Billable = value ${getRuleOperationDisplay(
            rule.operation
          )} ${rule.number}`}</Typography>
        </CardContent>
      </CardActionArea>
      <CardActions>
        <Button size="small" color="primary" onClick={handleDelete}>
          Delete
        </Button>
      </CardActions>
    </Card>
  );
}
