import { writable, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface Clip {
    id: string;
    path: string;
    start: number;
    duration: number;
    offset: number;
}

function createTimelineStore() {
    const { subscribe, set, update } = writable<Clip[]>([]);

    return {
        subscribe,
        addClip: (path: string, duration: number = 10.0) => {
            update(clips => {
                // For simplicity, append to end
                const lastClip = clips[clips.length - 1];
                const startTime = lastClip ? (lastClip.start + lastClip.duration) : 0.0;

                const newClip: Clip = {
                    id: Math.random().toString(36).substr(2, 9),
                    path,
                    start: startTime,
                    duration: duration,
                    offset: 0.0
                };

                const updated = [...clips, newClip];

                // Sync with Backend
                console.log("Syncing Composition to Rust:", updated);
                invoke('update_composition', { newClips: updated });

                return updated;
            });
        },
        clear: () => {
            set([]);
            invoke('update_composition', { newClips: [] });
        }
    };
}

export const clips = createTimelineStore();
export const tracks = writable([]); // Future use
