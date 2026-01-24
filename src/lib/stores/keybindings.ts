import { writable } from 'svelte/store';

// Store: Keybindings Map
export const keybindings = writable<Record<string, string>>({
    'Space': 'togglePlayback',
    'Control+z': 'undo',
});
