<script lang="ts">
  type Props = {
    name: string;
  };

  const { name }: Props = $props();
</script>

<p
  class="error-label"
  id="{name}-validation"
  data-felte-reporter-dom-for={name}
  aria-live="polite"
></p>

<style>
  .error-label {
    color: #c53939;
  }
</style>
