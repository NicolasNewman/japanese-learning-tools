<script>
  import KanjiBank from "$lib/importer/kanji-bank";
  import { installManifest } from "$lib/manifest-installer";
  import createTitlebar from "$lib/menu";
  import "../../app.css";
  import Alert from "../../components/Alert.svelte";
  import { ModeWatcher } from "mode-watcher";

  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import AppSidebar from "$lib/components/app-sidebar.svelte";
  import { statusState } from "../../stores/statusState.svelte";

  let { children } = $props();
  const titlebar = createTitlebar();

  installManifest()
    .then((manifest) => {
      statusState.manifestStatus = "success";
      return manifest;
    })
    .catch((error) => {
      statusState.manifestStatus = "error";
      statusState.manifestError = error.message;
    });
  statusState.manifestStatus = "loading";
</script>

<ModeWatcher defaultMode="system" />
<div class="mt-[30px] h-[calc(100vh-30px)] overflow-y-hidden">
  <Sidebar.Provider
    style="--sidebar-width: 8rem; --sidebar-width-mobile: 8rem;"
    class="h-[calc(100vh-30px)]"
  >
    <AppSidebar />
    <main class="relative w-full">
      <Alert />
      <!-- <Sidebar.Trigger /> -->
      {@render children?.()}
    </main>
  </Sidebar.Provider>
</div>
