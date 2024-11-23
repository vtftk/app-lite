<script lang="ts">
  import type { ThrowableConfig } from "$lib/api/types";
  import { invoke } from "@tauri-apps/api/core";

  type Props = {
    config: ThrowableConfig;
  };

  const { config }: Props = $props();

  async function testThrow() {
    await invoke("test_throw", {
      config,
      amount: 1,
    });
  }

  async function testThrowMany() {
    await invoke("test_throw", {
      config,
      amount: 10,
    });
  }
</script>

<div class="throwable">
  <div class="throwable__content">
    <div class="throwable__image-wrapper">
      <img class="throwable__image" src={config.image.src} alt="Throwable" />
    </div>
    <p class="throwable__name">{config.name}</p>
  </div>

  <div class="throwable__actions">
    <button class="throw-button" onclick={() => {}}>Edit</button>
    <button class="throw-button" onclick={() => {}}>Delete</button>
    <button class="throw-button" onclick={testThrow}>Throw</button>
    <button class="throw-button" onclick={testThrowMany}>Throw Many</button>
  </div>
</div>

<style>
  .throwable {
    background-color: #222;

    display: flex;
    flex-flow: column;
    gap: 0.75rem;

    padding: 1rem;
  }

  .throwable__content {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }

  .throwable__actions {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }

  .throwable__image {
    width: 3rem;
    height: 3rem;
    object-fit: contain;
    background-color: #333;
    border-radius: 2rem;
  }

  .throwable__name {
    color: #fff;
    font-weight: bold;
  }

  .throw-button {
    padding: 0.5rem 0.75rem;
    background-color: #333;
    border: 1px solid #666;
    color: #fff;
    border-radius: 0.25rem;
    cursor: pointer;
  }

  .throw-button:hover {
    background-color: #444;
  }
</style>
