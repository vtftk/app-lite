<script lang="ts">
  import { createGetRedeemsList, refreshRedeemsList } from "$lib/api/twitch";
  import FormSelect from "$lib/components/form/FormSelect.svelte";
  import SolarRefreshBold from "~icons/solar/refresh-bold";

  type Props = {
    id: string;
    name: string;
    label: string;

    selected: any;
    onChangeSelected: (value: any) => void;
  };

  const { id, label, name, selected, onChangeSelected }: Props = $props();

  const redeemsList = createGetRedeemsList();

  const items = $derived(
    ($redeemsList.data ?? []).map((item) => ({
      value: item.id,
      label: item.title,
      description: item.prompt,
    }))
  );
</script>

{#snippet twitchRedeemItem(item: any)}
  <div class="text-stack item">
    <p class="text-stack--top">{item.label}</p>
    <p class="text-stack--bottom">{item.description}</p>
  </div>
{/snippet}

<div class="container">
  <FormSelect
    {id}
    {name}
    {label}
    {items}
    item={twitchRedeemItem}
    {selected}
    {onChangeSelected}
  />

  <button type="button" class="btn" onclick={refreshRedeemsList}>
    <SolarRefreshBold />
    Refresh Redeems
  </button>
</div>

{#if $redeemsList.isLoading}
  Loading...
{/if}

<style>
  .container {
    display: flex;
    gap: 1rem;
    align-items: flex-end;
    width: 100%;
  }

  .container :global(.form-input) {
    flex: auto;
  }
  .container :global(.form-input [data-select-trigger]) {
    height: 2.65rem;
  }
</style>
