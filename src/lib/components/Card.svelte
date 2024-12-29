<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLAttributes } from "svelte/elements";

  type Props = {
    children?: Snippet;
  } & HTMLAttributes<HTMLDivElement>;

  const { children, ...props }: Props = $props();
</script>

<div {...props} class="card">
  {@render children?.()}
</div>

<style>
  .card {
    background-color: #1a1a1a;
    border: 1px solid #2f2f2f;
    padding: 1rem;
    border-radius: 0.5rem;
  }
</style>
