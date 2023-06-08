export enum HttpMethod {
  GET = 'GET',
  POST = 'POST',
  PUT = 'PUT',
  DELETE = 'DELETE',
  PATCH = 'PATCH',
}

export interface ApiResponse<T> {
  status: number;
  body?: T;
}

export const defaultHeaders = {
  'Content-Type': 'application/json',
};

export async function runRequest<ReturnType, BodyType>(
  url: string,
  method: HttpMethod,
  body?: BodyType,
  headers: HeadersInit = defaultHeaders
): Promise<ApiResponse<ReturnType>> {
  const response = await fetch(url, {
    method,
    headers,
    body: body ? JSON.stringify(body) : undefined,
  });

  return {
    status: response.status,
    body: response.status === 204 ? undefined : await response.json(),
  } as ApiResponse<ReturnType>;
}

export async function runGetRequest<ReturnType>(
  url: string,
  headers: HeadersInit = defaultHeaders
): Promise<ApiResponse<ReturnType>> {
  return runRequest<ReturnType, undefined>(
    url,
    HttpMethod.GET,
    undefined,
    headers
  );
}

export async function runPostRequest<ReturnType, BodyType>(
  url: string,
  body: BodyType,
  headers: HeadersInit = defaultHeaders
): Promise<ApiResponse<ReturnType>> {
  return runRequest<ReturnType, BodyType>(url, HttpMethod.POST, body, headers);
}

export async function runPutRequest<ReturnType, BodyType>(
  url: string,
  body: BodyType,
  headers: HeadersInit = defaultHeaders
): Promise<ApiResponse<ReturnType>> {
  return runRequest<ReturnType, BodyType>(url, HttpMethod.PUT, body, headers);
}

export async function runDeleteRequest<ReturnType, BodyType>(
  url: string,
  body?: BodyType,
  headers: HeadersInit = defaultHeaders
): Promise<ApiResponse<ReturnType>> {
  return runRequest<ReturnType, BodyType>(
    url,
    HttpMethod.DELETE,
    body,
    headers
  );
}

export async function runPatchRequest<ReturnType, BodyType>(
  url: string,
  body: BodyType,
  headers: HeadersInit = defaultHeaders
): Promise<ApiResponse<ReturnType>> {
  return runRequest<ReturnType, BodyType>(url, HttpMethod.PATCH, body, headers);
}
