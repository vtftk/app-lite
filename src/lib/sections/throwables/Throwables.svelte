<script lang="ts">
  import type { ItemConfig } from "$lib/api/types";
  import BulkThrowableImport from "$lib/components/throwable/BulkThrowableImport.svelte";
  import ThrowableItem from "./ThrowableItem.svelte";

  type Props = {
    items: ItemConfig[];
  };

  const { items }: Props = $props();
</script>

<div class="grid">
  <a class="btn" href="/throwables/create"> Create Throwable </a>

  <BulkThrowableImport />

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
</style>
