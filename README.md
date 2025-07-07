# video2text

自动下载 Bilibili 视频音频并转录为字幕的工具。

> 想通过英文字幕练习口语，但 Bilibili 上下载的字幕格式经常重复不便于阅读，于是写了这个工具自动提取音频并转录生成更干净的字幕文本。

------

## 项目结构

```
├── Cargo.lock
├── Cargo.toml
├── README.md
├── install/
│   └── install.sh          # 依赖安装脚本，包含 python3 pip whisper torch yt-dlp 等校验与安装
├── src/
│   ├── formatter.rs        # 将 whisper 转录结果转换为 LRC 格式
│   ├── main.rs             # 核心逻辑：音频下载、语音识别、字幕文件生成
└── whisper_transcribe.py   # whisper 调用脚本
```

------

## 主要流程

```
             +------------------------+
             |    用户输入视频链接     |
             +----------+-------------+
                        |
             +----------v-------------+
             |  Rust：获取视频标题    |
             +----------+-------------+
                        |
             +----------v-------------+
             | Rust：清理并创建临时目录|
             +----------+-------------+
                        |
             +----------v-------------+
             | Rust：调用 yt-dlp 下载 |
             |       最佳音频(mp3)    |
             +----------+-------------+
                        |
             +----------v-------------+
             |  调用 Python Whisper   |
             | whisper_transcribe.py  |
             |    转录音频为 JSON     |
             +----------+-------------+
                        |
             +----------v-------------+
             | Rust：读取 JSON        |
             | 调用 formatter 模块生成 |
             |      字幕文本文件      |
             +----------+-------------+
                        |
             +----------v-------------+
             | Rust：清理临时目录     |
             +----------+-------------+
                        |
             +----------v-------------+
             |   输出字幕文本文件路径  |
             +------------------------+
```

------

## 如何编译本项目

### 环境要求

- Linux
- Rust 环境
- Python3（带 pip）

### 克隆项目

```
git clone https://github.com/noahzip/video2text.git
cd video2text
```

### 安装依赖

```
bash ./install/install.sh
```

该脚本会自动检测并安装 `python3`, `pip`, `whisper`, `torch`, `yt-dlp` 等依赖。

------

## 使用

```
cargo run -- https://www.bilibili.com/video/BV...
```

运行后，会自动：

- 获取视频标题
- 下载音频并转成 mp3
- 调用 Whisper 转录音频
- 生成字幕文本文件，存放在 `result/视频标题.txt`

------

## 示例输出

```
==== 🧹 清理临时目录... ====


==== 📥 获取视频标题... ====

📌 视频标题: 当地人推荐啥我吃啥？一天吃了11种…
📁 输出文件名: result/当地人推荐啥我吃啥？一天吃了11种….txt

==== 🎧 下载音频... ====

[BiliBili] Extracting URL: https://www.bilibili.com/video/BV11h3sztEfv/?spm_id_from=333.1007.tianma.1-3-3.click
[BiliBili] BV11h3sztEfv: Downloading webpage
[BiliBili] BV11h3sztEfv: Extracting videos in anthology
...
[download] Destination: result/tmp/audio.m4a
[download] 100% of   15.37MiB in 00:00:03 at 4.56MiB/s
[ExtractAudio] Destination: result/tmp/audio.mp3
Deleting original file result/tmp/audio.m4a (pass -k to keep)

==== 🧠 调用 Whisper 识别语音... ====


==== 📝 生成字幕文本文件... ====


==== ✅ 完成！字幕文件已保存 ====

📄 路径: result/当地人推荐啥我吃啥？一天吃了11种….txt
```

------

## 生成的字幕示例

```
[00:00.00] Coming up in today's food adventure,
[00:01.32] I'm letting locals pick my next meal for an entire day.
[00:04.48] And I think I need to do this more often
[00:06.44] because the food was absolutely incredible.
```

## 交叉编译生成windows可执行文件

### 步骤一：安装 `x86_64-pc-windows-gnu` 工具链

```
rustup target add x86_64-pc-windows-gnu
```

### 步骤二：安装 MinGW-w64（提供 Windows 编译器）

```
sudo apt update
sudo apt install mingw-w64
```

> 确保安装的是 `x86_64-w64-mingw32-gcc` 可用。

### 步骤三：构建 Windows `.exe`

在 WSL 项目目录下运行：

```
cargo build --release --target x86_64-pc-windows-gnu
```

### 构建完成后，文件位置：

```
target/x86_64-pc-windows-gnu/release/video2text.exe
```

### windows环境下运行需要如下条件

```
1.移动video2text.exe到任意目录下
2.copy whisper_transcribe.py 到当前目录下
3.下载如下可执行文件到当前目录下：ffmpeg: ffmpeg.exe ffmrobe.exe 、yt-dlp.exe
4.需要Python环境和pip配置
5.powershell 执行命令 .\video2text.exe "bilibil/youtube视频链接"
```
