import { Frequency, Operator } from './billableFilter';
import createUrl from '../../common/api/urlUtils';
import API_URL from './api';
import { ChartItem } from './chartItem';
import { SWRResponse, useGetSWR } from '../../common/api/swr';

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

export function useChartBillables({
  operator,
  frequency,
  start,
  end,
  name,
}: BillableFilters): SWRResponse<ChartItem[]> {
  const url = createUrl(`${API_URL}/billables`, {
    operator,
    frequency: frequency === Frequency.NONE ? undefined : frequency,
    start,
    end,
    name,
  });
  const { data, loading, success } = useGetSWR<Billable[]>(url);
  return {
    data: data?.map(
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
