<script lang="ts">
  import { goto } from "$app/navigation";
  import { Button } from "flowbite-svelte";
</script>

<main class="container">
  <h1>Welcome to Tauri + Svelte</h1>
  <Button color="primary" onclick={async () => await goto("/settings")}>
    Click Me
  </Button>
</main>
