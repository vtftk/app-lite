type HttpMethod =
  | "GET"
  | "POST"
  | "PUT"
  | "DELETE"
  | "PATCH"
  | "OPTIONS"
  | "HEAD"
  | "TRACE"
  | "CONNECT";

type HttpResponseFormatMap = {
  json: object;
  text: string;
};

type HttpResponseFormat = keyof HttpResponseFormatMap;

type HttpBody = object | string;

type HttpOptions = Partial<{
  /**
   * URL to make the HTTP request to
   */
  url: string;

  /**
   * HTTP request method
   */
  method: HttpMethod;

  /**
   * Response type format expected
   */
  responseFormat: HttpResponseFormat;

  /**
   * Request headers
   */
  headers: Partial<Record<string, string>>;

  /**
   * HTTP request body
   */
  body: HttpBody;

  /**
   * Optional request timeout in milliseconds
   */
  timeout: number;
}>;

export type HttpResponse<BodyFormat> = {
  /**
   * Response status code
   */
  status: number;

  /**
   * Response headers
   */
  headers: Partial<Record<string, string>>;

  /**
   * Helper to check if the response is a 2xx response code
   */
  get ok(): boolean;

  /**
   * Response body
   */
  body: HttpResponseBody<BodyFormat>;
};

type HttpResponseBody<F> = F extends keyof HttpResponseFormatMap
  ? HttpResponseFormatMap[F]
  : HttpResponseFormatMap["text"];

/**
 * Performs an HTTP request with the provided options
 *
 * @param options The request options
 * @returns Promise to the HTTP response
 */
export async function request<O extends HttpOptions>(
  options: O,
): Promise<HttpResponse<O["responseFormat"]>> {
  // URL must be defined and a string
  if (options.url === undefined || typeof options.url !== "string") {
    throw new Error("url must be a present and a string");
  }

  let requestBody = undefined;
  const body = options.body;
  if (typeof body === "string") {
    requestBody = { type: "text", value: body };
  } else if (typeof body === "object") {
    requestBody = { type: "json", value: body };
  }

  const responseFormat = (options.responseFormat ?? "text").toLowerCase();

  const response = await Deno.core.ops.op_http_request({
    url: options.url,
    method: options.method,
    body: requestBody,
    headers: options.headers,
    timeout: options.timeout,
    response_format: responseFormat,
  });

  return {
    ...response,

    get ok() {
      return Math.floor(response.status / 100) == 2;
    },
  };
}

// Get requests do not need a body, method, or URL in the options
type GetHttpOptions = Omit<HttpOptions, "body" | "method" | "url">;

/**
 * Perform an HTTP GET request to the provided URL
 *
 * @param url The URL to GET
 * @param options Additional HTTP request options
 * @returns Promise to the HTTP response
 */
export function get<O extends GetHttpOptions>(
  url: string,
  options?: O,
): Promise<HttpResponse<O["responseFormat"]>> {
  return request({ ...options, url, method: "GET" });
}

/**
 * Perform an HTTP POST request to the provided URL with
 * an optional request body
 *
 * @param url The URL to POST
 * @param body Optional request body
 * @param options Additional HTTP request options
 * @returns Promise to the HTTP response
 */
export function post<B extends HttpBody | undefined, O extends HttpOptions>(
  url: string,
  body?: B,
  options?: O,
): Promise<HttpResponse<O["responseFormat"]>> {
  return request({ ...options, url, method: "POST", body });
}

/**
 * Perform an HTTP PUT request to the provided URL with
 * an optional request body
 *
 * @param url The URL to PUT
 * @param body Optional request body
 * @param options Additional HTTP request options
 * @returns Promise to the HTTP response
 */
export function put<B extends HttpBody | undefined, O extends HttpOptions>(
  url: string,
  body?: B,
  options?: O,
): Promise<HttpResponse<O["responseFormat"]>> {
  return request({ ...options, url, method: "PUT", body });
}

/**
 * Perform an HTTP PATCH request to the provided URL with
 * an optional request body
 *
 * @param url The URL to PATCH
 * @param body Optional request body
 * @param options Additional HTTP request options
 * @returns Promise to the HTTP response
 */
export function patch<B extends HttpBody | undefined, O extends HttpOptions>(
  url: string,
  body?: B,
  options?: O,
): Promise<HttpResponse<O["responseFormat"]>> {
  return request({ ...options, url, method: "PATCH", body });
}
