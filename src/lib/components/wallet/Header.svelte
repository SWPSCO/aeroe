<script lang="ts">
    import type { WalletBalance } from '$lib/services/tauri';

    interface Props {
        balance: WalletBalance | null;
        masterPubkey: string | null;
    }
    let { balance, masterPubkey }: Props = $props();

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
    {#if balance && masterPubkey}
    <div class="flex gap-4 mt-4 items-{collapsed ? 'start' : 'center'} {collapsed ? 'flex-col' : ''}">
        <div class="flex-1 flex font-title items-{collapsed ? 'start' : 'end'} gap-2 {collapsed ? 'flex-col' : ''}">
            <div class="flex gap-2 items-end">
                <div class="text-{collapsed ? '4xl' : '3xl'} text-highlight-orange">{balance.amount}</div>
                <div class="uppercase text-2xl text-dark">{balance.coin}</div>
            </div>
            {#if !collapsed}
                <svg width="1" height="32" class="text-light mx-2">
                    <rect width="1" height="32" fill="currentColor" />
                </svg>
            {/if}
            <div class="text-xs text-gray-500 font-mono self-end">
                {masterPubkey}
            </div>
        </div>
        <div class="flex gap-2">
            <!-- Buttons can be re-enabled later -->
        </div>
    </div>
    {/if}
</div>