export function formatDate(date: Date) {
  return new Intl.DateTimeFormat(undefined, {
    dateStyle: "medium",
  }).format(date);
}
export function formatTime(date: Date) {
  return new Intl.DateTimeFormat(undefined, {
    timeStyle: "short",
  }).format(date);
}
