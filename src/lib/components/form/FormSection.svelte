<script lang="ts">
  import type { Snippet } from "svelte";

  type Props = {
    title?: string;
    description?: string;

    children?: Snippet;
  };

  const { title, description, children }: Props = $props();
</script>

<section class="section">
  {#if title || description}
    <div class="section__head">
      {#if title}
        <h2>{title}</h2>
      {/if}

      {#if description}
        <p>{description}</p>
      {/if}
    </div>
  {/if}

  <div class="section__content">
    {@render children?.()}
  </div>
</section>

<style>
  .section {
    display: flex;
    flex-flow: column;

    border: 1px solid #333;
    padding: 1rem;
    gap: 1rem;
  }

  .section__head {
    padding-bottom: 1rem;
    border-bottom: 1px solid #333;
  }

  .section__head h2 {
    color: #fff;
    font-size: 1.25rem;
    margin-bottom: 0.25rem;
  }

  .section__head p {
    color: #ccc;
  }

  .section__content {
    display: flex;
    flex-flow: column;
    gap: 1rem;
  }
</style>
