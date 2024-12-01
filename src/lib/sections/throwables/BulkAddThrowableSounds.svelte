<script lang="ts">
  import SoundPreview from "$lib/components/sounds/SoundPreview.svelte";
  import getBackendURL from "$lib/utils/url";
  import type { SoundConfig } from "$shared/appData";
  import { Checkbox, Dialog, Separator } from "bits-ui";
  import { fade, scale } from "svelte/transition";
  import SettingsIcon from "~icons/solar/settings-bold";

  type Props = {
    sounds: Readonly<SoundConfig[]>;
    onSubmit: (sounds: SoundConfig[]) => void;
  };

  const { sounds, onSubmit }: Props = $props();

  let selected: string[] = $state([]);

  const isAllSelected = $derived(selected.length === sounds.length);

  const onSelectSound = (sound: SoundConfig) => {
    if (selected.includes(sound.id)) {
      selected = selected.filter((id) => id !== sound.id);
    } else {
      selected = [...selected, sound.id];
    }
  };

  const onToggleAll = () => {
    if (isAllSelected) {
      selected = [];
    } else {
      selected = sounds.map((sound) => sound.id);
    }
  };

  const onSave = () => {
    onSubmit(sounds.filter((sound) => selected.includes(sound.id)));
  };
</script>

<Dialog.Root>
  <Dialog.Trigger class="btn">
    <SettingsIcon />Add Impact Sounds
  </Dialog.Trigger>
  <Dialog.Portal>
    <Dialog.Overlay transition={fade} transitionConfig={{ duration: 150 }} />
    <Dialog.Content transition={scale}>
      <Dialog.Title>Select Sounds</Dialog.Title>

      <Dialog.Description class="text-sm text-foreground-alt">
        Choose which impact sounds you'd like to add the the selected
        throwables.
      </Dialog.Description>

      <Separator.Root />

      <div class="sound-table-wrapper">
        <table class="sound-table">
          <thead>
            <tr>
              <th class="sound-column sound-column--checkbox">
                <Checkbox.Root
                  id="terms"
                  aria-labelledby="terms-label"
                  checked={isAllSelected}
                  onCheckedChange={onToggleAll}
                >
                  <Checkbox.Indicator let:isChecked>
                    {#if isChecked}
                      <span>&#10003;</span>
                    {/if}
                  </Checkbox.Indicator>
                </Checkbox.Root>
              </th>
              <th class="sound-column sound-column--name">Sound Name</th>
              <th class="sound-column sound-column--preview">Preview</th>
            </tr>
          </thead>
          <tbody>
            {#each sounds as sound (sound.id)}
              <tr class="sound-row">
                <td class="sound-column sound-column--checkbox">
                  <Checkbox.Root
                    id="terms"
                    aria-labelledby="terms-label"
                    checked={selected.includes(sound.id)}
                    onCheckedChange={() => onSelectSound(sound)}
                  >
                    <Checkbox.Indicator let:isChecked>
                      {#if isChecked}
                        <span>&#10003;</span>
                      {/if}
                    </Checkbox.Indicator>
                  </Checkbox.Root>
                </td>

                <td class="sound-column sound-column--name"> {sound.name} </td>

                <td class="sound-column sound-column--preview">
                  <SoundPreview src={getBackendURL(sound.src)} />
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>

      <div data-dialog-actions>
        <Dialog.Close>
          <span class="sr-only">Cancel</span>
        </Dialog.Close>
        <Dialog.Close onclick={onSave}>
          <span class="sr-only">Save</span>
        </Dialog.Close>
      </div>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>

<style>
  .sound-table-wrapper {
    padding: 1rem;
    max-height: 300px;
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
