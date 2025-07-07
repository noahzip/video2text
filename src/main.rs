use std::env;
use std::fs;
use std::process::Command;
use sanitize_filename::sanitize;
use std::path::Path;

mod formatter;

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("❌ 用法: cargo run -- <视频链接>");
        return;
    }
    let url = &args[1];

    let tmp_dir = "result/tmp";
    log("🧹 清理临时目录...");
    clean_temp_dir(tmp_dir);
    fs::create_dir_all(tmp_dir).expect("❌ 创建临时目录失败");

    log("📥 获取视频标题...");
    let raw_title = get_video_title(url);
    let safe_title = sanitize(&raw_title);
    println!("📌 视频标题: {}", raw_title);
    println!("📁 输出文件名: result/{}.txt", safe_title);

    let audio_path = format!("{}/audio.mp3", tmp_dir);
    let json_path = format!("{}/output.json", tmp_dir);
    let output_txt_path = format!("result/{}.txt", safe_title);

    log("🎧 下载音频...");
    download_audio(url, &audio_path);

    log("🧠 调用 Whisper 识别语音...");
    convert_audio_to_text(&audio_path, &json_path);

    log("📝 生成字幕文本文件...");
    formatter::generate_lrc(&json_path, &output_txt_path);

    clean_temp_dir(tmp_dir);
    log("✅ 完成！字幕文件已保存");
    println!("📄 路径: {}", output_txt_path);
}

/// 日志打印封装
fn log(msg: &str) {
    println!("\n==== {} ====\n", msg);
}

/// 删除临时目录
fn clean_temp_dir(dir: &str) {
    if Path::new(dir).exists() {
        fs::remove_dir_all(dir).expect("❌ 删除临时目录失败");
    }
}

/// 获取视频标题并返回
fn get_video_title(url: &str) -> String {
    let output = Command::new("yt-dlp")
        .args(["--get-title", url])
        .output()
        .expect("❌ 获取视频标题失败");

    if !output.status.success() {
        panic!(
            "❌ 获取视频标题失败: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

/// 使用 yt-dlp 下载音频
fn download_audio(url: &str, audio_path: &str) {
    let status = Command::new("yt-dlp")
        .args([
            "-f", "bestaudio",
            "-x",
            "--audio-format", "mp3",
            "-o", audio_path,
            url,
        ])
        .status()
        .expect("❌ 执行 yt-dlp 失败");
    if !status.success() {
        panic!("❌ 音频下载失败");
    }
}

/// 调用 whisper_transcribe.py 进行语音转录
fn convert_audio_to_text(audio_path: &str, json_path: &str) {
    let python_cmd = if cfg!(target_os = "windows") { "python" } else { "python3" };

    let output = Command::new(python_cmd)
        .args(["whisper_transcribe.py", audio_path, "--output", json_path])
        .output()
        .expect("❌ 调用 Whisper 失败");

    if !output.status.success() {
        eprintln!("❌ Whisper 转录失败:\n{}", String::from_utf8_lossy(&output.stderr));
        panic!();
    }
}
