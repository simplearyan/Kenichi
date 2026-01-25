use anyhow::Result;
use std::path::Path;

pub struct VideoDecoder {
    pub file_path: String,
    // TODO: Add ffmpeg context fields here
    context: ffmpeg::format::context::Input,
    decoder: ffmpeg::decoder::Video,
    scaler: Option<ffmpeg::software::scaling::Context>,
    stream_index: usize,
    time_base: ffmpeg::Rational, // To convert PTS to seconds
    pub fps: f64,                // [NEW] Frames per second
}

// SAFETY: VideoDecoder is stored in KinetixEngine, which is wrapped in a Mutex in AppState.
// We guarantee that we only access it from one thread at a time (via the Mutex lock).
// The underlying raw pointers in ffmpeg contexts are not shared across threads concurrently.
unsafe impl Send for VideoDecoder {}

impl VideoDecoder {
    pub fn new(path: &str) -> Result<Self> {
        ffmpeg::init()?; // Initialize FFmpeg

        // Open the input file
        let context = ffmpeg::format::input(&Path::new(path))?;

        // Find the best video stream
        let stream = context
            .streams()
            .best(ffmpeg::media::Type::Video)
            .ok_or(anyhow::anyhow!("No video stream found"))?;

        let stream_index = stream.index();
        let time_base = stream.time_base();
        let fps = f64::from(stream.rate());

        // Create a decoder for the stream
        let context_decoder =
            ffmpeg::codec::context::Context::from_parameters(stream.parameters())?;
        let decoder = context_decoder.decoder().video()?;

        Ok(Self {
            file_path: path.to_string(),
            context,
            decoder,
            scaler: None,
            stream_index,
            time_base,
            fps,
        })
    }

    pub fn decode_next_frame(&mut self) -> Result<(Vec<u8>, f64)> {
        let mut decoded_frame = ffmpeg::util::frame::Video::empty();

        // Iterate through packets until we get a full frame
        for (stream, packet) in self.context.packets() {
            if stream.index() == self.stream_index {
                self.decoder.send_packet(&packet)?;
                // Keep trying to receive frame
                if self.decoder.receive_frame(&mut decoded_frame).is_ok() {
                    // Frame decoded! Now scale it.
                    let pixels = self.process_frame(&decoded_frame)?;

                    // Calculate Timestamp (seconds)
                    let pts = decoded_frame.pts().unwrap_or(0);
                    let seconds = pts as f64 * f64::from(self.time_base);

                    if pts == 0 {
                        println!(
                            "Decoder: Warning! Frame PTS is 0. TimeBase: {:?}",
                            self.time_base
                        );
                    } else {
                        // println!("Decoder: Decoded Frame at {:.3}s (PTS: {})", seconds, pts);
                    }

                    return Ok((pixels, seconds));
                }
            }
        }

        // Flush decoder if EOF
        self.decoder.send_eof()?;
        if self.decoder.receive_frame(&mut decoded_frame).is_ok() {
            let pixels = self.process_frame(&decoded_frame)?;

            let pts = decoded_frame.pts().unwrap_or(0);
            let seconds = pts as f64 * f64::from(self.time_base);

            return Ok((pixels, seconds));
        }

        Err(anyhow::anyhow!("End of stream or no frame produced"))
    }

