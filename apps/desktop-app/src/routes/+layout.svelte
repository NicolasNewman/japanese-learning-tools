<script>
    import KanjiBank from "$lib/importer/kanji-bank";
    import { installManifest } from "$lib/manifest-installer";
    import initMenu from "$lib/menu";
  let { children } = $props();
  import "../app.css";

  const result = installManifest();
  KanjiBank.setKanji("a", 2, "wanikani");
</script>
{#await result}
  <p>Loading manifest...</p>
{:then manifest}
  <p>Manifest installed: {JSON.stringify(manifest)}</p>
{:catch error}
  <p>Error installing manifest: {error}</p>
{/await}

{@render children()}