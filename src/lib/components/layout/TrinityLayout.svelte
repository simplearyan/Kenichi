<script lang="ts">
    import MediaPanel from "../panels/MediaPanel.svelte";
    import VideoViewport from "../preview/VideoViewport.svelte";
    import Inspector from "../panels/Inspector.svelte";
    import Timeline from "../panels/Timeline.svelte";
    import { timecode } from "$lib/stores/playback";
    import { getCurrentWindow } from "@tauri-apps/api/window";

    const appWindow = getCurrentWindow();

    function minimize() {
        appWindow.minimize();
    }

    function maximize() {
        appWindow.toggleMaximize();
    }

    function close() {
        appWindow.close();
    }
</script>

<div
    class="h-screen w-screen flex flex-col bg-transparent overflow-hidden text-ui-text"
>
    <header
        class="h-40px nle-panel border-b flex items-center px-4 justify-between bg-kenichi-header select-none"
        data-tauri-drag-region
    >
        <div class="flex items-center gap-3 pointer-events-none">
            <div
                class="w-20px h-20px bg-brand-accent rounded-sm flex items-center justify-center"
            >
                <span class="i-lucide-play text-black text-[12px]"></span>
            </div>
            <span class="text-ui-bold tracking-tighter uppercase font-900"
                >Kenichi</span
            >
        </div>

        <div class="flex items-center gap-1">
            <button
                class="nle-icon-btn"
                aria-label="Minimize"
                onclick={minimize}
                ><span class="i-lucide-minus w-14px h-14px"></span></button
            >
            <button
                class="nle-icon-btn"
                aria-label="Maximize"
                onclick={maximize}
                ><span class="i-lucide-square w-12px h-12px"></span></button
            >
            <button
                class="nle-icon-btn hover:bg-red-600! hover:text-white"
                aria-label="Close"
                onclick={close}
                ><span class="i-lucide-x w-16px h-16px"></span></button
            >
        </div>
    </header>

    <main class="flex-1 flex min-h-0">
        <aside class="w-300px nle-panel border-r flex flex-col">
            <div class="panel-header">
                <span class="i-lucide-folder-open mr-2"></span> Media Assets
            </div>
            <MediaPanel />
        </aside>

        <section
            class="flex-1 relative bg-transparent flex flex-col overflow-hidden"
        >
            <div
                class="panel-header bg-black/50 backdrop-blur-sm border-b-transparent absolute top-0 w-full z-10"
            >
                <span class="i-lucide-monitor-play mr-2"></span> Program Preview
            </div>

            <VideoViewport />

            <div
                class="absolute bottom-4 left-1/2 -translate-x-1/2 flex items-center gap-4 bg-kenichi-panel/80 backdrop-blur px-4 py-2 rounded-full border border-white/10"
            >
                <button class="nle-icon-btn" aria-label="Previous Frame"
                    ><span class="i-lucide-skip-back"></span></button
                >
                <button
                    class="w-32px h-32px rounded-full bg-white text-black flex items-center justify-center hover:scale-105 transition-transform"
                    aria-label="Play"
                >
                    <span class="i-lucide-play fill-current"></span>
                </button>
                <button class="nle-icon-btn" aria-label="Next Frame"
                    ><span class="i-lucide-skip-forward"></span></button
                >
            </div>
        </section>

        <aside class="w-320px nle-panel border-l flex flex-col">
            <div class="panel-header">
                <span class="i-lucide-sliders-horizontal mr-2"></span> Inspector
            </div>
            <Inspector />
        </aside>
    </main>

    <footer class="h-300px nle-panel border-t flex flex-col shadow-2xl">
        <div class="panel-header justify-between">
            <div class="flex items-center">
                <span class="i-lucide-layers-2 mr-2"></span> Timeline
            </div>
            <div class="flex items-center gap-4">
                <div class="flex items-center gap-1 text-brand-accent">
                    <span class="i-lucide-clock text-[12px]"></span>
                    <span class="font-mono">{$timecode}</span>
                </div>
            </div>
        </div>
        <Timeline />
    </footer>
</div>

<style>
    /* Ensure smooth transitions for layout shifts on low-end hardware */
    main,
    aside,
    section,
    footer {
        transition:
            width 0.1s ease-out,
            height 0.1s ease-out;
    }
</style>
