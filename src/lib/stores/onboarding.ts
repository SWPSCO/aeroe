import { writable, get } from 'svelte/store';
import { terms as termsService } from '$lib/services/tauri';
import { mainStore } from './main';

interface OnboardingState {
    termsAccepted: boolean;
    privacyAccepted: boolean;
    error: string | null;
}

function createOnboardingStore() {
    const store = writable<OnboardingState>({
        termsAccepted: false,
        privacyAccepted: false,
        error: null,
    });
    const { subscribe, update, set } = store;

    async function checkStatus() {
        const termsRes = await termsService.isTermsAccepted();
        const privacyRes = await termsService.isPrivacyAccepted();

        if (termsRes.success && privacyRes.success) {
            update(s => ({
                ...s,
                termsAccepted: termsRes.data ?? false,
                privacyAccepted: privacyRes.data ?? false,
            }));
        } else {
            update(s => ({ ...s, error: 'Failed to check terms status.' }));
        }
    }

    async function acceptTerms() {
        const res = await termsService.setTermsAccepted();
        if (res.success) {
            await checkStatus();
        } else {
            update(s => ({...s, error: 'Failed to accept terms.'}));
        }
    }

    async function acceptPrivacy() {
        const res = await termsService.setPrivacyAccepted();
        if (res.success) {
            // Privacy is the last step. After accepting,
            // we transition to the welcome flow to create a wallet.
            mainStore.completeOnboarding();
        } else {
            update(s => ({...s, error: 'Failed to accept privacy policy.'}));
        }
    }

    return {
        subscribe,
        checkStatus,
        acceptTerms,
        acceptPrivacy,
    }
}

export const onboardingStore = createOnboardingStore(); 