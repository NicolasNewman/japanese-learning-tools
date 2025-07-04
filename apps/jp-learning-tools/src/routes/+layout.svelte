<script>
  import KanjiBank from "$lib/importer/kanji-bank";
  import { installManifest } from "$lib/manifest-installer";
  import createTitlebar from "$lib/menu";
  import "../app.css";
  import Alert from "../components/Alert.svelte";

  let { children } = $props();
  const titlebar = createTitlebar();

  const result = installManifest();
</script>

{#await result}
  <p>Loading manifest...</p>
{:then manifest}
  <p>Manifest installed: {JSON.stringify(manifest)}</p>
{:catch error}
  <p>Error installing manifest: {error}</p>
{/await}

<div class="relative mt-4">

  <Alert />
  {@render children()}
</div>
