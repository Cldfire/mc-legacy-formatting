// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = editor_gui::EditorApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
