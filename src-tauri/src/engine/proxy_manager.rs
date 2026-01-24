use std::path::{Path, PathBuf};
use tokio::process::Command;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct ProxyManager {
    // Track active jobs to avoid duplicate work
    active_jobs: Arc<Mutex<HashMap<String, bool>>>,
}

impl ProxyManager {
    pub fn new() -> Self {
        Self {
            active_jobs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_proxy_path(&self, source: &str) -> PathBuf {
        let source_path = Path::new(source);
        let stem = source_path.file_stem().unwrap_or_default();
        let mut proxy_name = stem.to_os_string();
        proxy_name.push(".kenichi_proxy.mp4");
        
        // Save proxy next to original file (simple for now)
        source_path.with_file_name(proxy_name)
    }

    /// Returns the path to the proxy file (if ready) or the original source.
    /// Triggers generation in the background if needed.
    pub async fn ensure_proxy(&self, source: String) -> String {
        let proxy_path = self.get_proxy_path(&source);
        
        if proxy_path.exists() {
            return proxy_path.to_string_lossy().to_string();
        }

        // Check if already generating
        let mut jobs = self.active_jobs.lock().await;
        if jobs.contains_key(&source) {
            println!("Proxy generation already in progress for: {}", source);
            return source; // Return original while processing
        }
        
        // Mark as working
        jobs.insert(source.clone(), true);
        drop(jobs); // Release lock before awaiting

        // Spawn background task
        let manager = self.clone();
        let source_clone = source.clone();
        
        tauri::async_runtime::spawn(async move {
            if let Err(e) = manager.generate_proxy(&source_clone).await {
                eprintln!("Failed to generate proxy for {}: {}", source_clone, e);
            }
            // Cleanup job marker
            let mut jobs = manager.active_jobs.lock().await;
            jobs.remove(&source_clone);
        });

        source // Return original while waiting
    }

    async fn generate_proxy(&self, source: &str) -> anyhow::Result<()> {
        let proxy_path = self.get_proxy_path(source);
        println!("Starting Proxy Generation: {:?}", proxy_path);

        // Resolve Sidecar Path
        // In dev: src-tauri/bin/ffmpeg...
        // In prod: handled by Tauri sidecar injection, but we call manual binary here for control
        let cwd = std::env::current_dir()?;
        let sidecar_path = cwd.join("src-tauri/bin/ffmpeg-x86_64-pc-windows-msvc.exe");
        
        let ffmpeg_cmd = if sidecar_path.exists() {
            sidecar_path.to_string_lossy().to_string()
        } else {
            "ffmpeg".to_string() // Fallback to PATH
        };

        let status = Command::new(ffmpeg_cmd)
            .arg("-i").arg(source)
            .arg("-vf").arg("scale=-2:480") // Fixed 480p height
            .arg("-c:v").arg("libx264")
            .arg("-preset").arg("ultrafast")
            .arg("-tune").arg("fastdecode")
            .arg("-y")
            .arg(&proxy_path)
            .status() // Await the process
            .await?;

        if status.success() {
            println!("Proxy Complete: {:?}", proxy_path);
            Ok(())
        } else {
            anyhow::bail!("FFmpeg exited with error code")
        }
    }
}
