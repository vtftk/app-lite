<script lang="ts">
  type Props = {
    href: string;
    title: string;
    text: string;
  };

  const { href, title, text }: Props = $props();
</script>

<a {href} class="button">
  <p class="title">{title}</p>
  <span class="text">{text}</span>
</a>

<style>
  .button {
    display: block;
    padding: 0.5rem;
    border: 2px solid #fff;
    text-decoration: none;
    background-color: #222;
  }

  .button:hover {
    background-color: #444;
  }

  .title {
    color: #fff;
  }

  .text {
    color: #ccc;
  }
</style>
