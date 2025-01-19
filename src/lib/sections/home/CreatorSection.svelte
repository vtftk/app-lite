<script lang="ts">
  import { getVersion } from "@tauri-apps/api/app";
</script>

<div class="block">
  <img
    class="avatar"
    width="64"
    height="64"
    src="/avatar-64x64.png"
    alt="Jacob Avatar"
  />
  <div class="creator-text">
    <p>
      Created by
      <a class="creator" href="https://github.com/jacobtread" target="_blank">
        Jacobtread
      </a>
    </p>
    <p class="version">
      Version:
      {#await getVersion() then version}
        {version}
      {/await}
    </p>
  </div>
</div>

<style>
  .avatar {
    display: inline-block;
    vertical-align: middle;
  }

  .block {
    display: flex;
    align-items: center;
  }

  .version {
    font-size: 0.8rem;
    color: #999;
  }

  .creator {
    color: #e66c6c;
    font-weight: bold;
  }
</style>
