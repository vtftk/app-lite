<script lang="ts">
  import { page } from "$app/stores";
  import type { Component } from "svelte";
  import type { SvelteHTMLElements } from "svelte/elements";

  type Props = {
    icon: Component<SvelteHTMLElements["svg"]>;
    href: string;
    title: string;
    text: string;
  };

  const { icon: Icon, href, title, text }: Props = $props();
</script>

<a {href} class="button" class:button--selected={$page.route.id == href}>
  <Icon class="icon" />
  <div class="content">
    <p class="title">{title}</p>
    <span class="text">{text}</span>
  </div>
</a>

<style>
  .button {
    display: block;
    padding: 0.5rem;
    border: 1px solid #333;
    text-decoration: none;
    background-color: #222;
    border-radius: 0.5rem;
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .content {
    display: flex;
    flex-flow: column;
    gap: 0.25rem;
  }

  .button:global(> .icon) {
    color: white;
    width: 1.5rem;
    height: 1.5rem;
  }

  .button:hover {
    background-color: #333;
  }

  .button--selected {
    background-color: #444;
  }

  .button--selected:hover {
    background-color: #555;
  }

  .title {
    color: #fff;
  }

  .text {
    color: #ccc;
    font-size: 0.8rem;
  }
</style>
