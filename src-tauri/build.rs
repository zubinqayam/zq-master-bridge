// Tauri v2 handles all Windows resource embedding (VERSIONINFO, icon,
// manifest) automatically via tauri-build. Do NOT add winres or any
// other resource compiler here - it causes duplicate VERSIONINFO and
// a fatal LNK1123 linker error.
fn main() {
    tauri_build::build()
}
