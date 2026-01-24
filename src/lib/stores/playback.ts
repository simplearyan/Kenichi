import { writable } from 'svelte/store';

export const isPlaying = writable(false);
export const currentTime = writable(0.0); // in seconds
export const timecode = writable("00:00:00:00");

// Simple helper to format time
export function formatTimecode(seconds: number): string {
    // Placeholder logic for now, simple HH:MM:SS:FF
    // Assuming 30fps for now
    const frame = Math.floor((seconds % 1) * 30);
    const wholeSeconds = Math.floor(seconds);
    const s = wholeSeconds % 60;
    const m = Math.floor(wholeSeconds / 60) % 60;
    const h = Math.floor(wholeSeconds / 3600);

    const pad = (n: number) => n.toString().padStart(2, '0');
    return `${pad(h)}:${pad(m)}:${pad(s)}:${pad(frame)}`;
}
