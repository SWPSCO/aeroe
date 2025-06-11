<script lang="ts">
    import { terms } from "$lib/scripts/commands";
    import { onMount } from "svelte";
    import Button from "./Button.svelte";

    let termsAccepted = $state(undefined);
    let termsLoading = $state(false);
    let privacyAccepted = $state(undefined);
    let privacyLoading = $state(false);

    const getTermsAndPrivacy = async () => {
        const termsResponse = await terms.isTermsAccepted();
        const privacyResponse = await terms.isPrivacyAccepted();
        termsAccepted = termsResponse.success ? termsResponse.data : termsResponse.error;
        privacyAccepted = privacyResponse.success ? privacyResponse.data : privacyResponse.error;
    }

    const setTermsAccepted = async (doc: "terms" | "privacy") => {
        if (doc == "terms") {
            termsLoading = true;
            const termsResponse = await terms.setTermsAccepted();
            if (termsResponse.success) {
                const termsResponse = await terms.isTermsAccepted();
                termsAccepted = termsResponse.success ? termsResponse.data : termsResponse.error;
            }
            termsLoading = false;
        } else if (doc == "privacy") {
            privacyLoading = true;
            const privacyResponse = await terms.setPrivacyAccepted();
            if (privacyResponse.success) {
                const privacyResponse = await terms.isPrivacyAccepted();
                privacyAccepted = privacyResponse.success ? privacyResponse.data : privacyResponse.error;
            }
            privacyLoading = false;
        }
    }

    onMount(async () => {
        await getTermsAndPrivacy();
    });
</script>
<div class="flex flex-col gap-4 border-2 border-dark p-4">
    <div class="flex gap-4 text-xs font-title items-center">
        <div>Terms of use accepted: {termsLoading ? "loading..." : termsAccepted}</div>
        <Button onClick={() => setTermsAccepted("terms")}>Accept terms</Button>
    </div>
    <div class="flex gap-4 text-xs font-title items-center">
        <div>Privacy policy accepted: {privacyLoading ? "loading..." : privacyAccepted}</div>
        <Button onClick={() => setTermsAccepted("privacy")}>Accept privacy</Button>
    </div>
</div>