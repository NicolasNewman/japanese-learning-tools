<script lang="ts">
  import { get } from "$lib/persistence/mpvStore";
  import { mpvState } from "../../../stores/mpvState.svelte";

  const loadHistory = async () => {
    const history = await get("watch-history");
    console.log("History:", history);
    return history ?? {};
  };
</script>

{#await loadHistory() then history}
  <div class="space-y-2">
    <table>
      <thead>
        <tr>
          <th>File</th>
          <th>Timestamp</th>
          <th>Subtitle Offset</th>
        </tr>
      </thead>
      <tbody>
        {#each Object.entries(history).sort((a, b) => parseInt(b[0]) - parseInt(a[0])) as [watchHistoryId, item]}
          <tr
            on:click={() => {
              mpvState.watchHistoryId = parseInt(watchHistoryId);
              mpvState.mediaFile = item.path;
              mpvState.restorePoint = {
                timestamp: item.timestamp,
                subOffset: item.subOffset,
              };
            }}
          >
            <td>{item.path.split("/").pop()}</td>
            <td>{item.timestamp.toFixed(0)}</td>
            <td>{item.subOffset.toFixed(2)}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
{:catch error}
  <div class="text-red-500">Error loading history: {error.message}</div>
{/await}

<style>
  table {
    width: 100%;
    border-collapse: collapse;
  }

  th,
  td {
    border: 1px solid #ddd;
    padding: 8px;
    text-align: left;
  }

  tbody tr:nth-child(even) {
    /* rgb(20, 28, 42) / #1d293d */
    background-color: #172031;
  }

  tbody tr {
    cursor: pointer;
  }
  tbody tr:hover {
    background-color: #21324f;
  }
</style>
