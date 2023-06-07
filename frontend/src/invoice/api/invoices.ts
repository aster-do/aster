import useSWR from 'swr';
import BILLING_API_URL from './api';
import { runGraphQLPostRequest } from '../../common/graphql';
import { Billing, BillingDTO } from '../model';

export type PostInvoice = {
  data: {
    allBillings: BillingDTO[];
  };
};

export type ChartData = {
  group: string;
  date: string;
  value: number;
  surplus: number;
};

export function useInvoices(): {
  invoices: Billing[];
  loading: boolean;
  error: boolean;
  invoicesData: ChartData[];
} {
  const graphqlQuery = {
    query: `{
        allBillings {
          id
          generated_at          
          items
        }
      }`,
    variables: {},
  };

  const { data, error, isLoading } = useSWR(BILLING_API_URL, () =>
    runGraphQLPostRequest<PostInvoice>(BILLING_API_URL, graphqlQuery, {})
  );

  const invoicesData: ChartData[] = [];
  const invoices = data?.body?.data?.allBillings || [];
  const billings: Billing[] = [];

  invoices.forEach((invoice) => {
    const totalPrice = invoice?.items.reduce(
      (acc, item) => acc + item.price,
      0
    );
    invoicesData.push({
      group: 'invoices',
      date: invoice.generated_at,
      surplus: 0,
      value: totalPrice,
    });
    billings.push({ ...invoice, price: totalPrice });
  });

  return {
    invoices: billings,
    loading: isLoading,
    error,
    invoicesData,
  };
}
