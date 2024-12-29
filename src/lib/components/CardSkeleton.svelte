<script lang="ts">
  import type { HTMLAttributes } from "svelte/elements";

  import Card from "./Card.svelte";

  type Props = {} & HTMLAttributes<HTMLDivElement>;

  const props: Props = $props();
</script>

<Card {...props}>
  <div class="row">
    <div class="skeleton circle"></div>

    <div class="column">
      <div class="skeleton" style="width: 25%; height: 1.25rem"></div>
      <div class="skeleton" style="width: 60%; height: 1.25rem"></div>
    </div>
  </div>
</Card>

<style>
  .row {
    display: flex;
    gap: 1rem;
    flex-flow: row;
    align-items: center;
  }

  .circle {
    width: 4rem;
    height: 4rem;

    border-radius: 100%;
  }

  .column {
    flex: auto;
    display: flex;
    gap: 1rem;
    flex-flow: column;
  }
</style>
