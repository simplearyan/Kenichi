import { writable } from 'svelte/store';

// Store: Performance Metrics (e.g., GT 740 check)
export const performanceMode = writable<'high' | 'low'>('low'); // Default to safe low
