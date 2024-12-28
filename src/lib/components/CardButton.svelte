<script lang="ts">
  import type { Snippet, Component } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";

  type Props = {
    icon: Component;
    color?: string;
    label: string;
    description?: string;
    selected?: boolean;

    content?: Snippet;
    contentVisible?: boolean;
  } & HTMLButtonAttributes;

  const {
    icon: Icon,
    color,
    label,
    description,
    selected = false,
    content,
    contentVisible,
    ...buttonProps
  }: Props = $props();
</script>

<button
  {...buttonProps}
  type="button"
  class="item"
  data-item-color={color}
  class:item--selected={selected}
>
  <div class="item-top">
    <div class="item__icon">
      <Icon />
    </div>
    <div class="item__content">
      <h3 class="item__title">{label}</h3>

      {#if description}
        <p class="item__text">{description}</p>
      {/if}
    </div>
  </div>

  {#if contentVisible && content}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="item__content__inner"
      onclick={(event) => {
        event.stopPropagation();
      }}
    >
      {@render content()}
    </div>
  {/if}
</button>

<style>
  .item {
    background-color: #222;
    border: 2px solid #111;
    color: #ccc;
    text-align: left;

    display: flex;
    flex-flow: column;
    padding: 0.75rem;
    border: 2px solid #333;

    transition: all 0.25s;
    cursor: pointer;
    gap: 0.5rem;
    border-radius: 0.25rem;
  }

  .item-top {
    display: flex;
    flex-flow: row;
    align-items: flex-start;
    justify-items: flex-start;
    gap: 1rem;
  }

  .item--selected[data-item-color="purple"] {
    border-color: #dd82f0;
    background-color: #3c1b42;
  }

  .item[data-item-color="purple"] .item__icon {
    color: #dd82f0;
  }

  .item--selected[data-item-color="red"] {
    border-color: #f08282;
    background-color: #421b1b;
  }

  .item[data-item-color="red"] .item__icon {
    color: #f08282;
  }

  .item--selected[data-item-color="yellow"] {
    border-color: #eef082;
    background-color: #423f1b;
  }

  .item[data-item-color="yellow"] .item__icon {
    color: #f0ee82;
  }

  .item--selected[data-item-color="green"] {
    border-color: #a1f082;
    background-color: #1b421b;
  }

  .item[data-item-color="green"] .item__icon {
    color: #a1f082;
  }

  .item--selected[data-item-color="blue"] {
    border-color: #82bbf0;
    background-color: #1b2f42;
  }

  .item[data-item-color="blue"] .item__icon {
    color: #82bbf0;
  }

  .item--selected {
    border-color: #777;
    background-color: #444;
  }

  .item__icon {
    margin: 0.5rem auto;
    font-size: 2rem;
    line-height: 1;
  }

  .item__title {
    margin-bottom: 0.25rem;
    color: #fff;
    font-size: 1rem;
  }

  .item__text {
    font-size: 0.9rem;
  }

  .item__content {
    flex: auto;
  }

  .item__content__inner {
    padding: 1rem;
    background-color: #222;
    border: 1px solid #444;
    border-radius: 0.125rem;
    flex: auto;
    width: 100%;
    display: flex;
    flex-flow: column;
    cursor: default;
    gap: 0.5rem;
  }
</style>
