<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';

  async function startRegionCapture() {
    try {
      await invoke('start_region_select');
    } catch (error) {
      console.error('Failed to start region selector:', error);
    }
  }
</script>

<button on:click={startRegionCapture}>
  Select Region for OCR
</button>