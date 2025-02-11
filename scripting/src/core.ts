/**
 * Generates a UUIDv4
 *
 * @returns The generated UUID
 */
export function uuidv4(): string {
  return Deno.core.ops.op_uuid_v4();
}
