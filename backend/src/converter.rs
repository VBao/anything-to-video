use std::process::Command;
use std::path::Path;
use std::io;

pub struct VideoConverter {
    ffmpeg_path: String,
}

impl VideoConverter {
    pub fn new(ffmpeg_path: &str) -> Self {
        VideoConverter {
            ffmpeg_path: ffmpeg_path.to_string(),
        }
    }

    pub fn convert(&self, input_path: &Path, output_path: &Path, _format: &str) -> io::Result<()> {
        let status = Command::new(&self.ffmpeg_path)
            .arg("-i")
            .arg(input_path)
            .arg(output_path)
            .arg("-y") // Overwrite output file
            .status()?;

        if status.success() {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "FFmpeg conversion failed"))
        }
    }
}
