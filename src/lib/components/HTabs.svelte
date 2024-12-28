<script lang="ts">
  import type { Snippet, Component } from "svelte";

  import { Tabs } from "bits-ui";

  type Props = {
    tabs: Tab[];
  };

  type Tab = {
    value: string;
    icon?: Component;
    label: string;
    content: Snippet;

    // Disable padding around the content
    disablePadding?: boolean;
  };

  const { tabs }: Props = $props();

  const firstTabValue = $derived(tabs.length > 0 ? tabs[0].value : undefined);

  let value: string = $state("");

  $effect(() => {
    value = firstTabValue ?? "";
  });
</script>

<Tabs.Root bind:value>
  {#snippet child({ props })}
    <div {...props} class="root">
      <Tabs.List>
        {#each tabs as tab (tab.value)}
          <Tabs.Trigger value={tab.value}>
            {#if tab.icon}
              <tab.icon />
            {/if}
            {tab.label}
          </Tabs.Trigger>
        {/each}
      </Tabs.List>
      {#each tabs as tab (tab.value)}
        <Tabs.Content value={tab.value}>
          {#snippet child({ props })}
            {#if value === tab.value}
              <div
                {...props}
                class="content"
                class:content--disable-padding={tab.disablePadding}
              >
                {@render tab.content()}
              </div>
            {/if}
          {/snippet}
        </Tabs.Content>
      {/each}
    </div>
  {/snippet}
</Tabs.Root>

<style>
  .root {
    height: 100%;
    display: flex;
    flex-flow: column;
  }

  .content {
    position: relative;
    flex: auto;
    overflow: auto;
    flex-flow: column;
    border: 1px solid #333;
    padding: 1rem;
  }

  .content--disable-padding {
    padding: 0;
  }
</style>
