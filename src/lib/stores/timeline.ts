import { writable, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface Clip {
    id: string;
    path: string;
    name: string;
    start: number;
    duration: number;
    offset: number;
    trackId: number;
    zIndex: number; // [NEW] Added to match Backend
}

export const magneticMode = writable(true);

function createTimelineStore() {
    const { subscribe, set, update } = writable<Clip[]>([]);

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
                    trackId,
                    zIndex: trackId // Default z-index
                };

                const updated = [...clips, newClip];

                // Opimization: Use granular command
                console.log("Timeline: Adding Clip via Command", newClip);
                invoke('add_clip', { clip: newClip });

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

                // Decision: Since ripple modifies multiple clips, use full sync for safety
                // Ideally, we would have 'shift_clips' command in backend, but for now Full Sync is safer.
                console.log("Timeline: Syncing Ripple Delete via Full Update");
                invoke('update_composition', { newClips: nextClips });

                return nextClips;
            });
        },

        /**
         * Clears the timeline.
         */
        clear: () => {
            set([]);
            invoke('update_composition', { newClips: [] });
        }
    };
}

export const clips = createTimelineStore();
export const tracks = writable([
    { id: 1, name: 'Video 1' },
    { id: 2, name: 'Video 2' }
]);
