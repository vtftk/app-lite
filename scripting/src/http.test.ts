import { get, put, post, patch, request } from "./http";

describe("HTTP Utilities", () => {
  beforeEach(() => {
    // Mock Deno.core.ops
    globalThis.Deno = {
      core: {
        ops: {
          op_http_request: jest.fn(),
        },
      },
    } as never;
  });

  describe("request", () => {
    it("should throw an error if url is not provided", async () => {
      await expect(request({} as never)).rejects.toThrow(
        "url must be a present and a string",
      );
    });

    it("should throw an error if url is not a string", async () => {
      await expect(request({ url: 123 as never })).rejects.toThrow(
        "url must be a present and a string",
      );
    });

    it("should handle string body", async () => {
      (Deno.core.ops.op_http_request as jest.Mock).mockResolvedValueOnce({
        status: 200,
        headers: { "content-type": "text/plain" },
        body: "response text",
      });

      const response = await request({
        url: "http://example.com",
        body: "test body",
        responseFormat: "text",
      });

      expect(Deno.core.ops.op_http_request).toHaveBeenCalledWith({
        url: "http://example.com",
        method: undefined,
        body: { type: "text", value: "test body" },
        headers: undefined,
        timeout: undefined,
        response_format: "text",
      });
      expect(response.ok).toBe(true);
      expect(response.body).toBe("response text");
    });

    it("should handle JSON body", async () => {
      (Deno.core.ops.op_http_request as jest.Mock).mockResolvedValueOnce({
        status: 200,
        headers: { "content-type": "application/json" },
        body: { success: true },
      });

      const response = await request({
        url: "http://example.com",
        body: { key: "value" },
        responseFormat: "json",
      });

      expect(Deno.core.ops.op_http_request).toHaveBeenCalledWith({
        url: "http://example.com",
        method: undefined,
        body: { type: "json", value: { key: "value" } },
        headers: undefined,
        timeout: undefined,
        response_format: "json",
      });
      expect(response.ok).toBe(true);
      expect(response.body).toEqual({ success: true });
    });

    it("should default to text response format", async () => {
      (Deno.core.ops.op_http_request as jest.Mock).mockResolvedValueOnce({
        status: 200,
        headers: { "content-type": "text/plain" },
        body: "default text",
      });

      const response = await request({ url: "http://example.com" });
      expect(Deno.core.ops.op_http_request).toHaveBeenCalledWith(
        expect.objectContaining({
          response_format: "text",
        }),
      );
      expect(response.body).toBe("default text");
    });
  });

  describe("get", () => {
    it("should make a GET request", async () => {
      (Deno.core.ops.op_http_request as jest.Mock).mockResolvedValueOnce({
        status: 200,
        headers: { "content-type": "text/plain" },
        body: "response text",
      });

      const response = await get("http://example.com");
      expect(Deno.core.ops.op_http_request).toHaveBeenCalledWith({
        url: "http://example.com",
        method: "GET",
        body: undefined,
        headers: undefined,
        timeout: undefined,
        response_format: "text",
      });
      expect(response.body).toBe("response text");
    });

    it("should pass options to the request", async () => {
      (Deno.core.ops.op_http_request as jest.Mock).mockResolvedValueOnce({
        status: 200,
        headers: { "content-type": "application/json" },
        body: { key: "value" },
      });

      const response = await get("http://example.com", {
        responseFormat: "json",
        headers: { Authorization: "Bearer token" },
      });
      expect(Deno.core.ops.op_http_request).toHaveBeenCalledWith({
        url: "http://example.com",
        method: "GET",
        body: undefined,
        headers: { Authorization: "Bearer token" },
        timeout: undefined,
        response_format: "json",
      });
      expect(response.body).toEqual({ key: "value" });
    });
  });

  describe("post", () => {
    it("should make a POST request with a body", async () => {
      (Deno.core.ops.op_http_request as jest.Mock).mockResolvedValueOnce({
        status: 201,
        headers: { "content-type": "application/json" },
        body: { created: true },
      });

      const response = await post("http://example.com", { data: "value" });
      expect(Deno.core.ops.op_http_request).toHaveBeenCalledWith({
        url: "http://example.com",
        method: "POST",
        body: { type: "json", value: { data: "value" } },
        headers: undefined,
        timeout: undefined,
        response_format: "text",
      });
      expect(response.ok).toBe(true);
      expect(response.body).toEqual({ created: true });
    });

    it("should handle undefined body", async () => {
      (Deno.core.ops.op_http_request as jest.Mock).mockResolvedValueOnce({
        status: 200,
        headers: { "content-type": "text/plain" },
        body: "no body",
      });

      const response = await post("http://example.com");
      expect(Deno.core.ops.op_http_request).toHaveBeenCalledWith(
        expect.objectContaining({
          body: undefined,
        }),
      );
      expect(response.body).toBe("no body");
    });
  });

  describe("put", () => {
    it("should make a PUT request", async () => {
      (Deno.core.ops.op_http_request as jest.Mock).mockResolvedValueOnce({
        status: 200,
        headers: { "content-type": "text/plain" },
        body: "updated",
      });

      const response = await put("http://example.com", { data: "new value" });
      expect(Deno.core.ops.op_http_request).toHaveBeenCalledWith({
        url: "http://example.com",
        method: "PUT",
        body: { type: "json", value: { data: "new value" } },
        headers: undefined,
        timeout: undefined,
        response_format: "text",
      });
      expect(response.body).toBe("updated");
    });
  });

  describe("patch", () => {
    it("should make a PATCH request", async () => {
      (Deno.core.ops.op_http_request as jest.Mock).mockResolvedValueOnce({
        status: 200,
        headers: { "content-type": "application/json" },
        body: { patched: true },
      });

      const response = await patch("http://example.com", { key: "value" });
      expect(Deno.core.ops.op_http_request).toHaveBeenCalledWith({
        url: "http://example.com",
        method: "PATCH",
        body: { type: "json", value: { key: "value" } },
        headers: undefined,
        timeout: undefined,
        response_format: "text",
      });
      expect(response.body).toEqual({ patched: true });
    });
  });
});
