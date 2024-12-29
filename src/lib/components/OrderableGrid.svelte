<script lang="ts" generics="T extends { id: string }">
  import type { Snippet } from "svelte";
  import type { UpdateOrdering } from "$shared/dataV2";

  import { onMount } from "svelte";
  import {
    dndzone,
    type DndEvent,
    SHADOW_ITEM_MARKER_PROPERTY_NAME,
  } from "svelte-dnd-action";

  import SizeAndPositionManager from "./SizeAndPositionManager";

  type Props = {
    // Available items for the grid
    items: T[];

    // Snippet for rendering items
    item: Snippet<[T]>;

    itemHeight?: number;

    // Called when the ordering of the list is due for an update
    onUpdateOrder: (ordering: UpdateOrdering[]) => Promise<void>;

    // Optionally disable ordering when set
    disableOrdering?: boolean;
  };

  const {
    items: _items,
    item: renderItem,
    itemHeight = 68,
    onUpdateOrder,
    disableOrdering,
  }: Props = $props();

  let containerHeight: number = $state(0);
  let wrapper: HTMLDivElement | undefined = $state();
  let offsetState = $state(0);
  let wrapperStyle = $state("");
  let innerStyle = $state("");

  let previousOffset = 0;

  let styleCache: Partial<Record<number, string>> = {};

  // Local state for list of items to allow reordering
  let items: T[] = $state([]);

  let virtualizedItems: { item: T; index: number; style: string }[] = $state(
    [],
  );

  const itemCount = $derived(Math.ceil(items.length / 2));

  const sizeAndPositionManager = new SizeAndPositionManager({
    itemCount: 0,
    itemSize: itemHeight,
    estimatedItemSize: itemHeight,
  });

  // Update the items when the props change
  $effect(() => {
    items = _items;
  });

  function handleDndConsider(e: CustomEvent<DndEvent<T>>) {
    items = e.detail.items;
  }

  async function handleDndFinalize(e: CustomEvent<DndEvent<T>>) {
    items = e.detail.items;
    onUpdateOrder(items.map((item, index) => ({ id: item.id, order: index })));
  }

  $effect(() => {
    sizeAndPositionManager.updateConfig({
      itemSize: itemHeight,
      itemCount,
      estimatedItemSize: itemHeight,
    });

    recomputeSizes();
  });

  $effect(() => {
    if (previousOffset !== offsetState) {
      refresh();
      previousOffset = offsetState;
    }
  });

  $effect(() => {
    const _ = containerHeight;
    recomputeSizes();
  });

  refresh();

  /**
   * the third argument for event bundler
   * @see https://github.com/WICG/EventListenerOptions/blob/gh-pages/explainer.md
   */
  const thirdEventArg = (() => {
    let result: boolean | { passive: true } = false;

    try {
      const arg = Object.defineProperty({}, "passive", {
        get() {
          result = { passive: true };
          return true;
        },
      });

      // @ts-expect-error Testing passive
      window.addEventListener("testpassive", arg, arg);
      // @ts-expect-error Testing passive
      window.remove("testpassive", arg, arg);
    } catch (_e) {
      /* */
    }

    return result;
  })();

  onMount(() => {
    if (wrapper === undefined) return;

    const resizeObserver = new ResizeObserver((entries) => {
      for (let entry of entries) {
        containerHeight = entry.target.clientHeight;
      }
    });

    containerHeight = wrapper.clientHeight;

    resizeObserver.observe(wrapper);
    wrapper.addEventListener("scroll", handleScroll, thirdEventArg);

    return () => {
      if (wrapper === undefined) return;
      wrapper.removeEventListener("scroll", handleScroll);
    };
  });

  function refresh() {
    const { start, stop } = sizeAndPositionManager.getVisibleRange({
      containerSize: containerHeight,
      offset: offsetState,
      overscanCount: 2,
    } as never);

    let updatedItems = [];

    const totalSize = sizeAndPositionManager.getTotalSize();

    innerStyle = `flex-direction:column;height:${totalSize}px;`;

    if (start !== undefined && stop !== undefined) {
      for (let index = start; index <= (stop + 1) * 2; index++) {
        let item = items[index];
        if (index < items.length) {
          updatedItems.push({
            item,
            index,
            style: getStyle(index),
          });
        }
      }
    }

    virtualizedItems = updatedItems;
  }

  function recomputeSizes(startIndex = 0) {
    styleCache = {};
    sizeAndPositionManager.resetItem(startIndex);
    refresh();
  }

  function handleScroll(event: Event) {
    const offset = wrapper?.scrollTop ?? 0;

    if (offset < 0 || offsetState === offset || event.target !== wrapper)
      return;

    offsetState = offset;
  }

  function getStyle(index: number) {
    if (styleCache[index]) return styleCache[index];

    const row = Math.floor(index / 2);

    const listIndex = row;
    const column = index % 2;

    const { size, offset } =
      sizeAndPositionManager.getSizeAndPositionForIndex(listIndex);

    let style = `height:${size}px;position:absolute;top:${offset}px;`;
    if (column === 1) {
      style += "width:calc(50% - 4px);left:calc(50% + 4px);";
    } else {
      style += "left:0;width:calc(50% - 4px)";
    }

    return (styleCache[index] = style);
  }
</script>

<div bind:this={wrapper} class="wrapper" style={wrapperStyle}>
  <div
    style={innerStyle}
    class="grid"
    use:dndzone={{ items, dragDisabled: disableOrdering }}
    onconsider={handleDndConsider}
    onfinalize={handleDndFinalize}
  >
    {#each virtualizedItems as item (item.item.id)}
      <div class="item-wrapper" style={item.style}>
        {@render renderItem(item.item)}

        <!-- eslint-disable-next-line @typescript-eslint/no-explicit-any -->
        {#if (item.item as any)[SHADOW_ITEM_MARKER_PROPERTY_NAME]}
          <div class="custom-shadow-item"></div>
        {/if}
      </div>
    {/each}
  </div>
</div>

<style>
  .item-wrapper {
    position: relative;
  }

  .custom-shadow-item {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    visibility: visible;
    border: 3px dashed #444;
    background: #212121;
    opacity: 0.5;
    margin: 0;
  }

  .wrapper {
    overflow: auto;
    will-change: transform;
    -webkit-overflow-scrolling: touch;
    height: 100%;
    width: 100%;
    position: relative;
  }
</style>
