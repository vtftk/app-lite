<script lang="ts">
  import type { Snippet } from "svelte";

  type Props = {
    color?: "red" | "green" | "blue" | "yellow" | "purple";
    size?: "small" | "medium" | "large";
    children?: Snippet;
  };

  const { color = "green", size = "small", children }: Props = $props();
</script>

<span class="label" data-color={color} data-size={size}>
  {@render children?.()}
</span>

<style>
  .label {
    display: inline-block;
    margin: 0 0.2rem;
    background-color: #222;
    border: 1px solid #111;
    color: #ccc;
    font-size: 0.9rem;
    font-weight: normal;

    border-radius: 0.25rem;
    padding: 0.25rem 0.75rem;

    vertical-align: middle;
  }

  .label[data-size="small"] {
    font-size: 0.8rem;
    padding: 0.15rem 0.5rem;
  }

  .label[data-color="purple"] {
    border-color: #dd82f0;
    background-color: #3c1b42;
    color: #dd82f0;
  }

  .label[data-color="red"] {
    border-color: #f08282;
    background-color: #421b1b;
    color: #f08282;
  }

  .label[data-color="yellow"] {
    border-color: #eef082;
    background-color: #423f1b;
    color: #f0ee82;
  }

  .label[data-color="green"] {
    border-color: #a1f082;
    background-color: #1b421b;
    color: #a1f082;
  }

  .label[data-color="blue"] {
    border-color: #82bbf0;
    background-color: #1b2f42;
    color: #82bbf0;
  }
</style>
