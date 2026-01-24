use super::KinetixEngine;

impl KinetixEngine {
    pub fn load_video(&mut self, path: &str) {
        println!("Engine: Loading Video into Texture: {}", path);
        self.current_file = Some(path.to_string());

        // TODO: In real implementation:
        // 1. Open video with FFmpeg
        // 2. Decode first frame
        // 3. Upload to WGPU Texture

        // For now, we just store the path. The shader will keep rendering the "empty" quad (gradient).
    }
}
