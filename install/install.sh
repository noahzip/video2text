#!/bin/bash
set -e

echo "更新pip..."
python3 -m pip install --upgrade pip

echo "安装 Python 依赖: whisper, torch, yt-dlp ..."
pip install --upgrade openai-whisper torch yt-dlp

if command -v ffmpeg >/dev/null 2>&1; then
    echo "ffmpeg 已安装"
else
    echo "检测到未安装 ffmpeg，尝试安装..."

    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if command -v apt-get >/dev/null 2>&1; then
            sudo apt-get update
            sudo apt-get install -y ffmpeg
        elif command -v yum >/dev/null 2>&1; then
            sudo yum install -y ffmpeg
        else
            echo "请手动安装 ffmpeg"
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        if command -v brew >/dev/null 2>&1; then
            brew install ffmpeg
        else
            echo "Homebrew 未安装，请先安装 Homebrew"
            echo "访问 https://brew.sh/"
        fi
    else
        echo "未知操作系统，请手动安装 ffmpeg"
    fi
fi

echo "一键安装完成！"
