<script lang="ts">
    import { aeroe, wallet } from "$lib/services/tauri";
    import Button from "./Button.svelte";

    let txCreateResult: any = $state(undefined);
    let txSignResult: any = $state(undefined);
    let txSendResult: any = $state(undefined);
    let txListResult: any = $state(undefined);

    let draftId = $state("");

    const walletName = async () => {
        const res = await aeroe.status();
        return res.data?.activeWallet;
    }

    const createTx = async () => {
        txCreateResult = undefined;
        const name = await walletName();
        if (!name) {
            txCreateResult = "No wallet selected";
            return;
        }
        const res = await wallet.createTx(name, [
            { recipient: "2mJCJ8HWBLdqM6yT2fB1HoVndTHK11MTzuj57pmi6srjkwvjwkmsEyZES6STfNL8vczjNucvVZjEXJpDzBaP9ft4c17JZfuZkqADWtKSzWFySCuQPeY6n2etEtzC2bHPSH2Y", amount: 100 }
        ], 1);
        txCreateResult = res;
    }

    const signTx = async () => {
        txSignResult = undefined;
        const name = await walletName();
        if (!name) {
            txSignResult = "No wallet selected";
            return;
        }
        const res = await wallet.signTx(name, draftId);
        txSignResult = res;
    }

    const sendTx = async () => {
        txSendResult = undefined;
        const name = await walletName();
        if (!name) {
            txSendResult = "No wallet selected";
            return;
        }
        const res = await wallet.sendTx(name, draftId);
        txSendResult = res;
    }

    const listUnsentTxs = async () => {
        txListResult = undefined;
        const name = await walletName();
        if (!name) {
            txListResult = "No wallet selected";
            return;
        }
        const res = await wallet.listUnsentTxs(name);
        txListResult = res;
    }
</script>
<div class="flex flex-col gap-4 border-2 border-dark p-4">
    <div class="flex gap-4 text-xs font-title items-center">
        <div>Wallet create tx:</div>
        <Button onClick={createTx} disabled={false}>Create</Button>
        <div>{JSON.stringify(txCreateResult)}</div>
    </div>
    <div class="flex gap-4 text-xs font-title items-center">
        <div>Wallet sign tx:</div>
        <input type="text" bind:value={draftId} />
        <Button onClick={signTx} disabled={false}>Sign</Button>
        <div>{JSON.stringify(txSignResult)}</div>
    </div>
    <div class="flex gap-4 text-xs font-title items-center">
        <div>Wallet send tx:</div>
        <input type="text" bind:value={draftId} />
        <Button onClick={sendTx} disabled={false}>Send</Button>
        <div>{JSON.stringify(txSendResult)}</div>
    </div>
    <div class="flex gap-4 text-xs font-title items-center">
        <div>Wallet list txs:</div>
        <Button onClick={listUnsentTxs} disabled={false}>List</Button>
        <div>{JSON.stringify(txListResult)}</div>
    </div>
</div>