// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "mc-legacy-formatting editor",
        native_options,
        Box::new(|cc| Box::new(editor_gui::EditorApp::new(cc))),
    );
}
