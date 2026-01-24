import { writable } from 'svelte/store';

// Store: HUD Notifications
export const hudMessage = writable<string | null>(null);
