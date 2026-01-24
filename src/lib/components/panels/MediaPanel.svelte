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
    </div>

    <div class="text-ui-muted text-center mt-10 opacity-30">
        <span class="i-lucide-upload-cloud text-3xl block mx-auto mb-2"></span>
        Drag & Drop Assets
    </div>
</div>
