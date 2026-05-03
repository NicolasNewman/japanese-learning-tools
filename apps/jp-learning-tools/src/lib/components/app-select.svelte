<script lang="ts">
  import * as Select from "$lib/components/ui/select/index.js";

  const {
    options,
    name,
    value = $bindable(),
    disabled = false,
    onValueChange = () => {},
  }: {
    options: { value: string; label: string }[];
    name: string;
    value?: string;
    disabled?: boolean;
    onValueChange?: (value: string) => void;
  } = $props();

  const triggerContent = $derived(
    options.find((f) => f.value === value)?.label ?? "Field",
  );
</script>

<Select.Root type="single" {name} {value} {disabled} {onValueChange}>
  <Select.Trigger class="w-[180px]">
    {triggerContent}
  </Select.Trigger>
  <Select.Content>
    <Select.Group>
      <Select.Label>Fruits</Select.Label>
      {#each options as option (option.value)}
        <Select.Item value={option.value} label={option.label}>
          {option.label}
        </Select.Item>
      {/each}
    </Select.Group>
  </Select.Content>
</Select.Root>
