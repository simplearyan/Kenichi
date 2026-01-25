<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    let container: HTMLDivElement;
    let observer: ResizeObserver;
    let backendReady = $state(false);

    async function syncViewport() {
        if (!container || !backendReady) return;

        const rect = container.getBoundingClientRect();
        const dpr = window.devicePixelRatio || 1;

        // Convert CSS pixels to Physical Pixels for WGPU
        const x = rect.x * dpr;
        const y = rect.y * dpr;
        const width = rect.width * dpr;
        const height = rect.height * dpr;

        console.log(`Syncing Viewport: X=${x} Y=${y} W=${width} H=${height}`);

        await invoke("update_viewport", { x, y, width, height });
    }

    onMount(() => {
        const init = async () => {
            try {
                console.log("Requesting WGPU surface attachment...");
                backendReady = await invoke("attach_wgpu_renderer");
                console.log("WGPU attachment result:", backendReady);

                if (backendReady) {
                    // Initialize ResizeObserver
                    observer = new ResizeObserver(() => {
                        syncViewport();
                    });
                    observer.observe(container);

                    // Also trigger once immediately and on window resize
                    window.addEventListener("resize", syncViewport);
                    syncViewport();
                }
            } catch (error) {
                console.error("Failed to attach WGPU backend:", error);
            }
        };

        if (container) init();

        return () => {
            if (observer) observer.disconnect();
            window.removeEventListener("resize", syncViewport);
        };
    });
</script>

<div class="flex-1 w-full h-full relative overflow-hidden bg-transparent">
    <!-- 
        This div acts as the "Hole" marker. 
        Its position determines where WGPU will draw.
    -->
    <div bind:this={container} class="absolute inset-0 w-full h-full">
        {#if !backendReady}
            <div class="w-full h-full flex items-center justify-center">
                <span class="text-[10px] text-brand-accent/20 font-mono"
                    >INITIALIZING_RENDERER...</span
                >
            </div>
        {/if}
    </div>
</div>
