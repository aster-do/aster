import React from 'react';
import dayjs, { Dayjs } from 'dayjs';
import { Stack } from '@mui/material';
import { DateTimePicker } from '@mui/x-date-pickers';
import { Frequency, Operator } from '../model/billableFilter';
import SimpleSelect from '../../common/components/SimpleSelect';

export interface BillableChartControlProps {
  operator: Operator;
  frequency: Frequency;
  start?: Date;
  end?: Date;
  onOperatorChange: (operator: Operator) => void;
  onFrequencyChange: (frequency: Frequency) => void;
  onStartChange: (start?: Date) => void;
  onEndChange: (end?: Date) => void;
}

export default function BillableChartControl({
  operator,
  frequency,
  start,
  end,
  onOperatorChange,
  onFrequencyChange,
  onStartChange,
  onEndChange,
}: BillableChartControlProps) {
  const handleStartChange = (date: Dayjs | null) => {
    onStartChange(date === null ? undefined : date.toDate());
  };

  const handleEndChange = (date: Dayjs | null) => {
    onEndChange(date === null ? undefined : date.toDate());
  };

  return (
    <Stack direction="row" spacing={2} alignItems="flex-end">
      <SimpleSelect
        label="Operator"
        options={[
          {
            value: Operator.MIN,
            label: 'Min',
          },
          {
            value: Operator.MAX,
            label: 'Max',
          },
          {
            value: Operator.AVG,
            label: 'Avg',
          },
          {
            value: Operator.SUM,
            label: 'Sum',
          },
          {
            value: Operator.COUNT,
            label: 'Count',
          },
        ]}
        selected={operator}
        onChange={onOperatorChange}
      />
      <SimpleSelect
        label="Frequency"
        options={[
          {
            value: Frequency.HOURLY,
            label: 'Hourly',
          },
          {
            value: Frequency.DAILY,
            label: 'Daily',
          },
          {
            value: Frequency.NONE,
            label: 'All time',
          },
        ]}
        selected={frequency}
        onChange={onFrequencyChange}
      />
      {frequency !== Frequency.NONE ? (
        <>
          <DateTimePicker
            label="Start date"
            value={dayjs(start)}
            onChange={handleStartChange}
            slotProps={{ textField: { variant: 'standard' } }}
            disableFuture
            ampm={false}
          />
          <DateTimePicker
            label="End date"
            value={dayjs(end)}
            onChange={handleEndChange}
            slotProps={{ textField: { variant: 'standard' } }}
            disableFuture
            ampm={false}
          />
        </>
      ) : null}
    </Stack>
  );
}
