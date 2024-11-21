export class InvalidMessageTypeError extends Error {
  expected?: string;
  messageType?: string;

  constructor(expected: string | undefined, messageType: string | undefined) {
    super(
      `unexpected response message type '${messageType}' expecting ${expected}`
    );
    this.expected = expected;
    this.messageType = messageType;
  }
}

export class APIError extends Error {
  errorId?: number;

  constructor(errorId: number | undefined, message: string | undefined) {
    super(message ?? "unknown error");
    this.errorId = errorId;
  }
}
