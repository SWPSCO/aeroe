<script>
    export let collapsed = false;
</script>
{#if !collapsed}
<div class="border-2 border-dark mb-4 items-center grid grid-cols-9 gap-4 p-4 text-xs font-title uppercase text-center font-bold bg-light py-8 px-4">
    <div class="col-span-2 flex flex-col gap-2">
        <div class="text-lg font-light">1.000 nocks</div>
        <div class="text-xs font-light">= $1.33 USD</div>
    </div>
    <div class="text-md font-semibold text-highlight-orange">Send</div>
    <div class="col-span-2">0x123123123123123...</div>
    <div class="col-span-2">0x123123123123123...</div>
    <div class="col-span-1 flex flex-col gap-2">
        <div class="text-md font-light">08:00 am</div>
        <div class="text-xs font-light opacity-50">Nov 25</div>
    </div>
    <div>
        <button class="text-xs font-light bg-dark p-2 hover:cursor-pointer active:bg-highlight-orange" aria-label="External Arrow">
            <svg width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M8.00001 6L1 13M10.541 1.08278L9.29921 1.19567C7.28731 1.37857 6.28141 1.47002 6.04471 2.12388C5.80791 2.77774 6.52221 3.49198 7.95071 4.92046L9.07951 6.0493C10.508 7.4778 11.2223 8.1921 11.8761 7.9553C12.53 7.7186 12.6214 6.7127 12.8043 4.70078L12.9172 3.45902C13.0273 2.24766 13.0824 1.64198 12.7202 1.27979C12.358 0.9176 11.7523 0.97266 10.541 1.08278Z" stroke="#EDEDED" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        </button>
    </div>
</div>
{:else}
<div class="border-2 border-dark p-4 font-title uppercase mb-4 flex flex-col gap-2">
    <div class="flex">
        <div class="text-xs font-bold flex-1 text-highlight-orange">Send</div>
        <div class="text-xs font-light opacity-50">08:00 am, 25 Nov 2024</div>
    </div>
    <div class="text-3xl">1.000 <span class="text-xl font-light opacity-50">nocks</span></div>
    <div class="flex items-center text-xs">
        <div class="w-24 opacity-50">Sender:</div>
        <div>0x123123123123123...</div>
    </div>
    <div class="flex items-center text-xs">
        <div class="text-xs w-24 opacity-50">Recipient:</div>
        <div>0x123123123123123...</div>
    </div>
</div>
{/if}