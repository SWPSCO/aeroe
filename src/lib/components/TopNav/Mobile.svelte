<script>
    import Logo from './Logo.svelte';
    import MobileMenu from './MobileMenu.svelte';
</script>
<nav class="flex justify-end bg-gray-1 border-b-4 border-light px-4">
    <div class="flex-1 m-auto">
        <Logo />
    </div>
    <MobileMenu />
</nav>

<style>
    nav {
        height: 66px;
    }
</style>