use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;
use tokio::process::Command as AsyncCommand;

#[derive(Parser)]
#[command(name = "vid2gif")]
#[command(about = "Convert videos to GIFs using ffmpeg")]
#[command(version = "0.1.0")]
struct Args {
    /// Input video file path
    #[arg(short, long)]
    input: PathBuf,

    /// Output GIF file path
    #[arg(short, long)]
    output: PathBuf,

    /// Speed multiplier (0.5 for half speed, 2.0 for double speed)
    #[arg(short, long, default_value = "1.0")]
    speed: f64,

    /// Enable looping for the GIF
    #[arg(long)]
    r#loop: bool,
}

struct VideoConverter {
    ffmpeg_path: String,
}

impl VideoConverter {
    fn new() -> Result<Self> {
        let ffmpeg_path = which::which("ffmpeg")
            .context("ffmpeg not found in PATH. Please install ffmpeg to use this tool.")?
            .to_string_lossy()
            .to_string();

        Ok(Self { ffmpeg_path })
    }

    async fn convert(&self, args: &Args) -> Result<()> {
        // Validate input file exists
        if !args.input.exists() {
            anyhow::bail!("Input file does not exist: {}", args.input.display());
        }

        // Validate speed parameter
        if args.speed <= 0.0 {
            anyhow::bail!("Speed must be greater than 0");
        }

        // Create output directory if it doesn't exist
        if let Some(parent) = args.output.parent() {
            tokio::fs::create_dir_all(parent).await
                .context("Failed to create output directory")?;
        }

        println!("Converting {} to {}", args.input.display(), args.output.display());
        println!("Speed: {}x", args.speed);
        println!("Loop: {}", if args.r#loop { "enabled" } else { "disabled" });

        // Build ffmpeg command
        let mut cmd = AsyncCommand::new(&self.ffmpeg_path);
        
        // Input file
        cmd.arg("-i").arg(&args.input);

        // Build filter complex for optimal GIF conversion
        let fps = if args.speed >= 1.0 {
            (10.0 * args.speed).min(30.0) // Cap at 30fps for high speeds
        } else {
            (10.0 * args.speed).max(5.0) // Minimum 5fps for low speeds
        };

        let filter_complex = format!(
            "fps={},scale=640:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse",
            fps
        );

        cmd.arg("-filter_complex").arg(&filter_complex);

        // Loop setting
        if args.r#loop {
            cmd.arg("-loop").arg("0"); // 0 means infinite loop
        } else {
            cmd.arg("-loop").arg("-1"); // -1 means no loop
        }

        // Output settings
        cmd.arg("-y") // Overwrite output file
            .arg(&args.output);

        println!("Running ffmpeg with optimized GIF conversion settings...");

        // Execute the command
        let output = cmd.output().await
            .context("Failed to execute ffmpeg command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("ffmpeg failed with error:\n{}", stderr);
        }

        // Get file sizes for comparison
        let input_size = tokio::fs::metadata(&args.input).await?.len();
        let output_size = tokio::fs::metadata(&args.output).await?.len();

        println!("✅ Conversion completed successfully!");
        println!("Input size:  {} KB", input_size / 1024);
        println!("Output size: {} KB", output_size / 1024);
        println!("Compression: {:.1}%", 
                 (1.0 - (output_size as f64 / input_size as f64)) * 100.0);

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Validate file extensions
    if let Some(ext) = args.input.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        if !["webm", "mp4", "avi", "mov", "mkv", "flv", "wmv"].contains(&ext_str.as_str()) {
            println!("⚠️  Warning: Input file extension '{}' may not be supported", ext_str);
        }
    }

    if let Some(ext) = args.output.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        if ext_str != "gif" {
            println!("⚠️  Warning: Output file extension should be '.gif'");
        }
    }

    let converter = VideoConverter::new()
        .context("Failed to initialize video converter")?;

    converter.convert(&args).await
        .context("Video conversion failed")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_args_parsing() {
        // Test basic argument parsing
        let args = Args::parse_from(&[
            "vid2gif",
            "--input", "test.webm",
            "--output", "test.gif",
            "--speed", "2.0",
            "--loop"
        ]);

        assert_eq!(args.input, PathBuf::from("test.webm"));
        assert_eq!(args.output, PathBuf::from("test.gif"));
        assert_eq!(args.speed, 2.0);
        assert!(args.r#loop);
    }

    #[test]
    fn test_default_speed() {
        let args = Args::parse_from(&[
            "vid2gif",
            "--input", "test.webm",
            "--output", "test.gif"
        ]);

        assert_eq!(args.speed, 1.0);
        assert!(!args.r#loop);
    }
}
