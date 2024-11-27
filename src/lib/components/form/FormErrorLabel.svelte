<script lang="ts">
  type Props = {
    name: string;
  };

  const { name }: Props = $props();
</script>

<ul
  class="error-label"
  id="{name}-validation"
  data-felte-reporter-dom-for={name}
  aria-live="polite"
></ul>

<style>
  .error-label {
    color: #c53939;
  }

  .error-label {
    list-style: circle;
    list-style-position: inside;
  }
</style>
