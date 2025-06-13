<script lang="ts">
    import { wallet } from "$lib/services/tauri";
    import Button from "./Button.svelte";

    let walletKeygen: any = $state(undefined);
    let walletCreate: any = $state(undefined);
    let walletLoad: any = $state(undefined);
    let walletMasterPubkey: any = $state(undefined);
    let walletBalance: any = $state(undefined);
    let walletHistory: any = $state(undefined);
    let drafts: any = $state(undefined);
    let draft: any = $state(undefined);
    let txResult: any = $state(undefined);

    let keygenLoading: boolean = $state(false);
    let loadLoading: boolean = $state(false);

    let phrase: string = $state("");
    let newWalletName: string = $state("some wallet name");
    let loadWalletName: string = $state("");

    const keygen = async () => {
        keygenLoading = true;
        walletKeygen = undefined;
        const res = await wallet.keygen();
        if (res.success && res.data) {
            walletKeygen = res.data.join(" ");
        } else {
            walletKeygen = res.error;
        }
        keygenLoading = false;
    }

    const createWallet = async () => {
        // note that we're splitting the phrase into an array of strings here but use an array in the real thing
        const res = await wallet.create(newWalletName, phrase.split(" ")); 
        walletCreate = res;
    }

    const loadWallet = async () => {
        loadLoading = true;
        walletLoad = undefined;
        const res = await wallet.load(loadWalletName);
        walletLoad = res;
        loadLoading = false;
    }

    const getMasterPubkey = async () => {
        walletMasterPubkey = undefined;
        const res = await wallet.masterPubkey(loadWalletName);
        walletMasterPubkey = res;
    }

    const getBalance = async () => {
        walletBalance = undefined;
        const res = await wallet.balance(loadWalletName);   
        walletBalance = res;
    }

    const getHistory = async () => {
        walletHistory = undefined;
        const res = await wallet.getHistory(loadWalletName);
        walletHistory = res;
    }

    const listDrafts = async () => {
        drafts = undefined;
        const res = await wallet.listDrafts(loadWalletName);
        drafts = res;
    }

    const createDraft = async () => {
        draft = undefined;
        const res = await wallet.createDraft(loadWalletName);
        draft = res;
    }

    const sendTransaction = async () => {
        txResult = undefined;
        const res = await wallet.sendTransaction(loadWalletName, draft.id);
        txResult = res;
    }
</script>

<div class="flex flex-col gap-4 border-2 border-dark p-4">
    <div class="flex gap-4 text-xs font-title items-center">
        <div>Wallet keygen:</div>
        <Button onClick={keygen} disabled={keygenLoading}>
            {keygenLoading ? "Generating..." : "Keygen"}
        </Button>
        <div>{JSON.stringify(walletKeygen)}</div>
    </div>
    <div class="flex gap-4 text-xs font-title items-center">
        <div>Wallet create:</div>
        <input class="p-1 text-xs" type="text" placeholder="wallet name" bind:value={newWalletName} />
        <input class="p-1 text-xs" type="text" placeholder="phrase as string here but in real component it's an array of strings" bind:value={phrase} />
        <Button onClick={createWallet} disabled={false}>Create</Button>
        <div>{JSON.stringify(walletCreate)}</div>
    </div>
    <div class="flex gap-4 text-xs font-title items-center">
        <div>Wallet load:</div>
        <input class="p-1 text-xs" type="text" placeholder="wallet name" bind:value={loadWalletName} />
        <Button onClick={loadWallet} disabled={loadLoading}>{loadLoading ? "Loading..." : "Load"}</Button>
        <div>{JSON.stringify(walletLoad)}</div>
    </div>
    <div class="flex gap-4 text-xs font-title items-center">
        <div>Wallet master pubkey:</div>
        <Button onClick={getMasterPubkey} disabled={false}>Master pubkey</Button>
        <div>{JSON.stringify(walletMasterPubkey)}</div>
    </div>
    <div class="flex gap-4 text-xs font-title items-center">
        <div>Wallet balance:</div>
        <Button onClick={getBalance} disabled={false}>Balance</Button>
        <div>{JSON.stringify(walletBalance)}</div>
    </div>
    <div class="flex gap-4 text-xs font-title items-center">
        <div>Wallet history:</div>
        <Button onClick={getHistory} disabled={false}>History</Button>
        <div>{JSON.stringify(walletHistory)}</div>
    </div>
    <div class="flex gap-4 text-xs font-title items-center">
        <div>List drafts:</div>
        <Button onClick={listDrafts} disabled={false}>List</Button>
        <div>{JSON.stringify(drafts)}</div>
    </div>
    <div class="flex gap-4 text-xs font-title items-center">
        <div>Create draft:</div>
        <Button onClick={createDraft} disabled={false}>Create</Button>
        <div>{JSON.stringify(draft)}</div>
    </div>
    <div class="flex gap-4 text-xs font-title items-center">
        <div>Send transaction:</div>
        <Button onClick={sendTransaction} disabled={false}>Send</Button>
        <div>{JSON.stringify(txResult)}</div>
    </div>
</div>