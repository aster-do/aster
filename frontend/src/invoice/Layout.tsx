import { Button, Grid } from '@mui/material';
import React from 'react';
import { LineChartOptions, ScaleTypes } from '@carbon/charts/interfaces';
import { LineChart } from '@carbon/charts-react';
import InvoiceCard from './components/InvoiceCard';
import { useInvoices } from './api/invoices';
import LoadingWrapper from '../common/components/LoadingWrapper';
import MessageWrapper from '../common/components/MessageWrapper';

export default function Layout() {

  const { invoices: billings, loading, error, invoicesData } = useInvoices();
  const chartOptions: LineChartOptions = {
    title: 'Invoices price evolution',
    axes: {
      bottom: {
        title: 'Monthly invoice',
        mapsTo: 'date',
        scaleType: ScaleTypes.TIME,
      },
      left: {
        mapsTo: 'value',
        title: 'Total price',
        scaleType: ScaleTypes.LINEAR,
      },
    },
    curve: 'line',
    height: '400px',
  };

  return (
    <div>
      <MessageWrapper message="No invoices found" show={error}>
        <LoadingWrapper loading={loading}>
          <Grid container spacing={2} direction="column">
            {billings &&
              billings.length > 0 &&
              billings.map((billing) => <InvoiceCard billing={billing} />)}
          </Grid>
        </LoadingWrapper>
      </MessageWrapper>

      <LineChart data={invoicesData} options={chartOptions} />

      <Button
        sx={{
          px: 2,
          py: 1,
          borderRadius: '5px',
          backgroundColor: 'primary.main',
          color: 'white',
        }}
      >
        Generate
      </Button>
    </div>
  );
}
