<script lang="ts">
  import NavButton from "./NavButton.svelte";

  import SolarBasketballBoldDuotone from "~icons/solar/basketball-bold-duotone";
  import SolarBallsBoldDuotone from "~icons/solar/balls-bold-duotone";
  import SolarBellBoldDuotone from "~icons/solar/bell-bold-duotone";
  import SolarChatSquareCodeBoldDuotone from "~icons/solar/chat-square-code-bold-duotone";
  import SolarSettingsBoldDuotone from "~icons/solar/settings-bold-duotone";
  import SolarHeadphonesRoundSoundBoldDuotone from "~icons/solar/headphones-round-sound-bold-duotone";
</script>

<nav>
  <ul>
    <li>
      <NavButton
        icon={SolarBasketballBoldDuotone}
        href="/throwables"
        title="Throwables"
        text="Items that can be thrown"
      />
    </li>
    <li>
      <NavButton
        icon={SolarBallsBoldDuotone}
        href="/collections"
        title="Collections"
        text="Collections of throwables"
      />
    </li>
    <li>
      <NavButton
        icon={SolarBellBoldDuotone}
        href="/events"
        title="Events"
        text="Events that can trigger actions"
      />
    </li>
    <li>
      <NavButton
        icon={SolarHeadphonesRoundSoundBoldDuotone}
        href="/sounds"
        title="Sounds"
        text="Sounds that can be triggered"
      />
    </li>
    <li>
      <NavButton
        icon={SolarChatSquareCodeBoldDuotone}
        href="/scripts"
        title="Command Scripts"
        text="Custom command scripts"
      />
    </li>
    <li>
      <NavButton
        icon={SolarSettingsBoldDuotone}
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
    gap: 0.5em;
  }
</style>
