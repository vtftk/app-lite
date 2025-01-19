export function filterNameSearch<T extends { name: string }>(
  options: T[],
  search: string,
) {
  search = search.trim().toLowerCase();

  if (search.length < 1) return options;

  return options.filter((option) => {
    const name = option.name.trim().toLowerCase();
    return name.startsWith(search) || name.includes(search);
  });
}
