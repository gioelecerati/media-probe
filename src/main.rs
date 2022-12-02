use log::{debug, info, warn};

use clap::Parser;

pub mod probe;
pub mod proc;
pub mod walk;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    XML,
    JSON,
    CSV,
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum, short, long, default_value_t = OutputFormat::JSON)]
    format: OutputFormat,

    // Folder to look for media files
    #[arg(long, default_value = ".")]
    folder: String,

    // Path to write output file
    #[arg(long, default_value = "output")]
    output: String,

    // Recursive
    #[arg(short, long)]
    recursive: bool,

    // Simplified
    #[arg(short, long)]
    simplified: bool,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SimpilifiedOutput {
    filename: String,
    size: u64,
    container_format: String,
    video_tracks: u8,
    audio_tracks: u8,
    subtitle_tracks: u8,
    other_tracks: u8,
    video_codec: String,
    audio_codec: String,
    pixel_format: String,
    duration: f64,
    path: String,
    metadata: String,
}

fn main() {
    env_logger::init();

    // Get ffprobe path
    let ffprobe_path = match which::which("ffprobe") {
        Ok(path) => path,
        Err(_) => panic!("ffprobe is not installed in your system or not available in your PATH"),
    };

    info!("ffprobe path found: {}", ffprobe_path.display());

    let ffprobe_path_string = ffprobe_path.to_string_lossy().to_string();

    // Parsing Clap parameters
    let args = Args::parse();

    debug!("Parsed arguments: {:?}", args);
    info!("Output format: {:?}", args.format);
    info!("Folder: {}", args.folder);
    info!("Recursive: {}", args.recursive);

    // Walk into folder
    let paths = walk::walk_in_folder(&args.folder);

    info!("Found {} files", paths.len());
    debug!("Paths: {:?}", paths);

    let mut result: Vec<probe::Output> = Vec::new();

    for video_file in paths {
        let p = probe::ffprobe_file(&video_file, &ffprobe_path_string);

        match p {
            Ok(output) => result.push(output),
            Err(e) => {
                warn!("Skipping file {}, err={}", video_file, e);
                continue;
            }
        }
    }

    let mut output_file = args.output;

    if output_file == "output" {
        let format = match args.format {
            OutputFormat::JSON => "json",
            OutputFormat::XML => "xml",
            OutputFormat::CSV => "csv",
        };
        // With date
        output_file = format!(
            "{}-{}.{}",
            output_file,
            chrono::Local::now().format("%Y-%m-%d-%H-%M-%s"),
            format
        );
    }

    if args.simplified {
        info!("Simplified output");
        let simplified_output = proc::generate_simple_output(result);
        proc::write_simplified_output(&simplified_output, &output_file, &args.format);
    } else {
        info!("Full output");
        proc::write_output(&result, &output_file, &args.format);
    }
}
