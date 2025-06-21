<script lang="ts">
  import { Alert, type AlertProps } from "flowbite-svelte";
  import { InfoCircleSolid } from "flowbite-svelte-icons";
  import { alertState, type AlertType } from "../stores/alertState.svelte";

  $effect(() => {
    if (alertState.alert !== null) {
      setTimeout(() => {
        alertState.alert = null;
      }, 5000);
    }
  });

  const alertTypeToColor: Record<AlertType, AlertProps["color"]> = {
    info: "blue",
    success: "green",
    warning: "yellow",
    error: "red",
  };
</script>

{#if alertState.alert !== null}
  <div class="absolute top-0 left-0 w-full">
    <Alert dismissable onclick={() => (alertState.alert = null)} color={alertTypeToColor[alertState.alert.alertType]}>
      {#snippet icon()}
        <InfoCircleSolid class="h-5 w-5" />
      {/snippet}
      <span class="font-medium">{alertState.alert.alertTitle}:</span>
      {alertState.alert.alertMessage}
    </Alert>
  </div>
{/if}
