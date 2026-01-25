<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount, onDestroy } from "svelte";

    let currentTime = $state(0.0);
    // let duration = $state(0.0);
    let isPlaying = $state(false);
    let polling = false;

    // config
    const PIXELS_PER_SECOND = 100;

    async function pollState() {
        if (!polling) return;
        try {
            const state: any = await invoke("get_playback_state");
            currentTime = state.current_time;
            isPlaying = state.is_playing;
            // duration = state.duration;

            if (polling) requestAnimationFrame(pollState);
        } catch (e) {
            console.error("Polling error:", e);
            // Retry slower if error
            setTimeout(() => {
                if (polling) pollState();
            }, 1000);
        }
    }

    onMount(() => {
        polling = true;
        pollState();
    });

    onDestroy(() => {
        polling = false;
    });

    async function handleSeek(e: MouseEvent) {
        // Calculate time from click position
        const rect = (e.currentTarget as HTMLElement).getBoundingClientRect();
        const outputX = e.clientX - rect.left;
        const time = outputX / PIXELS_PER_SECOND;

        // Optimistic update
        currentTime = time;

        invoke("seek", { time });
    }
</script>

<div class="flex-1 overflow-hidden relative" onclick={handleSeek}>
    <!-- Grid -->
    <div
        class="absolute inset-0 bg-white/2 opacity-5 pointer-events-none"
        style="background-image: linear-gradient(90deg, #fff 1px, transparent 1px); background-size: {PIXELS_PER_SECOND}px 100%;"
    ></div>

    <!-- Playhead -->
    <div
        class="absolute top-0 bottom-0 w-[2px] bg-brand-accent shadow-[0_0_10px_rgba(34,211,238,0.5)] z-10 pointer-events-none"
        style="left: {currentTime * PIXELS_PER_SECOND}px;"
    >
        <!-- Triangle Cap -->
        <div
            class="absolute -top-[5px] -left-[4px] border-l-[5px] border-r-[5px] border-t-[8px] border-t-brand-accent border-l-transparent border-r-transparent"
        ></div>
    </div>
</div>
