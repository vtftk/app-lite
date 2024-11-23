<script lang="ts">
  import type { ThrowableConfig } from "$lib/api/types";
  import CreateThrowable from "./CreateThrowable.svelte";
  import ThrowableItem from "./ThrowableItem.svelte";

  type Props = {
    items: ThrowableConfig[];
  };

  const { items }: Props = $props();

  let creating = $state(false);
</script>

{#if creating}
  <CreateThrowable
    onClose={() => {
      creating = false;
    }}
  />
{:else}
  <div class="grid">
    <button
      class="create"
      onclick={() => {
        creating = true;
      }}
    >
      Create Throwable
    </button>

    {#each items as item}
      <ThrowableItem config={item} />
    {/each}
  </div>
{/if}

<style>
  .grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
    padding: 1rem;
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
