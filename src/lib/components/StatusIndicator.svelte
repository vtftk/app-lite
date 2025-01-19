<script lang="ts" module>
  export type StatusColor = "green" | "orange" | "red";
</script>

<script lang="ts">
  import type { Snippet } from "svelte";

  type Props = {
    status: StatusColor;
    children?: Snippet;
  };

  const { status, children }: Props = $props();
</script>

<div class="indicator" data-status={status}>
  {@render children?.()}
</div>

<style>
  .indicator {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    color: #fff;
    width: 2rem;
    height: 2rem;
    background-color: black;
    border-radius: 1rem;
    flex-shrink: 0;
  }

  .indicator[data-status="green"] {
    background-color: green;
  }

  .indicator[data-status="orange"] {
    background-color: orange;
  }

  .indicator[data-status="red"] {
    background-color: red;
  }
</style>
