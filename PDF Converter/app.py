from flask import Flask, render_template, request, send_file
from PIL import Image
import os
import uuid
import shutil
import threading
from datetime import datetime, timedelta
import time
from io import BytesIO

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
    image_list = []

    # Pillow version compatibility
    try:
        resample_filter = Image.Resampling.LANCZOS
    except AttributeError:
        resample_filter = Image.ANTIALIAS

    for file in files:
        try:
            img = Image.open(file.stream)
            if img.mode != 'RGB':
                img = img.convert('RGB')

            # Resize if too large
            max_width = 1500
            if img.width > max_width:
                ratio = max_width / float(img.width)
                height = int(img.height * ratio)
                img = img.resize((max_width, height), resample_filter)

            image_list.append(img)

        except Exception as e:
            print(f"Error reading image: {e}")
            continue

    if image_list:
        pdf_bytes = BytesIO()
        image_list[0].save(pdf_bytes, format='PDF', save_all=True, append_images=image_list[1:])
        pdf_bytes.seek(0)
        return send_file(pdf_bytes, mimetype='application/pdf',
                         download_name='converted.pdf', as_attachment=True)

    return "No valid images uploaded."


# Port for Glitch
if __name__ == '__main__':
    port = int(os.environ.get("PORT", 3000))
    app.run(host='0.0.0.0', port=port, debug=True)
