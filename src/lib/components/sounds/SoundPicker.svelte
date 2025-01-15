<script lang="ts">
  import type { Snippet } from "svelte";
  import type { Sound, SoundId } from "$shared/dataV2";

  import getBackendURL from "$lib/utils/url";
  import { getAppData } from "$lib/api/runtimeAppData";
  import { createSoundsQuery } from "$lib/api/soundModel";

  import Dialog from "../Dialog.svelte";
  import Button from "../input/Button.svelte";
  import SoundPreview from "./SoundPreview.svelte";
  import SearchInput from "../form/SearchInput.svelte";
  import DialogCloseButton from "../DialogCloseButton.svelte";
  import ControlledCheckbox from "../input/ControlledCheckbox.svelte";

  type Props = {
    disabled?: boolean;
    buttonContent?: Snippet;
    addButtonLabel?: string;

    title?: string;
    description: string;

    selected: SoundId[];
    onChangeSelected: (sounds: Sound[]) => void;
  };

  const {
    disabled,
    buttonContent,
    addButtonLabel = "Done",
    title: titleLabel = "Select Sounds",
    description: descriptionLabel,
    selected: initialSelected,
    onChangeSelected,
  }: Props = $props();

  let search = $state("");
  let selected: SoundId[] = $state([]);

  $effect(() => {
    selected = initialSelected;
  });

  const appData = getAppData();
  const soundsQuery = createSoundsQuery();
  const sounds = $derived(filterOptionsSearch($soundsQuery.data ?? [], search));

  function filterOptionsSearch(options: Sound[], search: string) {
    search = search.trim().toLowerCase();
    if (search.length < 1) return options;

    return options.filter((option) => {
      const name = option.name.trim().toLowerCase();
      return name.startsWith(search) || name.includes(search);
    });
  }

  const onSelectSound = (sound: Sound) => {
    if (selected.includes(sound.id)) {
      selected = selected.filter((id) => id !== sound.id);
    } else {
      selected = [...selected, sound.id];
    }
  };

  const onToggleAll = () => {
    if (sounds.length > 0 && selected.length === sounds.length) {
      selected = [];
    } else {
      selected = sounds.map((sound) => sound.id);
    }
  };

  const onSave = () => {
    onChangeSelected(sounds.filter((sound) => selected.includes(sound.id)));
  };
</script>

<Dialog>
  {#snippet button({ props })}
    <Button {...props} {disabled} type="button">
      {#if buttonContent}
        {@render buttonContent()}
      {:else}
        {selected.length > 0
          ? `${selected.length} Sounds selected`
          : "Select Sounds"}
      {/if}
    </Button>
  {/snippet}

  {#snippet title()}
    {titleLabel}
  {/snippet}

  {#snippet description()}
    {descriptionLabel}
  {/snippet}

  {#snippet children()}
    <div class="selection">
      <SearchInput bind:value={search} placeholder="Search" />
    </div>

    <div class="sound-table-wrapper">
      <table class="sound-table">
        <thead>
          <tr>
            <th class="sound-column sound-column--checkbox">
              <ControlledCheckbox
                id="terms"
                aria-labelledby="terms-label"
                checked={sounds.length > 0 && selected.length === sounds.length}
                onCheckedChange={onToggleAll}
              />
            </th>
            <th class="sound-column sound-column--name">Sound Name</th>
            <th class="sound-column sound-column--preview">Preview</th>
          </tr>
        </thead>
        <tbody>
          {#each sounds as sound (sound.id)}
            <tr class="sound-row">
              <td class="sound-column sound-column--checkbox">
                <ControlledCheckbox
                  id="terms"
                  aria-labelledby="terms-label"
                  checked={selected.includes(sound.id)}
                  onCheckedChange={() => onSelectSound(sound)}
                />
              </td>

              <td class="sound-column sound-column--name"> {sound.name} </td>

              <td class="sound-column sound-column--preview">
                <SoundPreview
                  volume={sound.volume * $appData.sounds_config.global_volume}
                  src={getBackendURL(sound.src)}
                />
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/snippet}

  {#snippet actions()}
    <DialogCloseButton buttonLabel={{ text: "Close" }} />
    <DialogCloseButton
      buttonLabel={{ text: addButtonLabel }}
      onclick={onSave}
    />
  {/snippet}
</Dialog>

<style>
  .selection {
    display: flex;
    gap: 1rem;
    align-items: center;
    padding-left: 1rem;
    padding-right: 1rem;
    padding-bottom: 1rem;
  }

  .sound-table-wrapper {
    max-height: 300px;
    min-height: 300px;
    overflow-y: auto;
    width: 100%;
  }

  .sound-table {
    width: 100%;
    border-collapse: collapse;
  }

  .sound-table tr {
    border: 1px solid #333;
  }

  .sound-table thead {
    position: sticky;
    top: -25px;
    z-index: 1;
    background-color: #111;
  }

  .sound-table td,
  .sound-table th {
    padding: 0.5rem 0.25rem;
  }

  .sound-table .sound-column--checkbox {
    padding-left: 1rem;
    padding-right: 0rem;
  }

  .sound-table .sound-column--preview {
    padding-right: 1rem;
  }

  .sound-table th {
    text-align: left;
    height: 2.5rem;
  }

  .sound-table td:nth-last-child(1),
  .sound-table th:nth-last-child(1) {
    text-align: right;
    display: flex;
    align-items: center;
    justify-content: flex-end;
  }
</style>
