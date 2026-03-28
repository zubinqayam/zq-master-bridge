fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icons/icon.ico");
        res.set("ProductName", "ZQ Master Bridge");
        res.set("FileDescription", "ZQ Control Room — local-first AI assistant");
        res.set("LegalCopyright", "Copyright © 2026 Zubin Qayam");
        res.compile().unwrap();
    }
    tauri_build::build()
}
