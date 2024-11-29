declare global {
  type EventMap = {
    chat: ChatEvent;
  };

  interface ChatEvent {
    message: string;
  }

  export function on<K extends keyof EventMap>(
    key: K,
    callback: (event: EventMap[K]) => void
  ): void;
}
