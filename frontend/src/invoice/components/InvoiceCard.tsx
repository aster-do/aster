import React from 'react';
import { Card, CardContent, Grid, Typography } from '@mui/material';
import { Billing } from '../model';

type BillingCardProps = {
  billing: Billing;
};

export default function InvoiceCard({ billing }: BillingCardProps) {

  return (
    <Grid item xs={10}>
      <Card>
        <CardContent>
          <Grid
            container
            spacing={2}
            direction="row"
            justifyContent="space-between"
            alignContent="center"
          >
            <Grid item>
              <Typography variant="h5" component="div" maxWidth={200}>
                Invoice
              </Typography>

              <Typography
                variant="caption"
                component="div"
                color="text.secondary"
                maxWidth={400}
                minWidth={200}
              >
                {billing.id}
              </Typography>
              <Typography
                variant="subtitle1"
                component="div"
                maxWidth={200}
                minWidth={200}
              >
                {`Date : ${Date.parse(billing.generated_at).toString()}`}
              </Typography>
            </Grid>
            <Grid item>
              <Typography variant="subtitle1" component="div" maxWidth={200}>
                {`Total : ${billing.price}€`}
              </Typography>
            </Grid>
          </Grid>
          {`Items: ${billing.items.length}`}
        </CardContent>
      </Card>
    </Grid>
  );
}
