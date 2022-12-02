use crate::SimpilifiedOutput;

pub fn generate_simple_output(result: Vec<crate::probe::Output>) -> Vec<crate::SimpilifiedOutput> {
    let mut simplified: Vec<crate::SimpilifiedOutput> = vec![];

    for item in result {
        let mut video_tracks: Vec<serde_json::Value> = vec![];
        let mut audio_tracks: Vec<serde_json::Value> = vec![];
        let mut subtitle_tracks: Vec<serde_json::Value> = vec![];
        let mut other_tracks: Vec<serde_json::Value> = vec![];

        if item.streams.len() == 0 {
            crate::warn!("No streams found for file {}", item.path);
        }

        for stream in item.streams {
            let stream_type = stream["codec_type"].as_str().unwrap();

            match stream_type {
                "video" => {
                    crate::info!("Found video track");
                    video_tracks.push(stream);
                }
                "audio" => {
                    crate::info!("Found audio track");
                    audio_tracks.push(stream);
                }
                "subtitle" => {
                    crate::info!("Found subtitle track");
                    subtitle_tracks.push(stream);
                }
                _ => {
                    crate::warn!("Unknown stream type: {}", stream_type);
                    other_tracks.push(stream);
                }
            }
        }

        crate::debug!("Video tracks: {:?}", video_tracks);

        // Get extension of file from path

        let path = item.path.clone();
        let item_path = path.clone();

        let extension = path.split('.').last().unwrap();

        let video_codec = match video_tracks.len() {
            0 => "".to_string(),
            _ => video_tracks[0]["codec_name"].as_str().unwrap().to_string(),
        };

        let audio_codec = match audio_tracks.len() {
            0 => "".to_string(),
            _ => audio_tracks[0]["codec_name"].as_str().unwrap().to_string(),
        };

        let pixel_format = match video_tracks.len() {
            0 => "".to_string(),
            _ => video_tracks[0]["pix_fmt"].as_str().unwrap().to_string(),
        };

        let duration = match item.format["duration"].as_str() {
            Some(d) => d.parse::<f64>().unwrap(),
            None => 0.0,
        };

        let metadata = match item.format["tags"].as_object() {
            Some(m) => serde_json::to_string(m).unwrap(),
            None => "".to_string(),
        };

        let filename = item_path.split('/').last().unwrap().to_string();

        let simplified_item = crate::SimpilifiedOutput {
            filename: filename,
            size: item.format["size"]
                .as_str()
                .unwrap()
                .parse::<u64>()
                .unwrap(),
            container_format: extension.to_string(),
            video_tracks: video_tracks.len() as u8,
            audio_tracks: audio_tracks.len() as u8,
            subtitle_tracks: subtitle_tracks.len() as u8,
            other_tracks: other_tracks.len() as u8,
            video_codec,
            audio_codec,
            metadata,
            pixel_format,
            duration,
            path: item_path.to_string(),
        };

        crate::debug!("Simplified item: {:?}", simplified_item);
        simplified.push(simplified_item);
    }
    simplified
}

pub fn write_simplified_output(
    simplified_output: &Vec<SimpilifiedOutput>,
    output_file: &String,
    format: &crate::OutputFormat,
) {
    // Write to file as specified in args.format
    match format {
        crate::OutputFormat::JSON => {
            let json = serde_json::to_string(&simplified_output)
                .expect("Unable to serialize simplified output to JSON");
            crate::debug!("{}", json);
            std::fs::write(output_file.clone(), json).expect("Unable to write to file");
            crate::info!("JSON output written to {}", output_file);
        }
        crate::OutputFormat::XML => {
            crate::warn!("XML output not implemented yet");
        }
        crate::OutputFormat::CSV => {
            let mut wtr =
                csv::Writer::from_path(output_file.clone()).expect("Unable to create CSV writer");
            for row in simplified_output {
                wtr.serialize(row).expect("Unable to write CSV row");
            }
            wtr.flush().expect("Unable to flush CSV writer");
            crate::info!("CSV file written to {}", output_file);
        }
    }
}

pub fn write_output(
    output: &Vec<crate::probe::Output>,
    output_file: &String,
    format: &crate::OutputFormat,
) {
    // Write to file as specified in args.format
    match format {
        crate::OutputFormat::JSON => {
            let json = serde_json::to_string(&output).expect("Unable to serialize output to JSON");
            std::fs::write(output_file.clone(), json).expect("Unable to write to file");
            crate::info!("JSON output written to {}", output_file);
        }
        crate::OutputFormat::XML => {
            crate::warn!("XML output not implemented yet");
        }
        crate::OutputFormat::CSV => {
            crate::warn!("CSV output is implemented only for not --simplified data. Use --format JSON instead");
        }
    }
}
