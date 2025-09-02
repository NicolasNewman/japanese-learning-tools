<script>
  import KanjiBank from "$lib/importer/kanji-bank";
  import { installManifest } from "$lib/manifest-installer";
  import createTitlebar from "$lib/menu";
  import "../app.css";
  import Alert from "../components/Alert.svelte";
  import { ModeWatcher } from "mode-watcher";

  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import AppSidebar from "$lib/components/app-sidebar.svelte";

  let { children } = $props();
  const titlebar = createTitlebar();

  const result = installManifest();
</script>

<ModeWatcher defaultMode="system" />
{#await result}
  <p>Loading manifest...</p>
{:then manifest}
  <p>Manifest installed: {JSON.stringify(manifest)}</p>
{:catch error}
  {console.error(error)}
  <p>Error installing manifest: {error}</p>
{/await}

<div class="mt-4">
  <Sidebar.Provider
    style="--sidebar-width: 8rem; --sidebar-width-mobile: 8rem;"
  >
    <AppSidebar />
    <main class="relative w-full">
      <Alert />
      <Sidebar.Trigger />
      {@render children?.()}
    </main>
  </Sidebar.Provider>
</div>
