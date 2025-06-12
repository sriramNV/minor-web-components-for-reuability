from flask import Flask, render_template, request, send_file
from PIL import Image
import os
import uuid
import shutil
import threading
from datetime import datetime, timedelta
import time

app = Flask(__name__)
UPLOAD_FOLDER = 'uploads'
os.makedirs(UPLOAD_FOLDER, exist_ok=True)

# Cleanup thread (runs every 5 mins, deletes folders older than 10 mins)
def start_background_cleaner():
    def cleaner():
        while True:
            now = datetime.now()
            for folder in os.listdir(UPLOAD_FOLDER):
                path = os.path.join(UPLOAD_FOLDER, folder)
                if os.path.isdir(path):
                    created = datetime.fromtimestamp(os.path.getctime(path))
                    if now - created > timedelta(minutes=10):
                        try:
                            shutil.rmtree(path)
                            print(f"Deleted {path}")
                        except Exception as e:
                            print(f"Failed to delete {path}: {e}")
            time.sleep(300)
    threading.Thread(target=cleaner, daemon=True).start()

start_background_cleaner()

@app.route('/')
def index():
    return render_template('index.html')

@app.route('/upload', methods=['POST'])
def upload():
    files = request.files.getlist('images')
    temp_id = str(uuid.uuid4())
    temp_folder = os.path.join(UPLOAD_FOLDER, temp_id)
    os.makedirs(temp_folder, exist_ok=True)

    image_list = []
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

# Port for Glitch
if __name__ == '__main__':
    port = int(os.environ.get("PORT", 3000))
    app.run(host='0.0.0.0', port=port, debug=True)
