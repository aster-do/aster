import useSWR from 'swr';
import { Frequency, Operator } from './billableFilter';
import createUrl from '../../common/api/urlUtils';
import API_URL from './api';
import { runGetRequest } from '../../common/api';
import { ChartItem } from './chartItem';

export interface Billable {
  name: string;
  price: number;
  timestamp: Date;
  value: number;
}

export interface BillableFilters {
  operator: Operator;
  frequency?: Frequency;
  start?: Date;
  end?: Date;
  name?: string[];
}

export interface BillableResponse {
  billables: Billable[];
  loading: boolean;
  success: boolean;
}

export interface ChartBillableResponse {
  chartItems: ChartItem[];
  loading: boolean;
  success: boolean;
}

export function useBillables({
  operator,
  frequency,
  start,
  end,
  name,
}: BillableFilters): BillableResponse {
  const url = createUrl(`${API_URL}/billables`, {
    operator,
    frequency: frequency === Frequency.NONE ? undefined : frequency,
    start: frequency === Frequency.NONE ? undefined : start,
    end: frequency === Frequency.NONE ? undefined : end,
    name,
  });
  const { data, error, isLoading } = useSWR(url, () =>
    runGetRequest<Billable[]>(url)
  );

  return {
    billables: data?.body || [],
    loading: isLoading,
    success: !error,
  };
}

export function useChartBillables({
  operator,
  frequency,
  start,
  end,
  name,
}: BillableFilters): ChartBillableResponse {
  const { billables, success, loading } = useBillables({
    operator,
    frequency,
    start,
    end,
    name,
  });
  return {
    chartItems: billables.map(
      (billable) =>
        ({
          group: billable.name,
          key: billable.timestamp,
          value: billable.value,
        } as ChartItem)
    ),
    loading,
    success,
  };
}
