import type {
  CommandId,
  Command,
  UpdateCommand,
  CreateCommand,
} from "$shared/dataV2";
import { createMutation, createQuery } from "@tanstack/svelte-query";
import { invoke } from "@tauri-apps/api/core";
import { queryClient } from "./utils";
import { derived, type Readable } from "svelte/store";

const COMMANDS_KEY = ["commands"];

export function getCommands() {
  return invoke<Command[]>("get_commands");
}

export function createCommandsQuery() {
  return createQuery({
    queryKey: COMMANDS_KEY,
    queryFn: getCommands,
  });
}

function createCommandKey(id: CommandId) {
  return ["command", id] as const;
}

export function createCommandQuery(id: CommandId | Readable<CommandId>) {
  if (typeof id === "string") {
    return createQuery({
      queryKey: createCommandKey(id),
      queryFn: () => getCommandById(id),
    });
  }

  // Create query derived from ID store
  return createQuery(
    derived(id, (id) => ({
      queryKey: createCommandKey(id),
      queryFn: () => getCommandById(id),
    }))
  );
}

export function getCommandById(soundId: CommandId) {
  return invoke<Command | null>("get_command_by_id", { soundId });
}

function createCommand(create: CreateCommand) {
  return invoke<Command>("create_command", { create });
}

export function createCommandMutation() {
  return createMutation<Command, Error, CreateCommand>({
    mutationFn: (createItem) => createCommand(createItem),

    onSuccess: (data) => {
      // Invalidate the specific sound query
      const soundKey = createCommandKey(data.id);
      queryClient.setQueryData(soundKey, data);
    },
    onSettled: (_data, _err, _createItem) => {
      // Invalid the list of sounds
      queryClient.invalidateQueries({ queryKey: COMMANDS_KEY });
    },
  });
}

export function bulkCreateCommandMutation() {
  return createMutation<Command[], Error, CreateCommand[]>({
    mutationFn: (createItems) => Promise.all(createItems.map(createCommand)),
    onSuccess: (sounds) => {
      for (const sound of sounds) {
        // Invalidate the specific sound query
        const soundKey = createCommandKey(sound.id);
        queryClient.setQueryData(soundKey, sound);
      }
    },
    onSettled: (_data, _err, _createSound) => {
      // Invalid the list of sounds
      queryClient.invalidateQueries({ queryKey: COMMANDS_KEY });
    },
  });
}

function updateCommand(commandId: CommandId, update: UpdateCommand["update"]) {
  return invoke<Command>("update_command", { commandId, update });
}

export function updateCommandMutation() {
  return createMutation<Command, Error, UpdateCommand>({
    mutationFn: (update) => updateCommand(update.commandId, update.update),
    onSuccess: (data) => {
      // Invalidate the specific item query
      const itemKey = createCommandKey(data.id);
      queryClient.setQueryData(itemKey, data);
    },
    onSettled: (_data, _err, _updateItem) => {
      // Invalid the list of items
      queryClient.invalidateQueries({ queryKey: COMMANDS_KEY });
    },
  });
}

function deleteCommand(commandId: CommandId) {
  return invoke<void>("delete_command", { commandId });
}

export function deleteCommandMutation() {
  return createMutation<void, Error, CommandId>({
    mutationFn: (commandId) => deleteCommand(commandId),
    onMutate: (commandId) => {
      const soundKey = createCommandKey(commandId);

      // Cancel any queries for the item and clear the current item data
      queryClient.cancelQueries({ queryKey: soundKey });
      queryClient.setQueryData(soundKey, undefined);

      return undefined;
    },
    onSettled: (_data, _err, itemId) => {
      // Invalidate the specific item query
      const soundKey = createCommandKey(itemId);
      queryClient.invalidateQueries({ queryKey: soundKey });

      // Invalid the list of items
      queryClient.invalidateQueries({ queryKey: COMMANDS_KEY });
    },
  });
}

type BulkDeleteCommands = {
  commandIds: CommandId[];
};

export function bulkDeleteCommandMutation() {
  return createMutation<void[], Error, BulkDeleteCommands>({
    mutationFn: (deleteSounds) =>
      Promise.all(deleteSounds.commandIds.map(deleteCommand)),
    onMutate: (deleteSounds) => {
      for (const commandId of deleteSounds.commandIds) {
        const soundKey = createCommandKey(commandId);

        // Cancel any queries for the item and clear the current item data
        queryClient.cancelQueries({ queryKey: soundKey });
        queryClient.setQueryData(soundKey, undefined);
      }

      return undefined;
    },
    onSettled: (_data, _err, deleteItems) => {
      for (const commandId of deleteItems.commandIds) {
        // Invalidate the specific item query
        const soundKey = createCommandKey(commandId);
        queryClient.invalidateQueries({ queryKey: soundKey });

        // Invalid the list of items
        queryClient.invalidateQueries({ queryKey: COMMANDS_KEY });
      }

      // Invalid the list of items
      queryClient.invalidateQueries({ queryKey: COMMANDS_KEY });
    },
  });
}
