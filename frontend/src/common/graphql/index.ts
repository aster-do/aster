import { ApiResponse, runPostRequest } from '../api';

type GraphQlQuery = {
  query: string;
  variables: Object;
};

export async function runGraphQLGetRequest<ReturnType>(
  url: string,
  body: GraphQlQuery,
  headers: HeadersInit = {}
): Promise<ApiResponse<ReturnType>> {
  const modifiedHeaders: HeadersInit = {
    ...headers,
    'Content-Type': 'application/json',
  };
  return runPostRequest<ReturnType, GraphQlQuery>(url, body, modifiedHeaders);
}

export async function runGraphQLPostRequest<ReturnType>(
  url: string,
  body: GraphQlQuery,
  headers: HeadersInit = {}
): Promise<ApiResponse<ReturnType>> {
  const modifiedHeaders: HeadersInit = {
    ...headers,
    'Content-Type': 'application/json',
  };
  return runPostRequest<ReturnType, GraphQlQuery>(url, body, modifiedHeaders);
}
