<script lang="ts">
    import { vault } from "$lib/scripts/commands";
    import Button from "./Button.svelte";

    let vaultCreate: any = $state(undefined);
    let vaultLoad: any = $state(undefined);

    let createVaultPassword = $state("password");
    let loadVaultPassword = $state("password");

    const createVault = async () => {
        vaultCreate = undefined;
        const res = await vault.create(createVaultPassword);
        if (res.success) {
            vaultCreate = "Vault created";
        } else {
            vaultCreate = res.error;
        }
    }

    const loadVault = async () => {
        vaultLoad = undefined;
        const res = await vault.load(loadVaultPassword);
        if (res.success) {
            vaultLoad = "Vault loaded";
        } else {
            vaultLoad = res.error;
        }
    }
</script>
<div class="flex flex-col gap-4 border-2 border-dark p-4">
    <div class="flex gap-4 text-xs font-title items-center">
        <div>Create vault:</div>
        <div class="flex flex-col gap-1">
            <input class="p-1 text-xs" type="text" bind:value={createVaultPassword} />
            <Button onClick={createVault} disabled={false}>Create vault</Button>
        </div>
        <div>{vaultCreate}</div>
    </div>
    <div class="flex gap-4 text-xs font-title items-center">
        <div>Load vault:</div>
        <div class="flex flex-col gap-1">
            <input class="p-1 text-xs" type="text" bind:value={loadVaultPassword} />
            <Button onClick={loadVault} disabled={false}>Load vault</Button>
        </div>
        <div>{vaultLoad}</div>
    </div>
</div>