// This file acts as a switchboard for the API services.
// It exports the mock services if the VITE_MOCK_API environment variable is set to 'true'.
// Otherwise, it exports the real Tauri services.

import * as real from './tauri';
import * as mock from './tauri.mock';

const useMock = import.meta.env.VITE_MOCK_API === 'true';

console.log(useMock ? 'Using Mock API' : 'Using Real Tauri API');

export const terms = useMock ? mock.terms : real.terms;
export const updater = useMock ? mock.updater : real.updater;
export const aeroe = useMock ? mock.aeroe : real.aeroe;
export const vault = useMock ? mock.vault : real.vault;
export const wallet = useMock ? mock.wallet : real.wallet;
export const node = useMock ? mock.node : real.node; 