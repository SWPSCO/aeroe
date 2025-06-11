<script lang="ts">
    import { aeroe } from "$lib/scripts/commands";
    import Button from "./Button.svelte";

    let aeroeStatus: any = $state(undefined);

    const getAeroeStatus = async () => {
        const res = await aeroe.status();
        if (res.success) {
            aeroeStatus = res.data;
        } else {
            aeroeStatus = res.error;
        }
    }

</script>
<div class="flex flex-col gap-4 border-2 border-dark p-4">
    <div class="flex gap-4 text-xs font-title items-center">
        <div>Aeroe status</div>
        <Button onClick={getAeroeStatus}>Get Aeroe status</Button>
        <div>{JSON.stringify(aeroeStatus)}</div>
    </div>
</div>