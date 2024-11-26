<script lang="ts">
  import type { ItemConfig } from "$lib/api/types";
  import ThrowableItem from "./ThrowableItem.svelte";

  type Props = {
    items: ItemConfig[];
  };

  const { items }: Props = $props();
</script>

<div class="grid">
  <a class="create" href="/throwables/create"> Create Throwable </a>

  {#each items as item}
    <ThrowableItem config={item} />
  {/each}
</div>

<style>
  .grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.5rem;
    gap: 0.5rem;
  }

  .create {
    background-color: #222;

    display: flex;
    flex-flow: column;
    gap: 0.75rem;

    padding: 1rem;

    cursor: pointer;
    border: none;
    color: #fff;
    align-items: center;
    justify-content: center;
    font-size: 1em;
  }
</style>
