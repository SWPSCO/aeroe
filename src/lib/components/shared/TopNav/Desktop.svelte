<script>
    import DesktopItem from './DesktopItem.svelte';
    import { page } from '$app/stores';
    import Logo from './Logo.svelte';
    import { sessionStore } from '$lib/stores/session';
    import { walletStore } from '$lib/stores/wallet';
</script>

<nav class="flex justify-end gap-16 pr-16 bg-light h-[66px]">
    <div class="pt-1 border-b-4 border-light">
        <Logo />
    </div>
    <div class="flex-1 border-b-4 border-light"></div>
    <DesktopItem route="/wallet" active={$page.route.id?.startsWith('/wallet')}>Wallet</DesktopItem>
    <DesktopItem route="/mining" active={$page.route.id?.startsWith('/mining')}>Mining</DesktopItem>
    <!-- <DesktopItem route="/miners" active={$page.route.id?.startsWith('/miners')}>Miners</DesktopItem>
    <DesktopItem route="/nockpool" active={$page.route.id?.startsWith('/nockpool')}>Nockpool</DesktopItem> -->
    <DesktopItem route="/explorer" active={$page.route.id?.startsWith('/explorer')}>Explorer</DesktopItem>
    
    <div class="flex-1 border-b-4 border-light"></div>

    <div class="flex items-center gap-4 text-sm font-title pr-4 border-b-4 border-light">
        {#if $walletStore.status === 'loading'}
            <div class="text-gray-500 animate-pulse">Syncing...</div>
        {:else if $walletStore.status === 'loaded'}
            <div class="flex items-center gap-2">
                <div class="w-2 h-2 rounded-full bg-green-500"></div>
                <div class="text-dark">{$sessionStore.activeWalletName}</div>
            </div>
        {:else if $walletStore.status === 'error'}
            <div class="flex items-center gap-2">
                <div class="w-2 h-2 rounded-full bg-red-500"></div>
                <div class="text-red-500">Error</div>
            </div>
        {/if}
    </div>
</nav>