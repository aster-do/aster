import useSWR from 'swr';
import { defaultHeaders, runGetRequest, runPostRequest } from '.';

export interface SWRResponse<T> {
  data?: T;
  success: boolean;
  loading: boolean;
}

export function useGetSWR<ReturnType>(
  url: string,
  headers: HeadersInit = defaultHeaders
): SWRResponse<ReturnType> {
  const { data, error, isLoading } = useSWR(url, () =>
    runGetRequest<ReturnType>(url, headers)
  );

  return {
    data: data?.body,
    success: !error && !isLoading,
    loading: isLoading,
  };
}

export function usePostSWR<ReturnType, BodyType>(
  url: string,
  body: BodyType,
  headers: HeadersInit = defaultHeaders
): SWRResponse<ReturnType> {
  const { data, error, isLoading } = useSWR(url, () =>
    runPostRequest<ReturnType, BodyType>(url, body, headers)
  );

  return {
    data: data?.body,
    success: !error && !isLoading,
    loading: isLoading,
  };
}
