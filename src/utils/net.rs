use crate::utils::types::Result;
use curl::easy::{Easy, Form};
use regex::Regex;

struct CurlPostRequestSender {
    easy_curl: Easy,
}

impl CurlPostRequestSender {
    const STATUS_OK: u32 = 200;

    pub fn new(url: String) -> Result<Self> {
        let mut easy_curl = Easy::new();
        easy_curl.url(url.as_str())?;
        easy_curl.post(true)?;
        Ok(Self { easy_curl })
    }

    pub fn add_file(&mut self, path_to_file: &str) -> Result<&mut Self> {
        let mut file_form = Form::new();
        file_form.part("file").file(path_to_file).add()?;
        self.add_form_data(file_form)
    }

    pub fn add_post_fields(&mut self, post_fields: &str) -> Result<&mut Self> {
        self.easy_curl.post_field_size(post_fields.len() as u64)?;
        self.easy_curl.post_fields_copy(post_fields.as_bytes())?;
        Ok(self)
    }

    pub fn add_form_data(&mut self, form_data: Form) -> Result<&mut Self> {
        self.easy_curl.httppost(form_data)?;
        Ok(self)
    }

    pub fn send_request(&mut self) -> Result<Vec<u8>> {
        let mut response_data = Vec::new();
        {
            let mut transfer = self.easy_curl.transfer();
            transfer.write_function(|new_data| {
                response_data.extend_from_slice(new_data);
                Ok(new_data.len())
            })?;
            transfer.perform()?;
        }
        self.check_request_result()?;
        Ok(response_data)
    }

    fn check_request_result(&mut self) -> Result<()> {
        let resp_code = self.easy_curl.response_code().unwrap();
        if resp_code != CurlPostRequestSender::STATUS_OK {
            return Err(format!("Curl: Error! Response code: {}", resp_code).into());
        }
        Ok(())
    }
}

pub struct AnonFilesUploader {
    path_to_file: String,
}

impl AnonFilesUploader {
    const URL: &'static str = "https://api.filechan.org/upload"; // "https://anonfiles.com/api/upload";
    const REGEX_PATTERN: &'static str = r#"("short":")(\S+)("},"metadata")"#;

    pub const fn new(path_to_file: String) -> Self {
        Self { path_to_file }
    }

    pub fn upload(&self) -> Result<String> {
        let response = CurlPostRequestSender::new(AnonFilesUploader::URL.to_string())?
            .add_file(&self.path_to_file)?
            .send_request()?;
        AnonFilesUploader::get_file_short_link_from_response(&response)
    }

    fn get_file_short_link_from_response(response_data: &[u8]) -> Result<String> {
        let str_resp = std::str::from_utf8(response_data)?;
        let regex = Regex::new(AnonFilesUploader::REGEX_PATTERN).unwrap();
        let captures_short_link_index = 2;

        if let Some(captures) = regex.captures(str_resp) {
            if let Some(capture) = captures.get(captures_short_link_index) {
                return Ok(capture.as_str().to_string());
            }
        }
        Err("AnonFilesUploader: Cannot get short link from response".into())
    }
}

pub struct TelegramBotSender {
    token: String,
    chat_id: String,
}

impl TelegramBotSender {
    #[must_use]
    pub const fn new(token: String, chat_id: String) -> Self {
        Self { token, chat_id }
    }

    pub fn send_message(&self, message: &str) -> Result<()> {
        let request_url = format!(r#"https://api.telegram.org/bot{}/sendMessage"#, self.token);
        let request_fileds = self.build_request_fileds(message);
        println!("{}/{}", request_url, request_fileds);

        CurlPostRequestSender::new(request_url)?
            .add_post_fields(request_fileds.as_str())?
            .send_request()?;
        Ok(())
    }

    fn build_request_fileds(&self, message: &str) -> String {
        format!(
            "chat_id={}&text={}&parse_mode=Markdown&disable_web_page_preview=True",
            self.chat_id, message
        )
    }
}
