<script lang="ts">
    import { node } from "$lib/scripts/commands";
    import Button from "./Button.svelte";

    let masterStatus: any = $state(undefined);
    let miningStatus: any = $state(undefined);

    const startMaster = async () => {
        const res = await node.startMaster();
        masterStatus = res
    }

    const stopMaster = async () => {
        const res = await node.stopMaster();
        masterStatus = res;
    }
</script>
<div class="flex flex-col gap-4 border-2 border-dark p-4">
    <div class="flex gap-4 text-xs font-title items-center">
        <div>Master node:</div>
        <div class="flex gap-2">
            <Button onClick={startMaster} disabled={false}>Start</Button>
            <Button onClick={stopMaster} disabled={false}>Stop</Button>
        </div>
        <div>{JSON.stringify(masterStatus)}</div>
    </div>
</div>