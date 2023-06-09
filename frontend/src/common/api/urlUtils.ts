type UrlParamValue = string | number | boolean | Date;

type UrlParams = Record<
  string,
  UrlParamValue | UrlParamValue[] | undefined | null
>;

function valueToString(value: UrlParamValue): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  return value.toString();
}

export default function createUrl(baseUrl: string, params?: UrlParams): string {
  const url = new URL(baseUrl);

  if (!params) {
    return url.toString();
  }

  Object.entries(params).forEach(([key, value]) => {
    if (Array.isArray(value)) {
      url.searchParams.append(
        key,
        `[${value.map((v) => valueToString(v)).join(',')}]`
      );
    } else if (value !== null && value !== undefined) {
      url.searchParams.append(key, valueToString(value));
    }
  });
  return url.toString();
}
