<script>
    import { walletMasterPubkey } from '$lib/scripts/stores'
    
    let copyText = $state("copy")

    const copyToClipboard = () => {
        navigator.clipboard.writeText($walletMasterPubkey)
        copyText = "copied"
        setTimeout(() => {
            copyText = "copy"
        }, 2000)
    }
</script>
<div class="border-2 border-dark p-4 mb-8 flex flex-col gap-4 items-center">
    <div class="font-title text-xs text-center p-8">{$walletMasterPubkey}</div>
    <button class="border-2 border-dark p-2 w-[120px]" onclick={copyToClipboard}>{copyText}</button>
</div>
