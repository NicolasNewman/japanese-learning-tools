<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { command } from "tauri-plugin-libmpv-api";

  export let isRunning: boolean;

  let delay: number = 0;

  const adjustTimings = async (delay: number) => {
    console.log("Adjusting timings...");
    await command("add", ["sub-delay", delay]);
  };
</script>

<div class="space-y-8">
  <div class="flex items-center">
    <span class="mr-2 min-w-1/3"
      >{delay >= 0 ? "Increase" : "Decrease"} timings by {Math.abs(
        delay,
      )}s</span
    >
    <Input type="number" class="mr-2 w-16" bind:value={delay} />
    <Button onclick={() => adjustTimings(delay)} disabled={!isRunning}>
      Adjust timings
    </Button>
  </div>
</div>
