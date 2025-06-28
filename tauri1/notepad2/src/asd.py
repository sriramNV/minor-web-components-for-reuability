from matplotlib import pyplot as plt
import pandas as pd

# Define the stack data
data = [
    ["🌐 Universal Dev", "Python", "PostgreSQL", "AWS", "Ubuntu (Linux)"],
    ["🏢 Enterprise Java", "Java", "Oracle / PostgreSQL", "Azure / AWS", "Red Hat Enterprise Linux"],
    ["🌍 Fullstack JS", "JavaScript (Node.js)", "MongoDB", "Vercel / AWS", "Ubuntu (Linux)"],
    ["⚙️ DevOps / Cloud Native", "Go", "PostgreSQL", "Google Cloud", "Alpine Linux"],
    ["🖥️ Microsoft Ecosystem", "C# (.NET Core)", "SQL Server", "Azure", "Windows Server / Ubuntu"],
    ["🧠 AI / Data Science", "Python", "PostgreSQL / DuckDB", "AWS / GCP", "Ubuntu (Linux)"],
    ["📱 Mobile", "Dart (Flutter)", "Firebase", "Firebase / GCP", "Ubuntu (Dev), iOS/Android (Target)"],
    ["🎮 Game Dev", "C++", "SQLite / PostgreSQL", "AWS", "Windows (Dev), Linux (Server)"],
    ["🔗 Blockchain / Web3", "Rust / Solidity", "IPFS / PostgreSQL", "Decentralized / AWS", "Ubuntu (Linux)"],
    ["🌱 IoT / Embedded", "C / MicroPython", "SQLite / Edge", "AWS IoT / Azure IoT", "Embedded Linux / RTOS"]
]

# Create a DataFrame
df = pd.DataFrame(data, columns=["Use Case", "Language", "Database", "Cloud", "OS"])

# Plot the table using matplotlib
fig, ax = plt.subplots(figsize=(14, 6))
ax.axis('off')
tbl = ax.table(cellText=df.values, colLabels=df.columns, cellLoc='center', loc='center')
tbl.auto_set_font_size(False)
tbl.set_fontsize(10)
tbl.scale(1.2, 2.5)

# Save the table as an image
image_path = "tech_stack_summary.png"
pdf_path = "tech_stack_summary.pdf"
plt.savefig(image_path, bbox_inches='tight', dpi=300)
plt.savefig(pdf_path, bbox_inches='tight', dpi=300)
plt.close()

image_path, pdf_path
