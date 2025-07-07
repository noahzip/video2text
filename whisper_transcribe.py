import whisper
import sys
import json

if len(sys.argv) < 2:
    print("Usage: python whisper_transcribe.py <audio_path> [--output output.json]")
    exit(1)

audio_path = sys.argv[1]
output_path = "output.json"

if len(sys.argv) > 3 and sys.argv[2] == "--output":
    output_path = sys.argv[3]

model = whisper.load_model("base")
result = model.transcribe(audio_path, fp16=False)

with open(output_path, "w", encoding="utf-8") as f:
    json.dump(result, f, ensure_ascii=False, indent=2)
