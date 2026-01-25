<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    let pathInput = $state("");
    let loading = $state(false);

    async function handleImport() {
        if (!pathInput) return;
        loading = true;
        try {
            console.log("Importing:", pathInput);
            const proxyPath = await invoke("load_file", { path: pathInput });
            console.log("Loaded/Proxy Path:", proxyPath);
            pathInput = "";
        } catch (e) {
            console.error("Import failed:", e);
        } finally {
            loading = false;
        }
    }

    async function togglePlay(play: boolean) {
        try {
            if (play) await invoke("play");
            else await invoke("pause");
        } catch (e) {
            console.error("Playback control failed:", e);
        }
    }

    async function seekDebug() {
        try {
            await invoke("seek", { time: 5.0 });
        } catch (e) {
            console.error("Seek failed:", e);
        }
    }

    function onKeydown(e: KeyboardEvent) {
        if (e.key === "Enter") handleImport();
    }
</script>

<div class="flex-1 overflow-y-auto p-2">
    <div
        class="p-3 mb-4 rounded-lg bg-ui-surface border border-ui-border flex flex-col gap-2"
    >
        <div
            class="text-[10px] font-medium text-ui-foreground uppercase tracking-wider opacity-70"
        >
            Testing Import
        </div>
        <div class="flex gap-2">
            <input
                type="text"
                bind:value={pathInput}
                onkeydown={onKeydown}
                class="flex-1 bg-black/20 border border-ui-border rounded px-2 py-1 text-xs focus:border-brand-accent outline-none text-ui-foreground placeholder:text-ui-muted/50"
                placeholder="Resulting Proxy Path..."
            />
            <button
                onclick={handleImport}
                disabled={loading}
                class="px-3 py-1 bg-brand-accent text-brand-dark font-medium rounded text-xs hover:bg-brand-accent/90 disabled:opacity-50 transition-colors"
            >
                {loading ? "..." : "Load"}
            </button>
        </div>

        <div class="flex gap-2 mt-2 border-t border-ui-border pt-2">
            <button
                onclick={() => togglePlay(true)}
                class="px-2 py-1 bg-green-500/20 text-green-500 rounded text-xs hover:bg-green-500/30"
                >Play</button
            >
            <button
                onclick={() => togglePlay(false)}
                class="px-2 py-1 bg-red-500/20 text-red-500 rounded text-xs hover:bg-red-500/30"
                >Pause</button
            >
            <button
                onclick={seekDebug}
                class="px-2 py-1 bg-blue-500/20 text-blue-500 rounded text-xs hover:bg-blue-500/30"
                >Seek 5s</button
            >
        </div>
    </div>

    <div class="text-ui-muted text-center mt-10 opacity-30">
        <span class="i-lucide-upload-cloud text-3xl block mx-auto mb-2"></span>
        Drag & Drop Assets
    </div>
</div>
