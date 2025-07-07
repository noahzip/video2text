use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Segment {
    start: f64,
    text: String,
}

#[derive(Deserialize)]
struct WhisperResult {
    segments: Vec<Segment>,
}

pub fn generate_lrc(json_path: &str, lrc_path: &str) {
    let data = fs::read_to_string(json_path).expect("读取 JSON 失败");
    let parsed: WhisperResult = serde_json::from_str(&data).expect("解析 JSON 失败");

    let mut lrc = String::new();
    for seg in parsed.segments {
        let minutes = (seg.start / 60.0).floor() as u64;
        let seconds = seg.start % 60.0;
        lrc += &format!("[{:02}:{:05.2}] {}\n", minutes, seconds, seg.text.trim());
    }

    fs::write(lrc_path, lrc).expect("写入 LRC 失败");
}