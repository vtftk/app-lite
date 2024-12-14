import type {
  CommandId,
  Command,
  UpdateCommand,
  CreateCommand,
  LogsQuery,
  CommandLog,
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

export function getCommandById(commandId: CommandId) {
  return invoke<Command | null>("get_command_by_id", { commandId });
}

function createCommandLogsKey(id: CommandId, query: LogsQuery) {
  return ["command-logs", id, query] as const;
}

export function getCommandLogs(commandId: CommandId, query: LogsQuery) {
  return invoke<CommandLog[]>("get_command_logs", { commandId, query });
}

export function commandLogsQuery(commandId: CommandId, query: LogsQuery) {
  return createQuery({
    queryKey: createCommandLogsKey(commandId, query),
    queryFn: () => getCommandLogs(commandId, query),
  });
}

export function invalidateCommandLogs(commandId: CommandId, query: LogsQuery) {
  const queryKey = createCommandLogsKey(commandId, query);
  queryClient.invalidateQueries({ queryKey });
}

function createCommand(create: CreateCommand) {
  return invoke<Command>("create_command", { create });
}

export function createCommandMutation() {
  return createMutation<Command, Error, CreateCommand>({
    mutationFn: (createItem) => createCommand(createItem),

    onSuccess: (data) => {
      // Invalidate the specific command query
      const commandKey = createCommandKey(data.id);
      queryClient.setQueryData(commandKey, data);
    },
    onSettled: (_data, _err, _createItem) => {
      // Invalid the list of commands
      queryClient.invalidateQueries({ queryKey: COMMANDS_KEY });
    },
  });
}

export function bulkCreateCommandMutation() {
  return createMutation<Command[], Error, CreateCommand[]>({
    mutationFn: (createItems) => Promise.all(createItems.map(createCommand)),
    onSuccess: (commands) => {
      for (const command of commands) {
        // Invalidate the specific command query
        const commandKey = createCommandKey(command.id);
        queryClient.setQueryData(commandKey, command);
      }
    },
    onSettled: (_data, _err, _createSound) => {
      // Invalid the list of commands
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
      const commandKey = createCommandKey(commandId);

      // Cancel any queries for the item and clear the current item data
      queryClient.cancelQueries({ queryKey: commandKey });
      queryClient.setQueryData(commandKey, undefined);

      return undefined;
    },
    onSettled: (_data, _err, itemId) => {
      // Invalidate the specific item query
      const commandKey = createCommandKey(itemId);
      queryClient.invalidateQueries({ queryKey: commandKey });

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
        const commandKey = createCommandKey(commandId);

        // Cancel any queries for the item and clear the current item data
        queryClient.cancelQueries({ queryKey: commandKey });
        queryClient.setQueryData(commandKey, undefined);
      }

      return undefined;
    },
    onSettled: (_data, _err, deleteItems) => {
      for (const commandId of deleteItems.commandIds) {
        // Invalidate the specific item query
        const commandKey = createCommandKey(commandId);
        queryClient.invalidateQueries({ queryKey: commandKey });

        // Invalid the list of items
        queryClient.invalidateQueries({ queryKey: COMMANDS_KEY });
      }

      // Invalid the list of items
      queryClient.invalidateQueries({ queryKey: COMMANDS_KEY });
    },
  });
}
