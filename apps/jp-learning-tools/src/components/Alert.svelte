<script lang="ts">
  import { alertState, type AlertType } from "../stores/alertState.svelte";
  import * as Alert from "$lib/components/ui/alert/index.js";
  import CircleAlertIcon from "@lucide/svelte/icons/circle-alert";

  $effect(() => {
    if (alertState.alert !== null) {
      setTimeout(() => {
        alertState.alert = null;
      }, 8000);
    }
  });
  // TODO: handle with shadcn
  const alertTypeToColor: Record<AlertType, string> = {
    info: "blue",
    success: "green",
    warning: "yellow",
    error: "red",
  };
</script>

{#if alertState.alert !== null}
  <div class="absolute top-0 left-0 w-full flex justify-center">
    <Alert.Root class="w-fit min-w-1/2">
      <CircleAlertIcon class="size-4" />
      <Alert.Title>{alertState.alert.alertTitle}</Alert.Title>
      <Alert.Description>
        {alertState.alert.alertMessage}
      </Alert.Description>
    </Alert.Root>
  </div>
{/if}
