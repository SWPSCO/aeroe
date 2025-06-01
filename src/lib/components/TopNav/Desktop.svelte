<script>
    import DesktopItem from '$lib/components/TopNav/DesktopItem.svelte';
    import { page } from '$app/stores';
    import Logo from './Logo.svelte';
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
</nav>