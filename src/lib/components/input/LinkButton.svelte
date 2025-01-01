<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLAnchorAttributes } from "svelte/elements";

  type Props = {
    variant?: "default" | "warning" | "error";
    children?: Snippet;
  } & HTMLAnchorAttributes;

  const { variant, children, ...props }: Props = $props();
</script>

<a class="btn" {...props} data-variant={variant}>
  {@render children?.()}
</a>

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

  .btn[data-variant="warning"] {
    border-color: #f0c082;
    background-color: #422d1b;
    color: #f0c482;
  }

  .btn[data-variant="error"] {
    border-color: #f08282;
    background-color: #421b1b;
    color: #f08282;
  }

  .btn:hover {
    background-color: #444;
    border-color: #777;
  }

  .btn[data-variant="warning"]:hover {
    border-color: #f0c792;
    background-color: #775538;
    color: #f5d6a7;
  }

  .btn:disabled,
  .btn:disabled:hover {
    background-color: #222;
    border: 1px solid #333;
    cursor: not-allowed;
    color: #777;
  }
</style>
