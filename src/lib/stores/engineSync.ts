import { writable } from 'svelte/store';

// Store: Engine Synchronization
// Throttles IPC updates from Rust to Svelte
export const engineState = writable({
    ready: false,
    fps: 0,
});
