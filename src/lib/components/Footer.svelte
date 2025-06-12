<script lang="ts">
    import { getVersion } from '@tauri-apps/api/app';
    import { onMount } from 'svelte';
    import { listen } from '@tauri-apps/api/event';
    import { updater } from '$lib/services/tauri';

    let width = $state(0)
    let collapsed = $derived(width < 800)
    let version: string | null = $state(null)
    let updateInfo: { hasUpdate: boolean; updateVersion: string | null } = $state({
        hasUpdate: false,
        updateVersion: null,
    })
    let progress: { downloaded: number; total: number } = $state({
        downloaded: 0,
        total: 0,
    })
    let downloading = $state(false)

    const updateApp = async () => {
        downloading = true;
        await updater.downloadAndInstallUpdate();
    }

    onMount(async () => {
        version = await getVersion() || "unknown"
        const updateListener = await listen<{ hasUpdate: boolean; updateVersion: string | null }>('update', (event) => {
            updateInfo = event.payload;
        });
        const updateDownloadedListener = await listen<{ downloaded: number; total: number }>('update_downloaded', (event) => {
            progress = event.payload;
        });
        // unlistens
        // updateListener()
        // updateDownloadedListener()
    })

</script>
<div bind:clientWidth={width} class="h-[42px] px-4 bg-dark flex items-center gap-4">
    <!-- Aeroe Version -->
     {#if !collapsed}
     <div class="flex mx-4 gap-2 justify-center items-center">
        <div class="h-[24px] px-1 font-title text-sm text-light border border-light">v{version}</div>
        {#if updateInfo.hasUpdate}
            {#if downloading}
                <div class="text-center px-1 font-title text-xs text-highlight-orange">
                    downloading update: {(progress.downloaded / 1024 / 1024).toFixed(2)} / {(progress.total / 1024 / 1024).toFixed(2)} MB
                </div>
            {:else}
                <div class="text-center px-1 font-title text-xs text-highlight-orange">
                    update available: v{updateInfo.updateVersion}
                    <button class="text-highlight-orange" onclick={updateApp}>[<span class="underline">download and install</span>]
                    </button>
                </div>
            {/if}
        {/if}
     </div>
     {/if}
     <div class="flex-1"></div>
     <div class="flex gap-{collapsed ? "1" : "4"} text-light font-title text-xs {collapsed ? "flex-col" :""}">
        <div><a class="text-highlight-orange" href="https://swps.io/terms-of-use" target="_blank">[terms]</a></div>
        <div><a class="text-highlight-orange" href="https://swps.io/privacy-policies" target="_blank">[privacy]</a></div>

    </div>
</div>