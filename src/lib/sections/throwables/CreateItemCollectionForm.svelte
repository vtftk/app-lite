<script lang="ts">
  import { z } from "zod";
  import { Dialog } from "bits-ui";
  import { createForm } from "felte";
  import { toast } from "svelte-sonner";
  import reporterDom from "@felte/reporter-dom";
  import { fade, scale } from "svelte/transition";
  import { validator } from "@felte/validator-zod";
  import { toastErrorMessage } from "$lib/utils/error";
  import { type ItemCollectionWithItems } from "$lib/api/types";
  import FormTextInput from "$lib/components/form/FormTextInput.svelte";
  import ThrowablePicker from "$lib/components/throwable/ThrowablePicker.svelte";
  import {
    createItemCollection,
    setItemCollectionItems,
  } from "$lib/api/itemCollections";

  type Props = {
    existing?: ItemCollectionWithItems;
  };

  const { existing }: Props = $props();

  let open = $state(false);

  const schema = z.object({
    name: z.string().min(1, "You must specify a name"),
    item_ids: z.array(z.string()),
  });

  type Schema = z.infer<typeof schema>;

  // Defaults when creating a new sound
  const createDefaults: Partial<Schema> = {
    name: "",
    item_ids: [],
  };

  function createFromExisting(
    collection: ItemCollectionWithItems,
  ): Partial<Schema> {
    return {
      name: collection.name,
      item_ids: collection.items.map((item) => item.id),
    };
  }

  const { form, isValid, data, setFields } = createForm<z.infer<typeof schema>>(
    {
      // Derive initial values
      initialValues: existing ? createFromExisting(existing) : createDefaults,

      // Validation and error reporting
      extend: [validator({ schema }), reporterDom()],

      async onSubmit(values) {
        const savePromise = save(values);

        toast.promise(
          savePromise,
          existing
            ? {
                loading: "Saving collection...",
                success: "Saved collection",
                error: toastErrorMessage("Failed to save collection"),
              }
            : {
                loading: "Creating collection...",
                success: "Created collection",
                error: toastErrorMessage("Failed to create collection"),
              },
        );
      },
    },
  );

  async function save(values: Schema) {
    if (existing !== undefined) {
      // await updateItemCollection({
      //   soundId: existing.id,
      //   update: {
      //     name: values.name,
      //   },
      // });
      await setItemCollectionItems(existing.id, values.item_ids);
    } else {
      const collection = await createItemCollection({
        name: values.name,
      });
      await setItemCollectionItems(collection.id, values.item_ids);
    }
  }
</script>

<Dialog.Root {open} onOpenChange={(value) => (open = value)}>
  <Dialog.Trigger asChild let:builder>
    <button use:builder.action class="btn">
      {existing ? "Edit" : "Create Collection"}
    </button>
  </Dialog.Trigger>
  {#if open}
    <Dialog.Portal>
      <Dialog.Overlay transition={fade} transitionConfig={{ duration: 150 }} />
      <Dialog.Content transition={scale}>
        <Dialog.Title>
          {existing ? "Edit Collection" : "Create Collection"}
        </Dialog.Title>

        <form use:form class="form">
          <div class="content">
            <FormTextInput
              id="name"
              name="name"
              label="Name"
              placeholder="Hydration"
            />

            <ThrowablePicker
              selected={$data.item_ids}
              onChangeSelect={(selected) => {
                setFields("item_ids", selected, true);
              }}
            />
          </div>

          <div class="actions">
            <Dialog.Close class="btn">Cancel</Dialog.Close>
            <Dialog.Close class="btn" type="submit" disabled={!$isValid}>
              {existing ? "Save" : "Create"}
            </Dialog.Close>
          </div>
        </form>
      </Dialog.Content>
    </Dialog.Portal>
  {/if}
</Dialog.Root>

<style>
  .content {
    padding: 1rem 0rem;
  }
  .form {
    padding: 1rem;
  }

  .actions {
    display: flex;
    gap: 1rem;
    align-self: flex-end;
  }
</style>
