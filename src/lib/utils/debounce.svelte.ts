export function debounce<T>(
  getter: () => T,
  delay: number,
  initialValue: T,
): () => T {
  let value: T = $state(initialValue);
  let timer: number;

  $effect(() => {
    const newValue = getter(); // read here to subscribe to it
    if (timer) clearTimeout(timer);
    timer = setTimeout(() => {
      value = newValue;
    }, delay);
    return () => clearTimeout(timer);
  });

  return () => value!;
}
