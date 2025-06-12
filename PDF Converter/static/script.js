document.addEventListener("DOMContentLoaded", () => {
    const dropArea = document.getElementById("drop-area");
    const fileInput = document.getElementById("fileElem");
    const fileSelect = document.getElementById("fileSelect");
    const uploadForm = document.getElementById("uploadForm");
    const spinner = document.getElementById("spinner");


    function updatePreview(files) {
        const fileCount = document.getElementById("file-count");
        const thumbs = document.getElementById("thumbs");
        thumbs.innerHTML = "";

        if (!files.length) {
            fileCount.textContent = "No files selected";
            return;
        }

        if (files.length > 20) {
            fileCount.textContent = "⚠️ Too many files! Max: 20";
            return;
        }

        fileCount.textContent = `${files.length} image(s) selected`;

        [...files].forEach(file => {
            if (file.type.startsWith("image/")) {
                const reader = new FileReader();
                reader.onload = e => {
                    const img = document.createElement("img");
                    img.src = e.target.result;
                    thumbs.appendChild(img);
                };
                reader.readAsDataURL(file);
            }
        });
    }


    // ✅ Show previews when files are selected manually
    fileInput.addEventListener("change", () => {
        updatePreview(fileInput.files);
    });

    // ✅ Drag events
    ["dragenter", "dragover"].forEach(event =>
        dropArea.addEventListener(event, e => {
            e.preventDefault();
            dropArea.classList.add("hover");
        })
    );

    ["dragleave", "drop"].forEach(event =>
        dropArea.addEventListener(event, e => {
            e.preventDefault();
            dropArea.classList.remove("hover");
        })
    );

    // ✅ Handle drop
    dropArea.addEventListener("drop", e => {
        const dt = e.dataTransfer;
        const files = dt.files;

        if (files.length > 20) {
            alert("Please upload 20 or fewer images.");
            return;
        }

        const dataTransfer = new DataTransfer();
        for (let i = 0; i < files.length; i++) {
            dataTransfer.items.add(files[i]);
        }

        fileInput.files = dataTransfer.files;
        updatePreview(fileInput.files);
    });

    // ✅ Click to browse
    fileSelect.addEventListener("click", () => fileInput.click());

    // ✅ Form submission with spinner and fetch
    uploadForm.addEventListener("submit", async (e) => {
        e.preventDefault();
        const files = fileInput.files;

        if (!files.length) {
            alert("Please select some images.");
            return;
        }

        if (files.length > 20) {
            alert("Maximum 20 images allowed.");
            return;
        }

        spinner.classList.remove("hidden");

        const formData = new FormData();
        for (let file of files) {
            formData.append("images", file);
        }

        try {
            const response = await fetch("/upload", {
                method: "POST",
                body: formData
            });

            if (!response.ok) throw new Error("Upload failed");

            const blob = await response.blob();
            const url = window.URL.createObjectURL(blob);
            const link = document.createElement("a");
            link.href = url;
            link.download = "converted.pdf";
            document.body.appendChild(link);
            link.click();
            link.remove();
            window.URL.revokeObjectURL(url);
        } catch (err) {
            alert("Something went wrong.");
        } finally {
            spinner.classList.add("hidden");
        }
    });
});
