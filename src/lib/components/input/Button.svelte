<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";

  type Props = {
    children?: Snippet;
  } & HTMLButtonAttributes;

  const { children, ...props }: Props = $props();
</script>

<button type="button" class="btn" {...props}>
  {@render children?.()}
</button>

<style>
  .btn {
    padding: 0.5rem;
    background-color: #333;
    border: 1px solid #666;
    color: #fff;
    border-radius: 0.25rem;
    align-items: center;
    display: flex;
    gap: 0.5rem;
    cursor: pointer;
    font-size: 1em;
    text-decoration: none;
  }

  .btn:hover {
    background-color: #444;
    border-color: #777;
  }

  .btn:disabled,
  .btn:disabled:hover {
    background-color: #222;
    border: 1px solid #333;
    cursor: not-allowed;
    color: #777;
  }
</style>
