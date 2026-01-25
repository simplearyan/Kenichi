use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// --- Data Structures ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clip {
    pub id: String,
    pub path: String,
    pub name: String,
    pub start: f64,    // Global Timeline Time (Seconds)
    pub duration: f64, // Length in seconds
    pub offset: f64,   // Offset into the source file
    pub track_id: i32,
    pub z_index: i32, // Rendering order (higher = on top)
}

impl Clip {
    pub fn is_active(&self, time: f64) -> bool {
        time >= self.start && time < (self.start + self.duration)
    }

    /// Converts global timeline time to local media time
    pub fn get_media_time(&self, global_time: f64) -> f64 {
        (global_time - self.start) + self.offset
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id: i32,
    pub name: String,
    pub is_muted: bool,
    pub is_locked: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Timeline {
    pub tracks: HashMap<i32, Track>,
    pub clips: HashMap<String, Clip>, // Indexed by UUID
    pub duration: f64,
}

impl Timeline {
    pub fn new() -> Self {
        let mut tracks = HashMap::new();
        // Create default tracks
        tracks.insert(
            1,
            Track {
                id: 1,
                name: "Video 1".into(),
                is_muted: false,
                is_locked: false,
            },
        );
        tracks.insert(
            2,
            Track {
                id: 2,
                name: "Video 2".into(),
                is_muted: false,
                is_locked: false,
            },
        );

        Self {
            tracks,
            clips: HashMap::new(),
            duration: 0.0,
        }
    }
}

// --- Manager ---

pub struct TimelineManager {
    pub timeline: Timeline,
}

impl TimelineManager {
    pub fn new() -> Self {
        Self {
            timeline: Timeline::new(),
        }
    }

    pub fn add_clip(&mut self, mut clip: Clip) {
        // Ensure ID
        if clip.id.is_empty() {
            clip.id = Uuid::new_v4().to_string();
        }

        // Auto-assign Z-Index based on Track ID if not set
        if clip.z_index == 0 {
            clip.z_index = clip.track_id;
        }

        self.timeline.clips.insert(clip.id.clone(), clip);
        self.recalculate_duration();
    }

    pub fn remove_clip(&mut self, clip_id: &str) {
        self.timeline.clips.remove(clip_id);
        self.recalculate_duration();
    }

    pub fn get_active_clips(&self, time: f64) -> Vec<&Clip> {
        let mut active: Vec<&Clip> = self
            .timeline
            .clips
            .values()
            .filter(|c| c.is_active(time))
            .filter(|c| {
                // Check if track is muted
                self.timeline
                    .tracks
                    .get(&c.track_id)
                    .map(|t| !t.is_muted)
                    .unwrap_or(true) // Default to visible if track missing
            })
            .collect();

        // Sort by Z-Index (Painter's Algorithm)
        active.sort_by_key(|c| c.z_index);
        active
    }

    fn recalculate_duration(&mut self) {
        let max_end = self
            .timeline
            .clips
            .values()
            .map(|c| c.start + c.duration)
            .fold(0.0_f64, f64::max);
        self.timeline.duration = max_end;
    }
}
