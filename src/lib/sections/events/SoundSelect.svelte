<script lang="ts">
  import { createSoundsQuery } from "$lib/api/sounds";
  import FormSelect from "$lib/components/form/FormSelect.svelte";
  import type { SoundId } from "$shared/dataV2";
  import { derived } from "svelte/store";

  type Props = {
    id: string;
    name: string;
    label: string;
    description?: string;

    selected: SoundId;
    onChangeSelected: (value: SoundId) => void;
  };

  const { id, name, label, description, selected, onChangeSelected }: Props =
    $props();

  const soundsQuery = createSoundsQuery();

  const options = derived(soundsQuery, ($sounds) =>
    ($sounds.data ?? []).map((sound) => ({
      value: sound.id,
      label: sound.name,
    }))
  );

  type Option = (typeof $options)[0];
</script>

{#if $soundsQuery.isLoading}
  Loading sounds...
{/if}

{#snippet item(item: Option)}
  <div class="text-stack">
    <p class="text-stack--top">{item.label}</p>
  </div>
{/snippet}

<FormSelect
  {id}
  {name}
  {label}
  {description}
  items={$options}
  {item}
  {selected}
  {onChangeSelected}
/>
