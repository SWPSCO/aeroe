import { invoke } from '@tauri-apps/api/core';

/*
    This is the main function we call to invoke commands on the wallet.
    It takes the command name and optional arguments.
    Returns a promise that resolves to an object with a success bool and data/error.
    We do not need to touch this function.
*/
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

async function notImplemented() {
    return { success: false, error: "Not implemented" };
}

/*
    These are the commands for the terms of use and privacy policy.
    Note that the frontend MUST NEVER show any other pages as long as terms and privacy are not accepted.
*/
export const terms = {
    isTermsAccepted: async () => handleInvoke('terms_of_use_is_accepted'),
    isPrivacyAccepted: async () => handleInvoke('privacy_policy_is_accepted'),
    setTermsAccepted: async () => handleInvoke('accept_terms_of_use'),
    setPrivacyAccepted: async () => handleInvoke('accept_privacy_policy'),
}
/*
    Tauri emits an event whenever an update is available, this is then received by the footer
    The only command we ever need to call for the updater is to download and install the update.
*/
export const updater = {
    downloadAndInstallUpdate: async () => handleInvoke('download_and_install_update'),
}

/*
    These are commands related to the app itself. 
*/
export const aeroe = {
    status: async () => handleInvoke('aeroe_status'),
}
/*
    These are commands for creating and managing the keycrypt vault.
    We don't want to have the seedphrases be readable on disk.
    Keycrypt is a module that handles the encryption and decryption of the
    vault using a password.
*/
export const vault = {
    /**
     * @param {string} password
     */
    // used when exists is false
    create: async (password) => handleInvoke('vault_create', { password }),
    /**
     * @param {string} password
     */
    // used when exists is true
    load: async (password) => handleInvoke('vault_load', { password }),
}
/* 
    You guessed it, these are commands for managing wallets.
    They won't work unless the vault is loaded.
*/
export const wallet = {
    // this generates a new key phrase and returns it to the frontend (this one works without the vault loaded)
    keygen: async () => handleInvoke('keygen'),
    /**
     * @param {string} walletName
     * @param {string[]} seedphrase
     */
    // creates a new wallet in the vault, does not load it!
    create: async (walletName, seedphrase) => handleInvoke('wallet_create', { walletName, seedphrase }),
    /**
     * @param {string} walletName
     */
    // finds the wallet in the vault and loads the seedphrase into the nockapp
    load: async (walletName) => handleInvoke('wallet_load', { walletName }),
    /**
     * @param {string} walletName
     */
    masterPubkey: async (walletName) => handleInvoke('master_pubkey', { walletName }),
    /**
     * @param {string} walletName
     */
    balance: async (walletName) => handleInvoke('balance', { walletName }),
    // TODO:
    /**
     * @param {string} walletName
     */
    getHistory: async (walletName) => notImplemented(),
    /**
     * @param {string} walletName
     */
    listDrafts: async (walletName) => notImplemented(),
    /**
     * @param {string} walletName
     */
    createDraft: async (walletName) => notImplemented(),
    /**
     * @param {string} walletName
     * @param {string} draftId
     */
    sendTransaction: async (walletName, draftId) => notImplemented(),
}

/*
    These are commands for managing the nockchain node. Still immature, do not use.
*/
export const node = {
    startMaster: async () => handleInvoke('node_start_master'),
    stopMaster: async () => handleInvoke('node_stop_master'),
    /**
     * @param {string} miningProfile
     */
    startMining: async (miningProfile) => handleInvoke('node_start_mining', { miningProfile }),
    stopMining: async () => handleInvoke('node_stop_mining'),
}