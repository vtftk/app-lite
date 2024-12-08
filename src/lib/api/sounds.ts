import type {
  CreateItem,
  CreateSound,
  Item,
  ItemId,
  ItemWithImpactSounds,
  Sound,
  SoundId,
  UpdateItem,
  UpdateSound,
} from "$shared/dataV2";
import { createMutation, createQuery } from "@tanstack/svelte-query";
import { invoke } from "@tauri-apps/api/core";
import { queryClient } from "./utils";
import { derived, type Readable } from "svelte/store";

const SOUNDS_KEY = ["sounds"];

export function createSoundsQuery() {
  return createQuery({
    queryKey: SOUNDS_KEY,
    queryFn: () => invoke<Sound[]>("get_sounds"),
  });
}

function createSoundKey(id: SoundId) {
  return ["sound", id] as const;
}

export function createSoundQuery(id: SoundId | Readable<SoundId>) {
  if (typeof id === "string") {
    return createQuery({
      queryKey: createSoundKey(id),
      queryFn: () => getSoundById(id),
    });
  }

  // Create query derived from ID store
  return createQuery(
    derived(id, (id) => ({
      queryKey: createSoundKey(id),
      queryFn: () => getSoundById(id),
    }))
  );
}

export function getSoundById(soundId: SoundId) {
  return invoke<Sound | null>("get_sound_by_id", { soundId });
}

function createSound(create: CreateSound) {
  return invoke<Sound>("create_sound", { create });
}

export function createSoundMutation() {
  return createMutation<Sound, Error, CreateSound>({
    mutationFn: (createItem) => createSound(createItem),

    onSuccess: (data) => {
      // Invalidate the specific sound query
      const soundKey = createSoundKey(data.id);
      queryClient.setQueryData(soundKey, data);
    },
    onSettled: (_data, _err, _createItem) => {
      // Invalid the list of sounds
      queryClient.invalidateQueries({ queryKey: SOUNDS_KEY });
    },
  });
}

export function bulkCreateSoundMutation() {
  return createMutation<Sound[], Error, CreateSound[]>({
    mutationFn: (createItems) => Promise.all(createItems.map(createSound)),
    onSuccess: (sounds) => {
      for (const sound of sounds) {
        // Invalidate the specific sound query
        const soundKey = createSoundKey(sound.id);
        queryClient.setQueryData(soundKey, sound);
      }
    },
    onSettled: (_data, _err, _createSound) => {
      // Invalid the list of sounds
      queryClient.invalidateQueries({ queryKey: SOUNDS_KEY });
    },
  });
}

function updateSound(soundId: SoundId, update: UpdateSound["update"]) {
  return invoke<Sound>("update_sound", { soundId, update });
}

export function updateSoundMutation() {
  return createMutation<Sound, Error, UpdateSound>({
    mutationFn: (update) => updateSound(update.soundId, update.update),
    onSuccess: (data) => {
      // Invalidate the specific item query
      const itemKey = createSoundKey(data.id);
      queryClient.setQueryData(itemKey, data);
    },
    onSettled: (_data, _err, _updateItem) => {
      // Invalid the list of items
      queryClient.invalidateQueries({ queryKey: SOUNDS_KEY });
    },
  });
}

function deleteSound(soundId: SoundId) {
  return invoke<void>("delete_sound", { soundId });
}

export function deleteSoundMutation() {
  return createMutation<void, Error, SoundId>({
    mutationFn: (soundId) => deleteSound(soundId),
    onMutate: (soundId) => {
      const soundKey = createSoundKey(soundId);

      // Cancel any queries for the item and clear the current item data
      queryClient.cancelQueries({ queryKey: soundKey });
      queryClient.setQueryData(soundKey, undefined);

      return undefined;
    },
    onSettled: (_data, _err, itemId) => {
      // Invalidate the specific item query
      const soundKey = createSoundKey(itemId);
      queryClient.invalidateQueries({ queryKey: soundKey });

      // Invalid the list of items
      queryClient.invalidateQueries({ queryKey: SOUNDS_KEY });
    },
  });
}

type BulkDeleteSounds = {
  soundIds: ItemId[];
};

export function bulkDeleteSoundsMutation() {
  return createMutation<void[], Error, BulkDeleteSounds>({
    mutationFn: (deleteSounds) =>
      Promise.all(deleteSounds.soundIds.map(deleteSound)),
    onMutate: (deleteSounds) => {
      for (const soundId of deleteSounds.soundIds) {
        const soundKey = createSoundKey(soundId);

        // Cancel any queries for the item and clear the current item data
        queryClient.cancelQueries({ queryKey: soundKey });
        queryClient.setQueryData(soundKey, undefined);
      }

      return undefined;
    },
    onSettled: (_data, _err, deleteItems) => {
      for (const soundId of deleteItems.soundIds) {
        // Invalidate the specific item query
        const soundKey = createSoundKey(soundId);
        queryClient.invalidateQueries({ queryKey: soundKey });

        // Invalid the list of items
        queryClient.invalidateQueries({ queryKey: SOUNDS_KEY });
      }

      // Invalid the list of items
      queryClient.invalidateQueries({ queryKey: SOUNDS_KEY });
    },
  });
}
