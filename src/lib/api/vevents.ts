import type {
  EventId,
  VEvent as Event,
  UpdateEvent,
  CreateEvent,
} from "$shared/dataV2";
import { createMutation, createQuery } from "@tanstack/svelte-query";
import { invoke } from "@tauri-apps/api/core";
import { queryClient } from "./utils";
import { derived, type Readable } from "svelte/store";

const SCRIPTS_KEY = ["events"];

export function getEvents() {
  return invoke<Event[]>("get_events");
}

export function createEventsQuery() {
  return createQuery({
    queryKey: SCRIPTS_KEY,
    queryFn: getEvents,
  });
}

function createEventKey(id: EventId) {
  return ["event", id] as const;
}

export function createEventQuery(id: EventId | Readable<EventId>) {
  if (typeof id === "string") {
    return createQuery({
      queryKey: createEventKey(id),
      queryFn: () => getEventById(id),
    });
  }

  // Create query derived from ID store
  return createQuery(
    derived(id, (id) => ({
      queryKey: createEventKey(id),
      queryFn: () => getEventById(id),
    }))
  );
}

export function getEventById(soundId: EventId) {
  return invoke<Event | null>("get_event_by_id", { soundId });
}

function createEvent(create: CreateEvent) {
  return invoke<Event>("create_event", { create });
}

export function createEventMutation() {
  return createMutation<Event, Error, CreateEvent>({
    mutationFn: (createItem) => createEvent(createItem),

    onSuccess: (data) => {
      // Invalidate the specific sound query
      const soundKey = createEventKey(data.id);
      queryClient.setQueryData(soundKey, data);
    },
    onSettled: (_data, _err, _createItem) => {
      // Invalid the list of sounds
      queryClient.invalidateQueries({ queryKey: SCRIPTS_KEY });
    },
  });
}

export function bulkCreateEventMutation() {
  return createMutation<Event[], Error, CreateEvent[]>({
    mutationFn: (createItems) => Promise.all(createItems.map(createEvent)),
    onSuccess: (sounds) => {
      for (const sound of sounds) {
        // Invalidate the specific sound query
        const soundKey = createEventKey(sound.id);
        queryClient.setQueryData(soundKey, sound);
      }
    },
    onSettled: (_data, _err, _createSound) => {
      // Invalid the list of sounds
      queryClient.invalidateQueries({ queryKey: SCRIPTS_KEY });
    },
  });
}

function updateEvent(eventId: EventId, update: UpdateEvent["update"]) {
  return invoke<Event>("update_event", { eventId, update });
}

export function updateEventMutation() {
  return createMutation<Event, Error, UpdateEvent>({
    mutationFn: (update) => updateEvent(update.eventId, update.update),
    onSuccess: (data) => {
      // Invalidate the specific item query
      const itemKey = createEventKey(data.id);
      queryClient.setQueryData(itemKey, data);
    },
    onSettled: (_data, _err, _updateItem) => {
      // Invalid the list of items
      queryClient.invalidateQueries({ queryKey: SCRIPTS_KEY });
    },
  });
}

function deleteEvent(eventId: EventId) {
  return invoke<void>("delete_event", { eventId });
}

export function deleteEventMutation() {
  return createMutation<void, Error, EventId>({
    mutationFn: (eventId) => deleteEvent(eventId),
    onMutate: (eventId) => {
      const soundKey = createEventKey(eventId);

      // Cancel any queries for the item and clear the current item data
      queryClient.cancelQueries({ queryKey: soundKey });
      queryClient.setQueryData(soundKey, undefined);

      return undefined;
    },
    onSettled: (_data, _err, itemId) => {
      // Invalidate the specific item query
      const soundKey = createEventKey(itemId);
      queryClient.invalidateQueries({ queryKey: soundKey });

      // Invalid the list of items
      queryClient.invalidateQueries({ queryKey: SCRIPTS_KEY });
    },
  });
}

type BulkDeleteEvents = {
  eventIds: EventId[];
};

export function bulkDeleteEventMutation() {
  return createMutation<void[], Error, BulkDeleteEvents>({
    mutationFn: (deleteSounds) =>
      Promise.all(deleteSounds.eventIds.map(deleteEvent)),
    onMutate: (deleteSounds) => {
      for (const eventId of deleteSounds.eventIds) {
        const soundKey = createEventKey(eventId);

        // Cancel any queries for the item and clear the current item data
        queryClient.cancelQueries({ queryKey: soundKey });
        queryClient.setQueryData(soundKey, undefined);
      }

      return undefined;
    },
    onSettled: (_data, _err, deleteItems) => {
      for (const eventId of deleteItems.eventIds) {
        // Invalidate the specific item query
        const soundKey = createEventKey(eventId);
        queryClient.invalidateQueries({ queryKey: soundKey });

        // Invalid the list of items
        queryClient.invalidateQueries({ queryKey: SCRIPTS_KEY });
      }

      // Invalid the list of items
      queryClient.invalidateQueries({ queryKey: SCRIPTS_KEY });
    },
  });
}
