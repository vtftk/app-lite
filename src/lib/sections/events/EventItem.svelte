<script lang="ts">
  import type { VEvent } from "$lib/api/types";

  import { toast } from "svelte-sonner";
  import { deleteEvent } from "$lib/api/vevents";
  import { toastErrorMessage } from "$lib/utils/error";
  import SettingsIcon from "~icons/solar/settings-bold";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";
  import Button from "$lib/components/input/Button.svelte";
  import SolarMenuDotsBold from "~icons/solar/menu-dots-bold";
  import LinkButton from "$lib/components/input/LinkButton.svelte";
  import PopoverButton from "$lib/components/popover/PopoverButton.svelte";
  import ControlledCheckbox from "$lib/components/input/ControlledCheckbox.svelte";

  type Props = {
    config: VEvent;

    selected: boolean;
    onToggleSelected: VoidFunction;
  };

  const { config, selected, onToggleSelected }: Props = $props();

  async function onDelete() {
    if (!confirm("Are you sure you want to delete this event item?")) {
      return;
    }

    const deletePromise = deleteEvent(config.id);

    toast.promise(deletePromise, {
      loading: "Deleting event...",
      success: "Deleted event",
      error: toastErrorMessage("Failed to delete event"),
    });
  }
</script>

{#snippet popoverContent()}
  <LinkButton href="/events/{config.id}">
    <SettingsIcon /> View
  </LinkButton>
  <Button onclick={onDelete}><DeleteIcon /> Delete</Button>
{/snippet}

<div class="event">
  <ControlledCheckbox checked={selected} onCheckedChange={onToggleSelected} />

  <div class="event__content">
    <a title={config.name} href="/events/{config.id}" class="event__name">
      {config.name}
    </a>
  </div>

  <div class="action">
    <PopoverButton
      content={popoverContent}
      contentProps={{ align: "start", side: "left" }}
    >
      <SolarMenuDotsBold />
    </PopoverButton>
  </div>
</div>

<style>
  .event {
    background-color: #1a1a1a;
    border: 1px solid #2f2f2f;
    border-radius: 5px;

    display: flex;
    justify-content: flex-start;
    align-items: center;
    gap: 1rem;

    padding: 0.5rem;
    overflow: hidden;
  }

  .event__name {
    color: #fff;
    font-weight: bold;

    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
    text-decoration: none;
  }

  .event__name:hover {
    text-decoration: underline;
  }

  .event__content {
    display: flex;
    flex: auto;
    align-items: center;
    overflow: hidden;
  }

  .action {
    flex-shrink: 0;
  }
</style>
