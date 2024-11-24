<script lang="ts">
  import NavButton from "./NavButton.svelte";
</script>

<nav>
  <ul>
    <li>
      <NavButton
        href="/throwables"
        title="Throwables"
        text="Items that can be thrown"
      />
    </li>
    <li>
      <NavButton
        href="/collections"
        title="Collections"
        text="Collections of throwables"
      />
    </li>
    <li>
      <NavButton
        href="/events"
        title="Events"
        text="Events that can trigger actions"
      />
    </li>
    <li>
      <NavButton
        href="/scripts"
        title="Scripts"
        text="Custom scripts that can run"
      />
    </li>
    <li>
      <NavButton
        href="/settings"
        title="Settings"
        text="Configuration options"
      />
    </li>
  </ul>
</nav>

<style>
  nav > ul {
    display: flex;
    flex-flow: column;
    gap: 1rem;
  }
</style>
