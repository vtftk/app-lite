import {
  remove,
  getText,
  setText,
  setArray,
  getArray,
  setObject,
  getObject,
  createScopedCounter,
} from "./kv";

describe("KV Operations", () => {
  beforeEach(() => {
    // Mock Deno.core.ops
    globalThis.Deno = {
      core: {
        ops: {
          op_kv_get: jest.fn(),
          op_kv_set: jest.fn(),
          op_kv_remove: jest.fn(),
        },
      },
    } as never;
  });

  describe("remove", () => {
    it("should call op_kv_remove with the correct arguments", async () => {
      await remove("testKey");
      expect(Deno.core.ops.op_kv_remove).toHaveBeenCalledWith("testKey");
    });

    it("should throw an error if key is not a string", async () => {
      await expect(() => remove(123 as never)).toThrow(
        new Error("key must be a string"),
      );
    });
  });

  describe("getText", () => {
    it("should return the value if it exists", async () => {
      (Deno.core.ops.op_kv_get as jest.Mock).mockResolvedValueOnce("testValue");
      const result = await getText("testKey");
      expect(result).toBe("testValue");
    });

    it("should return the default value if the key does not exist", async () => {
      (Deno.core.ops.op_kv_get as jest.Mock).mockResolvedValueOnce(null);
      const result = await getText("testKey", "defaultValue");
      expect(result).toBe("defaultValue");
    });

    it("should throw an error if key is not a string", async () => {
      await expect(() => getText(123 as never)).rejects.toThrow(
        new Error("key must be a string"),
      );
    });
  });

  describe("setText", () => {
    it("should call op_kv_set with the correct arguments", async () => {
      await setText("testKey", "testValue");
      expect(Deno.core.ops.op_kv_set).toHaveBeenCalledWith(
        "Text",
        "testKey",
        "testValue",
      );
    });

    it("should throw an error if key is not a string", async () => {
      await expect(() => setText(123 as never, "testValue")).toThrow(
        new Error("key must be a string"),
      );
    });

    it("should throw an error if value is not a string", async () => {
      await expect(() => setText("testKey", 123 as never)).toThrow(
        new Error("value must be a string"),
      );
    });
  });

  describe("setArray", () => {
    it("should call op_kv_set with the correct arguments", async () => {
      const array = [1, 2, 3];
      await setArray("testKey", array);
      expect(Deno.core.ops.op_kv_set).toHaveBeenCalledWith(
        "Array",
        "testKey",
        array,
      );
    });

    it("should throw an error if key is not a string", async () => {
      await expect(() => setArray(123 as never, [1, 2, 3])).toThrow(
        new Error("key must be a string"),
      );
    });

    it("should throw an error if value is not an array", async () => {
      await expect(() => setArray("testKey", "notArray" as never)).toThrow(
        new Error("value must be an array"),
      );
    });
  });

  describe("getArray", () => {
    it("should return the array if it exists", async () => {
      const array = [1, 2, 3];
      (Deno.core.ops.op_kv_get as jest.Mock).mockResolvedValueOnce(
        JSON.stringify(array),
      );
      const result = await getArray("testKey");
      expect(result).toEqual(array);
    });

    it("should return the default value if the key does not exist", async () => {
      const defaultArray = [4, 5, 6];
      (Deno.core.ops.op_kv_get as jest.Mock).mockResolvedValueOnce(null);
      const result = await getArray("testKey", defaultArray);
      expect(result).toEqual(defaultArray);
    });

    it("should throw an error if key is not a string", async () => {
      await expect(() => getArray(123 as never)).rejects.toThrow(
        new Error("key must be a string"),
      );
    });
  });

  describe("setObject", () => {
    it("should call op_kv_set with the correct arguments", async () => {
      const obj = { a: 1 };
      await setObject("testKey", obj);
      expect(Deno.core.ops.op_kv_set).toHaveBeenCalledWith(
        "Object",
        "testKey",
        JSON.stringify(obj),
      );
    });

    it("should throw an error if key is not a string", async () => {
      await expect(() => setObject(123 as never, { a: 1 })).toThrow(
        new Error("key must be a string"),
      );
    });

    it("should throw an error if value is not an object", async () => {
      await expect(() => setObject("testKey", "notObject" as never)).toThrow(
        new Error("value must be a object"),
      );
    });
  });

  describe("getObject", () => {
    it("should return the object if it exists", async () => {
      const obj = { a: 1 };
      (Deno.core.ops.op_kv_get as jest.Mock).mockResolvedValueOnce(
        JSON.stringify(obj),
      );
      const result = await getObject("testKey");
      expect(result).toEqual(obj);
    });

    it("should return the default value if the key does not exist", async () => {
      const defaultObj = { b: 2 };
      (Deno.core.ops.op_kv_get as jest.Mock).mockResolvedValueOnce(null);
      const result = await getObject("testKey", defaultObj);
      expect(result).toEqual(defaultObj);
    });

    it("should throw an error if key is not a string", async () => {
      await expect(() => getObject(123 as never)).rejects.toThrow(
        new Error("key must be a string"),
      );
    });
  });

  describe("createScopedCounter", () => {
    it("should get the value for a specific scope", async () => {
      const counter = createScopedCounter("counterKey");
      (Deno.core.ops.op_kv_get as jest.Mock).mockResolvedValueOnce(
        JSON.stringify({ scopeA: 5 }),
      );
      const value = await counter.get("scopeA");
      expect(value).toBe(5);
    });

    it("should increase the value for a scope", async () => {
      const counter = createScopedCounter("counterKey");
      (Deno.core.ops.op_kv_get as jest.Mock).mockResolvedValueOnce(
        JSON.stringify({}),
      );
      const value = await counter.increase("scopeB");
      expect(value).toBe(1);
    });

    it("should set the value for a scope", async () => {
      const counter = createScopedCounter("counterKey");
      (Deno.core.ops.op_kv_get as jest.Mock).mockResolvedValueOnce(
        JSON.stringify({}),
      );
      await counter.set("scopeC", 10);
      expect(Deno.core.ops.op_kv_set).toHaveBeenCalled();
    });
  });
});
