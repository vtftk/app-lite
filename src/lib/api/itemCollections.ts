import type {
  Item,
  ItemId,
  ItemWithOrder,
  UpdateOrdering,
  ItemCollectionId,
  CreateItemCollection,
  ItemCollectionWithItems,
  ItemCollectionWithItemLinks,
} from "$shared/dataV2";

import { invoke } from "@tauri-apps/api/core";
import { createQuery } from "@tanstack/svelte-query";

import { queryClient } from "./utils";

const ITEM_COLLECTIONS_KEY = ["item-collections"];

export async function createItemCollection(create: CreateItemCollection) {
  const result = await invoke<ItemCollectionWithItemLinks>(
    "create_item_collection",
    { create },
  );

  const queryKey = itemCollectionKey(result.id);
  queryClient.setQueryData(queryKey, result);
  queryClient.invalidateQueries({ queryKey: ITEM_COLLECTIONS_KEY });

  return result;
}

export function getItemCollections() {
  return invoke<ItemCollectionWithItemLinks[]>("get_item_collections");
}

export function getItemCollection(id: ItemCollectionId) {
  return invoke<ItemCollectionWithItems>("get_item_collection", { id });
}

export async function updateItemCollectionOrderings(update: UpdateOrdering[]) {
  await invoke<void>("update_item_collection_orderings", { update });

  queryClient.invalidateQueries({ queryKey: ITEM_COLLECTIONS_KEY });
}

export async function updateItemCollectionItemOrderings(
  id: ItemCollectionId,
  update: UpdateOrdering[],
) {
  await invoke<void>("update_item_collection_item_orderings", { id, update });

  queryClient.invalidateQueries({ queryKey: itemCollectionKey(id) });
  queryClient.invalidateQueries({ queryKey: ITEM_COLLECTIONS_KEY });
}

export async function setItemCollectionItems(
  id: ItemCollectionId,
  items: ItemId[],
) {
  await invoke<void>("set_item_collection_items", { id, items });

  queryClient.invalidateQueries({ queryKey: itemCollectionKey(id) });
  queryClient.invalidateQueries({ queryKey: ITEM_COLLECTIONS_KEY });
}

export async function appendItemCollectionItems(
  id: ItemCollectionId,
  items: ItemId[],
) {
  await invoke<void>("append_item_collection_items", { id, items });

  queryClient.invalidateQueries({ queryKey: itemCollectionKey(id) });
  queryClient.invalidateQueries({ queryKey: ITEM_COLLECTIONS_KEY });
}

export async function deleteItemCollection(id: ItemCollectionId) {
  await invoke<void>("delete_item_collection", { id });

  const itemKey = itemCollectionKey(id);

  queryClient.cancelQueries({ queryKey: itemKey });
  queryClient.setQueryData(itemKey, undefined);

  queryClient.invalidateQueries({ queryKey: ITEM_COLLECTIONS_KEY });
}

// ---------------------------------------------------------------------------

export function mergeItemCollectionItems(
  collection: ItemCollectionWithItemLinks,
  items: Item[],
): ItemCollectionWithItems {
  const foundItems = collection.items.reduce((values, itemLink) => {
    const item = items.find((item) => item.id === itemLink.item_id);

    if (item !== undefined) {
      values.push({ ...item, order: itemLink.order });
    }

    return values;
  }, [] as ItemWithOrder[]);

  return {
    ...collection,
    items: foundItems,
  };
}

// ---------------------------------------------------------------------------

export function createItemCollectionsQuery() {
  return createQuery({
    queryKey: ITEM_COLLECTIONS_KEY,
    queryFn: getItemCollections,
  });
}

export function itemCollectionKey(id: ItemCollectionId) {
  return ["item-collection", id] as const;
}

export function createItemCollectionQuery(id: ItemCollectionId) {
  return createQuery({
    queryKey: itemCollectionKey(id),
    queryFn: () => getItemCollection(id),
  });
}
