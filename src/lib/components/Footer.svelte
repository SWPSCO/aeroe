<script>
    import { getVersion } from '@tauri-apps/api/app';
    import { onMount } from 'svelte';

    let width = $state(0)
    let collapsed = $derived(width < 800)
    let version = $state(null)

    onMount(async () => {
        version = await getVersion() || "unknown"
    })

</script>
<div bind:clientWidth={width} class="h-[42px] px-4 bg-dark flex items-center gap-4">
    <!-- Aeroe Version -->
     {#if !collapsed}
     <div class="h-[24px] px-1 mx-4 font-title text-sm text-light border border-light">v{version}</div>
     {/if}
     <div class="flex-1"></div>
     <div class="flex gap-{collapsed ? "1" : "4"} text-light font-title text-xs {collapsed ? "flex-col" :""}">
        <div><a class="text-highlight-orange" href="https://aeroe.io/terms-of-use" target="_blank">[terms]</a></div>
        <div><a class="text-highlight-orange" href="https://aeroe.io/privacy-policy" target="_blank">[privacy]</a></div>
    </div>
</div>