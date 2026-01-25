use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClipData {
    pub id: String,
    pub path: String,
    pub start: f64,    // Timeline start time
    pub duration: f64, // Length in seconds
    pub offset: f64,   // Video file offset
}

impl ClipData {
    pub fn is_active(&self, time: f64) -> bool {
        time >= self.start && time < (self.start + self.duration)
    }
}

pub fn get_active_clip(time: f64, clips: &[ClipData]) -> Option<&ClipData> {
    clips.iter().find(|clip| clip.is_active(time))
}
