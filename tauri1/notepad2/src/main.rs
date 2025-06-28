mod notepad_app;
mod file_buffer;
mod editor;
mod ui;
mod theme;


use eframe::NativeOptions;
use notepad_app::NotepadApp;

fn main() -> Result<(), eframe::Error> {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Rust Notepad",
        native_options,
        Box::new(|_cc| Ok(Box::new(NotepadApp::default()))),
    )
}