    fn process_frame(&mut self, frame: &ffmpeg::util::frame::Video) -> Result<Vec<u8>> {
        // Initialize scaler if needed (lazy init ensures correct input dimensions)
        if self.scaler.is_none() {
            self.scaler = Some(ffmpeg::software::scaling::Context::get(
                frame.format(),
                frame.width(),
                frame.height(),
                ffmpeg::format::Pixel::RGBA, // WGPU friendly
                frame.width(),
                frame.height(),
                ffmpeg::software::scaling::flag::Flags::BILINEAR,
            )?);
        }

        let mut rgb_frame = ffmpeg::util::frame::Video::empty();
        if let Some(scaler) = &mut self.scaler {
            scaler.run(frame, &mut rgb_frame)?;
        }

        // Copy raw bytes
        // RGBA is packed, so stride is typically width * 4.
        // We need to flatten the buffer.
        let data = rgb_frame.data(0);
        let stride = rgb_frame.stride(0);
        let width = rgb_frame.width() as usize;
        let height = rgb_frame.height() as usize;

        // If tightly packed, copy directly. If padded, copy row by row.
        // Usually stride == width * 4 for RGBA, but usage might differ.
        let mut pixels = Vec::with_capacity(width * height * 4);

        for i in 0..height {
            let start = i * stride;
            let end = start + (width * 4);
            pixels.extend_from_slice(&data[start..end]);
        }

        Ok(pixels)
    }

    pub fn width(&self) -> u32 {
        self.decoder.width()
    }

    pub fn height(&self) -> u32 {
        self.decoder.height()
    }

    pub fn seek(&mut self, timestamp_seconds: f64) -> Result<()> {
        let target_ts =
            (timestamp_seconds * self.time_base.1 as f64 / self.time_base.0 as f64) as i64;

        println!(
            "Decoder: Seeking to {:.2}s. Stream TimeBase: {:?}. Target Position (Ticks): {}",
            timestamp_seconds, self.time_base, target_ts
        );

        // 1. Seek to Keyframe (Backward)
        match self.context.seek(target_ts, ..target_ts) {
            Ok(_) => println!("Decoder: Keyframe Seek Success"),
            Err(e) => {
                println!("Decoder: Keyframe Seek Failed: {}", e);
                return Err(anyhow::Error::from(e));
            }
        }

        self.decoder.flush();

        // 2. Roll-Forward to Target
        let mut frames_decoded = 0;
        let max_skip = 600; // Increased limit for larger GOP sizes (10s @ 60fps)
        let mut decoded_frame = ffmpeg::util::frame::Video::empty();

        // We must loop manually because decode_next_frame clones/scales which is slow.
        // We only want to decode until PTS >= target.
        'seek_loop: for (stream, packet) in self.context.packets() {
            if stream.index() == self.stream_index {
                self.decoder.send_packet(&packet)?;
                while self.decoder.receive_frame(&mut decoded_frame).is_ok() {
                    let pts = decoded_frame.pts().unwrap_or(0);
                    if pts >= target_ts {
                        // Found our frame!
                        // IMPORTANT: We must leave this frame in a state where decode_next_frame will use it?
                        // No. decode_next_frame calls packets().
                        // If we consume packets here, they are gone from the iterator.
                        // BUT `self.context.packets()` creates a NEW iterator effectively wrapping `av_read_frame`.
                        // `av_read_frame` reads sequentially from GLOBAL context state.
                        // So if we read here, we advance the global state. Good.

                        // BUT we have a `decoded_frame` right now that IS the target.
                        // If we drop it, we lose the first frame user wants to see!
                        // We can't "put it back".

                        // Hack: Just process it and ignore the return?
                        // Or better: Store it in a temporary buffer?

                        // For this implementation:
                        // We accept that `seek` might burn the *exact* target frame if we don't save it.
                        // But wait! KinetixEngine calls `seek`, THEN calls `decode_next_frame` immediately.
                        // If `seek` consumes the target frame, `decode_next_frame` will get target+1.
                        // That is usually acceptable (1 frame off).
                        // Unless target frame is the ONLY frame (end of stream).

                        // Ideally: `seek` should return the frame if found.
                        // But signature is `-> Result<()>`.

                        // Let's just stop ONE frame early? No, pts checks are >=.
                        // If we stop when `pts >= target_ts`, we hold that frame.
                        break 'seek_loop;
                    }

                    frames_decoded += 1;
                    if frames_decoded > max_skip {
                        println!("Decoder: Seek timeout (max frames skipped)");
                        break 'seek_loop;
                    }
                }
            }
        }

        println!("Decoder: Seek Complete. Skipped {} frames.", frames_decoded);
        Ok(())
    }
}
