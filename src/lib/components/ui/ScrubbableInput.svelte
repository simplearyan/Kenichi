<script lang="ts">
    // UI: Scrubbable Number Input
    let { value = $bindable(0) } = $props();
</script>

<div class="cursor-ew-resize select-none bg-black/20 px-2 py-1 rounded">
    {value}
</div>
