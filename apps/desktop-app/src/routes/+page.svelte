<script lang="ts">
  import { goto } from "$app/navigation";
import {
    appDataDir,
    dataDir,
    resourceDir,
    executableDir,
    appLocalDataDir
  } from "@tauri-apps/api/path";
    import { Button } from "flowbite-svelte";
</script>

<main class="container">
  <h1>Welcome to Tauri + Svelte</h1>
  <Button color="primary" onclick={async () => await goto('/settings')}>
    Click Me
  </Button>

  {#await dataDir()}
    <p>Loading data directory...</p>
  {:then dir}
    <p>Data directory: {dir}</p>
  {:catch error}
    <p>Error loading data directory: {error.message}</p>
  {/await}
  {#await resourceDir()}
    <p>Loading resource directory...</p>
  {:then dir}
    <p>Resource directory: {dir}</p>
  {:catch error}
    <p>Error loading resource directory: {error.message}</p>
  {/await}
  {#await appDataDir()}
    <p>Loading appData directory...</p>
  {:then dir}
    <p>appData directory: {dir}</p>
  {:catch error}
    <p>Error loading appData directory: {error.message}</p>
  {/await}
  {#await appLocalDataDir()}
    <p>Loading appLocalDataDir directory...</p>
  {:then dir}
    <p>appLocalDataDir directory: {dir}</p>
  {:catch error}
    <p>Error loading appLocalDataDir directory: {error.message}</p>
  {/await}
  {#await executableDir()}
    <p>Loading executable directory...</p>
  {:then dir}
    <p>executable directory: {dir}</p>
  {:catch error}
    <p>Error loading executable directory: {error.message}</p>
  {/await}
</main>