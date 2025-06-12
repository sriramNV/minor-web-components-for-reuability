from flask import Flask, render_template, request, send_file
from PIL import Image
import os
import uuid
import shutil
import threading

app = Flask(__name__)

UPLOAD_FOLDER = 'uploads'
os.makedirs(UPLOAD_FOLDER, exist_ok=True)

# Background cleanup thread
def start_background_cleaner(folder, max_age_minutes=10, interval_seconds=300):
    def cleaner():
        import time
        from datetime import datetime, timedelta
        while True:
            now = datetime.now()
            for subfolder in os.listdir(folder):
                path = os.path.join(folder, subfolder)
                if os.path.isdir(path):
                    creation_time = datetime.fromtimestamp(os.path.getctime(path))
                    if now - creation_time > timedelta(minutes=max_age_minutes):
                        try:
                            shutil.rmtree(path)
                        except Exception as e:
                            print(f"Error deleting {path}: {e}")
            time.sleep(interval_seconds)
    threading.Thread(target=cleaner, daemon=True).start()

start_background_cleaner(UPLOAD_FOLDER)

@app.route('/')
def index():
    return render_template('index.html')

@app.route('/upload', methods=['POST'])
def upload():
    files = request.files.getlist('images')
    image_list = []
    temp_folder = os.path.join(UPLOAD_FOLDER, str(uuid.uuid4()))
    os.makedirs(temp_folder, exist_ok=True)

    for file in files:
        filepath = os.path.join(temp_folder, file.filename)
        file.save(filepath)
        img = Image.open(filepath)
        if img.mode != 'RGB':
            img = img.convert('RGB')
        image_list.append(img)

    if image_list:
        pdf_path = os.path.join(temp_folder, 'output.pdf')
        image_list[0].save(pdf_path, save_all=True, append_images=image_list[1:])
        return send_file(pdf_path, as_attachment=True)

    return "No valid images uploaded."

# ✅ Local development entry point only
if __name__ == '__main__':
    app.run(debug=True)  # ONLY for local testing
