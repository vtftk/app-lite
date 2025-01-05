<script lang="ts">
  import type { Snippet } from "svelte";

  type Props = {
    title: string;
    description: string;

    beforeTitle?: Snippet;
    actions?: Snippet;
    beforeContent?: Snippet;
    children?: Snippet;
  };

  const {
    title,
    description,
    beforeTitle,
    actions,
    beforeContent,
    children,
  }: Props = $props();
</script>

<div class="container">
  <div class="title-area">
    {#if beforeTitle}
      <div class="actions-start">
        {@render beforeTitle()}
      </div>
    {/if}

    <div>
      <h1 class="title">{title}</h1>
      <p class="text">{description}</p>
    </div>

    <div class="actions">
      {@render actions?.()}
    </div>
  </div>

  {@render beforeContent?.()}

  <div class="content-area">
    {@render children?.()}
  </div>
</div>

<style>
  .container {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;

    padding: 1rem;
    height: 100%;
  }

  .title {
    color: #fff;
    margin-bottom: 0.25rem;
    line-height: 1;
    font-size: 1.75rem;
  }

  .text {
    color: #ccc;
  }

  .title-area {
    display: flex;
  }

  .actions-start {
    display: flex;
    justify-content: flex-end;
    gap: 1rem;
    align-items: center;
    margin-right: 1rem;
  }

  .actions {
    display: flex;
    flex: auto;
    justify-content: flex-end;
    gap: 1rem;
    align-items: center;
  }

  .content-area {
    flex: auto;
    overflow-y: auto;
  }
</style>
