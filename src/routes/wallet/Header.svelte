<script>
    import Button from './Button.svelte'
    import { walletBalance } from '$lib/scripts/stores'

    let width = $state(0)
    let collapsed = $derived(width < 900)

    $effect(() => {
        const updateWidth = () => {
            width = window.innerWidth
        }

        window.addEventListener('resize', updateWidth)
        updateWidth() // Initial call

        return () => {
            window.removeEventListener('resize', updateWidth)
        }
    })
</script>
<div bind:clientWidth={width} class="pt-8 pb-4 px-8 bg-light">
    <div class="uppercase font-title text-dark text-sm">total assets value</div>
    <div class="flex gap-4 mt-4 items-{collapsed ? 'start' : 'center'} {collapsed ? 'flex-col' : ''}">
        <div class="flex-1 flex font-title items-{collapsed ? 'start' : 'end'} gap-2 {collapsed ? 'flex-col' : ''}">
            <div class="flex gap-2 items-end">
                <div class="text-{collapsed ? '4xl' : '3xl'} text-highlight-orange">{$walletBalance == null ? "errored" : $walletBalance}</div>
                <div class="uppercase text-2xl text-dark">nocks</div>
            </div>
            {#if !collapsed}
                <svg width="1" height="32" class="text-light mx-2">
                    <rect width="1" height="32" fill="currentColor" />
                </svg>
            {/if}
            <!-- DISABLE FOR MVP
            <div class="flex gap-2 items-end">
                <div class="text-{collapsed ? "2xl" : "3xl"} text-{collapsed ? "dark" : "highlight-orange"}">$1,300.28</div>
                <div class="border-2 border-dark text-dark text-xs p-1 w-12 flex items-center justify-center mb-1">
                    <span class="text-xs flex-1">USD</span>
                    <svg class="w-2 h-2" viewBox="0 0 10 10" fill="currentColor">
                        <polygon points="0,2 12,2 5,8" />
                    </svg>
                </div>
            </div>
            -->
        </div>
        <div class="flex gap-2">
            <!-- DISABLE FOR MVP
            <Button
                color="bg-highlight-orange"
                textColor="text-light">Buy</Button>
            <Button>Send</Button>
            <Button>Receive</Button>
            -->
        </div>
    </div>
</div>