<script lang="ts">
  import type { Snippet } from "svelte";

  type Props = {
    children?: Snippet;
  };

  const { children }: Props = $props();
</script>

<div class="sections">
  {@render children?.()}
</div>

<style>
  .sections {
    display: flex;
    flex-flow: column;
    gap: 1rem;
  }
</style>
