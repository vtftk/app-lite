import { watch } from "runed";

export function createSelection<T extends { id: string }>(items: () => T[]) {
  let selection: string[] = $state([]);

  const itemIds = () => items().map((item) => item.id);

  // Handle changes to the items selection removing any items
  // that are no longer present
  watch(itemIds, (itemIds) => {
    const filteredSelection = selection.filter((id) => itemIds.includes(id));

    // Only trigger the update if the selection changed (Prevent infinite loop)
    if (filteredSelection.length !== selection.length) {
      selection = filteredSelection;
    }
  });

  function toggle(itemId: string) {
    if (selection.includes(itemId)) {
      selection = selection.filter((id) => id !== itemId);
    } else {
      selection = [...selection, itemId];
    }
  }

  function toggleAll() {
    if (isAll()) {
      clear();
    } else {
      selection = itemIds();
    }
  }

  function includes(id: string) {
    return selection.includes(id);
  }

  /**
   * Takes the current selection replacing it with an empty selection
   *
   * @returns The current selection
   */
  function take(): string[] {
    const current = selection;
    selection = [];
    return current;
  }

  function total(): number {
    return selection.length;
  }

  function isAll() {
    const selectionTotal = selection.length;
    return selectionTotal > 0 && selectionTotal === items().length;
  }

  function isEmpty() {
    return selection.length < 1;
  }

  function clear() {
    selection = [];
  }

  return {
    get selection() {
      return selection;
    },

    toggle,
    toggleAll,
    includes,
    take,
    isAll,
    total,
    isEmpty,
    clear,
  };
}
