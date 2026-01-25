import { writable, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface Clip {
    id: string;
    path: string;
    name: string; // [NEW] Added for display
    start: number;
    duration: number;
    offset: number;
    trackId: number; // [NEW] Added for multi-track support
}

export const magneticMode = writable(true);

function createTimelineStore() {
    const { subscribe, set, update } = writable<Clip[]>([]);

    async function syncToBackend(currentClips: Clip[]) {
        try {
            console.log("Syncing Composition to Rust:", currentClips);
            // Transform for backend consistency if needed, currently direct mapping
            await invoke('update_composition', { newClips: currentClips });
        } catch (e) {
            console.error("Failed to sync composition:", e);
        }
    }

    return {
        subscribe,

        /**
         * Adds a new clip to the target track, appended to the end.
         */
        addClip: (path: string, duration: number = 10.0, trackId: number = 1) => {
            update(clips => {
                // Find end time of the last clip on this track
                const trackClips = clips.filter(c => c.trackId === trackId);
                const lastClip = trackClips.sort((a, b) => (a.start + a.duration) - (b.start + b.duration)).pop();

                const startTime = lastClip ? (lastClip.start + lastClip.duration) : 0.0;

                const newClip: Clip = {
                    id: Math.random().toString(36).substr(2, 9),
                    path,
                    name: path.split(/[\\/]/).pop() || 'Untitled',
                    start: startTime,
                    duration: duration,
                    offset: 0.0,
                    trackId
                };

                const updated = [...clips, newClip];
                syncToBackend(updated);
                return updated;
            });
        },

        /**
         * Removes a clip and optionally ripples following clips (Magnetic).
         */
        removeClip: (id: string) => {
            update(clips => {
                const index = clips.findIndex(c => c.id === id);
                if (index === -1) return clips;

                const deletedClip = clips[index];
                const isMagnetic = get(magneticMode);

                // 1. Filter out the deleted clip
                let nextClips = clips.filter(c => c.id !== id);

                // 2. If Magnetic, shift all following clips on the same track
                if (isMagnetic) {
                    nextClips = nextClips.map(clip => {
                        // Only affect clips on the same track that start AFTER the deleted one
                        if (clip.trackId === deletedClip.trackId && clip.start > deletedClip.start) {
                            return {
                                ...clip,
                                start: clip.start - deletedClip.duration
                            };
                        }
                        return clip;
                    });
                }

                syncToBackend(nextClips);
                return nextClips;
            });
        },

        /**
         * Clears the timeline.
         */
        clear: () => {
            const empty: Clip[] = [];
            set(empty);
            syncToBackend(empty);
        }
    };
}

export const clips = createTimelineStore();
export const tracks = writable([
    { id: 1, name: 'Video 1' },
    { id: 2, name: 'Video 2' }
]);
