<script lang="ts">
  import SolarMinimalisticMagniferLinear from "~icons/solar/minimalistic-magnifer-linear";
  type Props = {
    value: string;
    placeholder?: string;

    type?: "text" | "password";
  };

  let { value = $bindable(), placeholder, type = "text" }: Props = $props();
</script>

<label class="form-input">
  <div class="icon"><SolarMinimalisticMagniferLinear /></div>
  <input {type} bind:value {placeholder} aria-describedby="{name}-validation" />
</label>

<style>
  .form-input {
    display: inline-flex;
    flex-flow: row;
    gap: 0.5rem;
    flex: auto;

    background-color: #222;
    border: 1px solid #333;
    align-items: center;
    gap: 0.5rem;
    position: relative;
    border-radius: 0.5rem;
  }

  .icon {
    position: absolute;
    left: 0.5rem;
  }

  .form-input input {
    padding: 0.5rem;
    padding-left: 2.5rem;
    color: #fff;
    border-radius: 0.25rem;
    background-color: transparent;
    border: none;
  }
</style>
