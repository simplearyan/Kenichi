<script lang="ts">
    import "virtual:uno.css";
    import "@unocss/reset/tailwind.css";

    let { children } = $props();
</script>

{@render children()}
