document.addEventListener("DOMContentLoaded", () => {
    const dropArea = document.getElementById("drop-area");
    const fileInput = document.getElementById("fileElem");
    const fileSelect = document.getElementById("fileSelect");
    const uploadForm = document.getElementById("uploadForm");
    const spinner = document.getElementById("spinner");
    const thumbs = document.getElementById("thumbs");
    const fileCount = document.getElementById("file-count");


    // After your DOMContentLoaded or equivalent init:

    // const thumbs = document.getElementById("thumbs");

    // Initialize Sortable
    const sortable = new Sortable(thumbs, {
        animation: 150, // Smooth animation duration in ms
        ghostClass: "sortable-ghost", // class for the dragged item

        onEnd: function (evt) {
            const movedItem = filesArray.splice(evt.oldIndex, 1)[0];
            filesArray.splice(evt.newIndex, 0, movedItem);

            // Just update data-index attributes on current DOM elements
            const wrappers = thumbs.querySelectorAll(".thumb-wrapper");
            wrappers.forEach((el, i) => {
                el.setAttribute("data-index", i);
            });
        }

    });



    let filesArray = [];
    let dragSrcEl = null;

    // Prevent default drag behaviors on document
    ["dragenter", "dragover", "dragleave", "drop"].forEach(eventName => {
        document.body.addEventListener(eventName, e => e.preventDefault());
    });

    function handleDragStart(e) {
        dragSrcEl = this;
        this.classList.add("dragging");
        e.dataTransfer.effectAllowed = "move";
    }

    function handleDragOver(e) {
        e.preventDefault();
        e.dataTransfer.dropEffect = "move";
        return false;
    }

    function handleDrop(e) {
        e.preventDefault();
        if (dragSrcEl !== this) {
            const fromIndex = Number(dragSrcEl.getAttribute("data-index"));
            const toIndex = Number(this.getAttribute("data-index"));

            // Move dragged file in array
            const movedFile = filesArray.splice(fromIndex, 1)[0];
            filesArray.splice(toIndex, 0, movedFile);

            // Remove dragging class from old element
            dragSrcEl.classList.remove("dragging");

            dragSrcEl = null;  // Clear reference to prevent stale use

            renderPreviews();  // Re-render thumbnails to update UI
        }
    }

    function handleDragEnd(e) {
        this.classList.remove("dragging");
    }

    function renderPreviews() {
        thumbs.innerHTML = "";
        if (filesArray.length === 0) {
            fileCount.textContent = "No files selected";
            return;
        }
        fileCount.textContent = `${filesArray.length} image(s) selected`;

        filesArray.forEach((file, index) => {
            const reader = new FileReader();
            reader.onload = e => {
                const wrapper = document.createElement("div");
                wrapper.classList.add("thumb-wrapper");
                wrapper.setAttribute("data-index", index);

                const img = document.createElement("img");
                img.src = e.target.result;

                const removeBtn = document.createElement("div");
                removeBtn.classList.add("remove-btn");
                removeBtn.textContent = "×";
                removeBtn.title = "Remove image";
                removeBtn.addEventListener("click", () => {
                    filesArray.splice(index, 1);
                    renderPreviews();
                });

                wrapper.appendChild(img);
                wrapper.appendChild(removeBtn);
                thumbs.appendChild(wrapper);
            };
            reader.readAsDataURL(file);
        });
    }


    function addFiles(newFiles) {
        if (filesArray.length + newFiles.length > 20) {
            alert("Maximum 20 images allowed.");
            return;
        }
        filesArray = filesArray.concat(Array.from(newFiles));
        renderPreviews();
    }

    fileInput.addEventListener("change", () => {
        addFiles(fileInput.files);
        fileInput.value = "";
    });

    fileSelect.addEventListener("click", () => fileInput.click());

    ["dragenter", "dragover"].forEach(eventName => {
        dropArea.addEventListener(eventName, e => {
            e.preventDefault();
            dropArea.classList.add("hover");
        });
    });

    ["dragleave", "drop"].forEach(eventName => {
        dropArea.addEventListener(eventName, e => {
            e.preventDefault();
            dropArea.classList.remove("hover");
        });
    });

    dropArea.addEventListener("drop", e => {
        addFiles(e.dataTransfer.files);
    });

    uploadForm.addEventListener("submit", async e => {
        e.preventDefault();
        if (filesArray.length === 0) {
            alert("Please select some images first.");
            return;
        }
        spinner.classList.remove("hidden");

        const formData = new FormData();
        filesArray.forEach(file => formData.append("images", file));

        try {
            const res = await fetch("/upload", {
                method: "POST",
                body: formData
            });
            if (!res.ok) throw new Error("Upload failed");

            const blob = await res.blob();
            const url = window.URL.createObjectURL(blob);
            const a = document.createElement("a");
            a.href = url;
            a.download = "converted.pdf";
            document.body.appendChild(a);
            a.click();
            a.remove();
            window.URL.revokeObjectURL(url);

            filesArray = [];
            renderPreviews();
        } catch (err) {
            alert("Error uploading images.");
            console.error(err);
        } finally {
            spinner.classList.add("hidden");
        }
    });

    renderPreviews();
});
