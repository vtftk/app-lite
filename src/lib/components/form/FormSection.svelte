<script lang="ts">
  import type { Snippet } from "svelte";

  import { slide } from "svelte/transition";

  type Props = {
    title?: string;
    description?: string;
    empty?: boolean;

    action?: Snippet;

    children?: Snippet;
  };

  const { title, description, empty, action, children }: Props = $props();
</script>

<section class="section">
  {#if title || description || action}
    <div class="section__head" class:section__head--no-content={empty}>
      {#if title || description}
        <div class="section__head__text">
          {#if title}
            <h2>{title}</h2>
          {/if}

          {#if description}
            <p>{description}</p>
          {/if}
        </div>
      {/if}

      {#if action}
        <div class="actions">
          {@render action?.()}
        </div>
      {/if}
    </div>
  {/if}

  {#if !empty}
    <div class="section__content" transition:slide={{ duration: 200 }}>
      {@render children?.()}
    </div>
  {/if}
</section>

<style>
  .section {
    display: flex;
    flex-flow: column;

    gap: 1rem;

    background-color: #1a1a1a;
    border: 1px solid #2f2f2f;
    padding: 1rem;
    border-radius: 0.5rem;
  }

  .section__head__text {
    flex: auto;
  }

  .section__head {
    display: flex;
    align-items: center;
    padding-bottom: 1rem;
    border-bottom: 1px solid #333;
  }

  .section__head--no-content {
    padding-bottom: 0;
    border-bottom: none;
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
