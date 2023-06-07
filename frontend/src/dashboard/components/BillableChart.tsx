import { Stack } from '@mui/material';
import React, { useState } from 'react';
import { GroupedBarChart } from '@carbon/charts-react';
import BillableChartControl from './BillableChartControl';
import { Frequency, Operator } from '../model/billableFilter';
import MessageWrapper from '../../common/components/MessageWrapper';
import LoadingWrapper from '../../common/components/LoadingWrapper';
import { useChartBillables } from '../model/billable';

interface BillableChartProps {
  title: string;
}

export default function BillableChart({ title }: BillableChartProps) {
  const chartOptions = {
    title,
    axes: {
      left: {
        mapsTo: 'value',
      },
      bottom: {
        mapsTo: 'key',
      },
    },
    height: '400px',
  };
  const [operator, setOperator] = useState<Operator>(Operator.AVG);
  const [frequency, setFrequency] = useState<Frequency>(Frequency.HOURLY);
  const [start, setStart] = useState<Date | undefined>(undefined);
  const [end, setEnd] = useState<Date | undefined>(undefined);
  const { chartItems, success, loading } = useChartBillables({
    operator,
    frequency,
    start,
    end,
  });

  const handleOperatorChange = (op: Operator) => {
    setOperator(op);
  };

  const handleFrequencyChange = (freq: Frequency) => {
    setFrequency(freq);
  };

  const handleStartChange = (date?: Date) => {
    setStart(date);
  };

  const handleEndChange = (date?: Date) => {
    setEnd(date);
  };

  return (
    <Stack direction="column" sx={{ width: '100%' }}>
      <BillableChartControl
        operator={operator}
        frequency={frequency}
        start={start}
        end={end}
        onOperatorChange={handleOperatorChange}
        onFrequencyChange={handleFrequencyChange}
        onStartChange={handleStartChange}
        onEndChange={handleEndChange}
      />
      <LoadingWrapper loading={loading}>
        <MessageWrapper message="Error loading billables" show={!success}>
          <MessageWrapper
            message="No billables found"
            show={chartItems.length === 0}
          >
            <GroupedBarChart data={chartItems} options={chartOptions} />
          </MessageWrapper>
        </MessageWrapper>
      </LoadingWrapper>
    </Stack>
  );
}
