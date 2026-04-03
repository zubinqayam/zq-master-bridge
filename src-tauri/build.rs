fn main() {
    // tauri-build handles VERSIONINFO, icon, and manifest automatically.
    // Do NOT add winres here — it causes duplicate VERSION resources (LNK1123).
    tauri_build::build()
}
