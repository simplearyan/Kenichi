use anyhow::Result;
use std::path::Path;

pub struct VideoDecoder {
    pub file_path: String,
    // TODO: Add ffmpeg context fields here
    context: ffmpeg::format::context::Input,
    decoder: ffmpeg::decoder::Video,
    scaler: Option<ffmpeg::software::scaling::Context>,
    stream_index: usize,
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
        })
    }

    pub fn decode_next_frame(&mut self) -> Result<Vec<u8>> {
        let mut decoded_frame = ffmpeg::util::frame::Video::empty();

        // Iterate through packets until we get a full frame
        for (stream, packet) in self.context.packets() {
            if stream.index() == self.stream_index {
                self.decoder.send_packet(&packet)?;
                // Keep trying to receive frame
                if self.decoder.receive_frame(&mut decoded_frame).is_ok() {
                    // Frame decoded! Now scale it.
                    return self.process_frame(&decoded_frame);
                }
            }
        }

        // Flush decoder if EOF
        self.decoder.send_eof()?;
        if self.decoder.receive_frame(&mut decoded_frame).is_ok() {
            return self.process_frame(&decoded_frame);
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
}
