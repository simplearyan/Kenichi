<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    let backendReady = $state(false);

    onMount(async () => {
        try {
            console.log("Requesting WGPU surface attachment...");
            backendReady = await invoke("attach_wgpu_renderer");
            console.log("WGPU attachment result:", backendReady);
        } catch (error) {
            console.error("Failed to attach WGPU backend:", error);
        }
    });
</script>

<div class="flex-1 flex items-center justify-center relative">
    <div
        class="w-full h-full flex items-center justify-center {backendReady
            ? 'bg-transparent'
            : 'bg-#080808'}"
    >
        {#if !backendReady}
            <span class="text-[10px] text-brand-accent/20 font-mono"
                >INITIALIZING_RENDERER...</span
            >
        {/if}
    </div>
</div>
