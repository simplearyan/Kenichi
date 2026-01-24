<script lang="ts">
    // Keybindings Modal Component
    let { open = false } = $props();
</script>

{#if open}
    <div
        class="fixed inset-0 bg-black/50 z-50 flex items-center justify-center"
    >
        <div class="bg-kenichi-panel p-4 rounded border border-white/10">
            Keyboard Shortcuts (TODO)
        </div>
    </div>
{/if}
