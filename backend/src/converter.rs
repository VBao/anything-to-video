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
        // We could use _format to force the output format with -f, but ffmpeg usually detects it from extension.
        // However, passing it explicitly can be safer if extension doesn't match format name perfectly.
        // For simplicity and standard usage, we rely on output_path extension which we constructed from _format.

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
