<script lang="ts">
  import { createGetRedeemsList, refreshRedeemsList } from "$lib/api/twitch";

  type Props = {
    name: string;
    id: string;
  };

  const { name, id }: Props = $props();

  const redeemsList = createGetRedeemsList();
</script>

<button onclick={refreshRedeemsList}>Refresh Redeems</button>
<select {name} {id}>
  {#if $redeemsList.isLoading}
    <option value="" disabled selected>Loading...</option>
  {:else if $redeemsList.data}
    {#each $redeemsList.data as item}
      <option value={item.id}>{item.title}</option>
    {/each}
  {/if}
</select>
