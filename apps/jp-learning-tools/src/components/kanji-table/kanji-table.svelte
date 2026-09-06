<script lang="ts" generics="TData extends RowData">
  import {
    type ColumnDef,
    type RowData,
    createTable,
    FlexRender,
  } from "@tanstack/svelte-table";
  import * as Table from "$lib/components/ui/table/index.js";
  import { features, type DataTableFeatures } from "./kanji-table-features.js";
  import { Input } from "$lib/components/ui/input/index.js";

  type DataTableProps<TData extends RowData> = {
    columns: ColumnDef<DataTableFeatures, TData>[];
    data: TData[];
  };

  let { data, columns }: DataTableProps<TData> = $props();

  const table = createTable({
    features,
    get data() {
      return data;
    },
    columns,
  });
  table.setPageSize(15);
</script>

<div class="flex h-full min-h-0 flex-col">
  <div class="flex items-center py-4">
    <Input
      placeholder="Filter vocab..."
      value={(table.getColumn("value")?.getFilterValue() as string) ?? ""}
      onchange={(e) => {
        table.getColumn("value")?.setFilterValue(e.currentTarget.value);
      }}
      oninput={(e) => {
        table.getColumn("value")?.setFilterValue(e.currentTarget.value);
      }}
      class="max-w-sm"
    />
  </div>
  <div class="min-h-0 flex-1 overflow-y-auto rounded-md border">
    <Table.Root>
      <Table.Header>
        {#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
          <Table.Row>
            {#each headerGroup.headers as header (header.id)}
              <Table.Head colspan={header.colSpan}>
                {#if !header.isPlaceholder}
                  <FlexRender {header} />
                {/if}
              </Table.Head>
            {/each}
          </Table.Row>
        {/each}
      </Table.Header>
      <Table.Body>
        {#each table.getRowModel().rows as row (row.id)}
          <Table.Row data-state={row.getIsSelected() && "selected"}>
            {#each row.getVisibleCells() as cell (cell.id)}
              <Table.Cell>
                <FlexRender {cell} />
              </Table.Cell>
            {/each}
          </Table.Row>
        {:else}
          <Table.Row>
            <Table.Cell colspan={columns.length} class="h-24 text-center">
              No results.
            </Table.Cell>
          </Table.Row>
        {/each}
      </Table.Body>
    </Table.Root>
  </div>
</div>
