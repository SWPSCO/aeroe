<script>
    // import ContactIconButton from './ContactIconButton.svelte'
    import BigButton from './BigButton.svelte'
    import { walletBalance } from '$lib/scripts/stores'
</script>
<!-- <div class="p-8 bg-light mb-[32px] border-2 border-dark"> -->
<div class="p-8 bg-gray-800 mb-[32px] border-2 border-dark opacity-30 cursor-not-allowed">
    <div class="text-xs font-title uppercase opacity-50 mb-4">send to</div>
    <div class="flex items-center gap-4 mb-8">
        <input 
            type="text"
            placeholder="enter address"
            class="flex-1 uppercase font-title text-xs cursor-not-allowed"
            disabled
        />
        <!-- DISABLE FOR MVP
        <input 
            type="text"
            placeholder="select contact or enter address"
            class="flex-1 uppercase font-title text-xs"
        />
        -->
        <!-- DISABLE FOR MVP
        <ContactIconButton />
        -->
    </div>
    <div class="text-xs font-title uppercase opacity-50 mb-4">amount</div>
    <div class="flex items-center gap-4 mb-2">
        <input 
            type="number"
            placeholder="0.00"
            class="flex-1 uppercase font-title text-xs cursor-not-allowed"
            disabled
        />
        <button class="max-button uppercase font-title size-[36px] text-light text-xs cursor-not-allowed" disabled>max</button>
    </div>
    <div class="text-xs opacity-50 font-title uppercase mb-8">Balance: {$walletBalance}</div>
    <BigButton disabled>send</BigButton>
</div>

<style>
    input {
        border-radius: 4px;
        /* background: #E2E2E2; */
        background: #090909;
        box-shadow: 0.5px 0.5px 1px 0px #090909 inset;
    }
    .max-button {
        border-radius: 4px;
        background: #090909;
        box-shadow: 1px 1px 1px 0px rgba(0, 0, 0, 0.40) inset, -1px -1px 1px 0px rgba(9, 9, 9, 0.40) inset;
    }
</style>