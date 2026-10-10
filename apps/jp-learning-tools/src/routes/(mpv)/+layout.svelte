<script>
    let { children } = $props();
</script>

<style>
:global(html),
:global(body) {
  background: transparent;
}
</style>

{@render children?.()}