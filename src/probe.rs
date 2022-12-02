#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Output {
    pub format: serde_json::Value,
    pub streams: Vec<serde_json::Value>,
    pub path: String,
}

pub fn ffprobe_file(path: &String, ffprobe_path: &String) -> Result<Output, String> {
    // Run ffprobe
    let output = std::process::Command::new(ffprobe_path)
        .arg("-v")
        .arg("quiet")
        .arg("-print_format")
        .arg("json")
        .arg("-show_format")
        .arg("-show_streams")
        .arg(path)
        .output()
        .expect("Failed to execute ffprobe");

    // Parse ffprobe output
    let ffprobe_output: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();

    crate::debug!("ffprobe output: {:#?}", ffprobe_output);
    crate::info!("Parsed file: {}", path);

    let streams = ffprobe_output["streams"].as_array();

    match streams {
        Some(_) => {
            let output = Output {
                format: ffprobe_output["format"].clone(),
                streams: ffprobe_output["streams"].as_array().unwrap().to_vec(),
                path: path.clone(),
            };
            return Ok(output);
        }
        None => {
            let err = format!("No streams found for file {}", path);
            crate::warn!("{}", err);
            return Err(err);
        }
    }
}
