<script lang="ts">
  import { Label } from "$lib/components/ui/label/index.js";
  import { Textarea } from "$lib/components/ui/textarea/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { translateJpEn } from "$lib/commands";
  import Loader2Icon from "@lucide/svelte/icons/loader-2";

  let jpText = $state("");
  let enText = $state("");
  let isProcessing = $state(false);
</script>

<div class="h-[calc(100vh-30px)] p-4 flex flex-col">
  <div class="flex flex-col w-full h-full gap-1.5 mb-3">
    <div class="flex h-[calc(100%-20px)] gap-3">
      <div class="w-1/2">
        <Label for="jp-text" class="mb-2">Japanese</Label>
        <Textarea
          id="jp-text"
          name="jp-text"
          class="h-full"
          placeholder="Enter Japanese text"
          bind:value={jpText}
        />
      </div>
      <div class="w-1/2">
        <Label for="en-text" class="mb-2">English</Label>
        <Textarea
          id="en-text"
          name="en-text"
          class="h-full"
          disabled={true}
          bind:value={enText}
        />
      </div>
    </div>
    <p class="text-muted-foreground text-sm mt-5">
      Translations done locally using opus-mt-ja-en from Helsinki-NLP
    </p>
  </div>
  <Button
    disabled={isProcessing}
    class="w-32"
    onclick={() => {
      isProcessing = true;
      translateJpEn(jpText)
        .then((res) => {
          enText = res;
          isProcessing = false;
          console.log(res);
        })
        .catch((err) => {
          enText = err;
          isProcessing = false;
          console.error(err);
        });
    }}
  >
    {#if isProcessing}
      <Loader2Icon class="animate-spin" />
    {/if}
    Translate
  </Button>
</div>
