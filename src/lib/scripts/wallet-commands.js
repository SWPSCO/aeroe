import { invoke } from '@tauri-apps/api/core';

/**
 * @param {string} command
 * @param {import('@tauri-apps/api/core').InvokeArgs} [args]
 */
async function handleInvoke(command, args) {
    try {
        const data = await (args ? invoke(command, args) : invoke(command));
        return { success: true, data: data };
    } catch (error) {
        return { success: false, error: error };
    }
}

export const terms = {
    isTermsAccepted: async () => handleInvoke('terms_of_use_is_accepted'),
    setTermsAccepted: async () => handleInvoke('accept_terms_of_use'),
    isPrivacyAccepted: async () => handleInvoke('privacy_policy_is_accepted'),
    setPrivacyAccepted: async () => handleInvoke('accept_privacy_policy'),
}

// sets memory
export const set = {
    initialize: async () => handleInvoke('initialize'),
};

// fetches from memory
export const get = {
    masterPubkey: async () => handleInvoke('get_master_pubkey'),
    balance: async () => handleInvoke('get_balance'),
};

// fetches from nockapp state
export const peek = {
    state: async () => handleInvoke('peek_state'),
    seedphrase: async () => handleInvoke('peek_seedphrase'),
    balance: async () => handleInvoke('peek_balance'),
    receiveAddress: async () => handleInvoke('peek_receive_address'),
    masterPubkey: async () => handleInvoke('peek_master_pubkey'),
    pubkeys: async () => handleInvoke('peek_pubkeys'),
};

// sends actions to nockapp state
export const poke = {
    keygen: async () => handleInvoke('keygen'),
    /**
     * @param {string[]} seedphrase - An array of 24 strings representing the seed phrase.
     */
    genMasterPrivkey: async (seedphrase) => handleInvoke('gen_master_privkey', { seedphrase }),
    updateBalance: async () => handleInvoke('update_balance'),
    /**
     * @param {string} keyType
     * @param {number} index
     * @param {string} label
     */
    deriveChild: async (keyType, index, label) => handleInvoke('derive_child', { keyType, index, label }),
    listNotes: async () => handleInvoke('list_notes'),
    /**
     * @param {string} pubkey
     */
    listNotesByPubkey: async (pubkey) => handleInvoke('list_notes_by_pubkey', { pubkey }),
};

export const status = {
    isReady: async () => handleInvoke('is_ready'),
    isSetupComplete: async () => handleInvoke('is_setup_complete'),
};