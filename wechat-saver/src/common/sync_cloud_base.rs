use std::collections::HashMap;
use std::path::{Path, PathBuf};
use anyhow::{anyhow, Result};
use crate::common::utils;
use crate::common::http_client::HttpClient;
use crate::common::model::{Contact, ContactExisted, FileIndex, Response, SaveMessage, UploadFileResponse, UploadURLFileRequest, User};
use crate::common::msg_type::{is_file, MsgType};
use crate::common::voice_decode::wechat_voice_decode;
use crate::common::account_base::AccountInfoBase;

#[derive(Clone)]
pub struct SyncCloudBase {
    pub account_info: AccountInfoBase,
    pub endpoint: String,
    pub http_client: HttpClient,
}

impl SyncCloudBase {
    pub fn new(account_info: AccountInfoBase, endpoint: &str) -> Self {
        Self {
            account_info,
            endpoint: endpoint.to_string(),
            http_client: HttpClient::new()
        }
    }

    pub async fn sync_all(&self) {
        let this = self.clone();
        let sync_user_handler = tokio::spawn(async move { this.sync_user_info().await });
        let this = self.clone();
        let sync_contact_handler = tokio::spawn(async move { this.sync_contact().await });
        let this = self.clone();
        let sync_message_handler = tokio::spawn(async move { this.sync_message().await });
        
        if let Err(err) = tokio::try_join!(sync_user_handler, sync_contact_handler, sync_message_handler) {
            println!("Error during sync_all: {:?}", err);
        }
    }

    async fn upload_avatar(&self, wx_id: &str) -> HashMap<&str, String> {
        let mut result_map = HashMap::new();
        let endpoint = format!("{}/api/v1/upload/wx_file", self.endpoint);
        let upload_remote_file_endpoint = format!("{}/api/v1/upload/remote", self.endpoint);
        let avatar_path_str = format!("{}/{}",
                                     self.account_info.file_path.avatar_path.as_path().to_str().unwrap(),
                                     utils::get_avatar_path(wx_id).as_path().to_str().unwrap()
        );
        let hd_avatar_path_str = format!("{}/{}",
                                        self.account_info.file_path.avatar_path.as_path().to_str().unwrap(),
                                        utils::get_hd_avatar_path(wx_id).as_path().to_str().unwrap()
        );
        let mut remote_avatar_map = HashMap::new();
        
        match self.account_info.select_contact_avatar(wx_id).await {
            Ok(map) => {
                remote_avatar_map = map;
                println!("remote_avatar_map: {:?}", remote_avatar_map);
            }
            Err(err) => {
                println!("select contact avatar failed: {:?}", err);
            }
        }

        let avatar_path = Path::new(&avatar_path_str);
        let hd_avatar_path = Path::new(&hd_avatar_path_str);
        let mut avatar_path_map = HashMap::new();
        avatar_path_map.insert("avatar", avatar_path);
        avatar_path_map.insert("hd_avatar", hd_avatar_path);

        for (key, path) in avatar_path_map {
            if path.exists() {
                match self.http_client
                    .upload_wx_file::<Response<UploadFileResponse>>(&endpoint, path, &self.account_info.latest_user.wechat_id, "avatar").await {
                    Ok(upload_file_response) => {
                        if upload_file_response.code == 200 {
                            if let Some(resp_data) = upload_file_response.data {
                                let object_name = resp_data.object_name.clone();
                                result_map.insert(key, object_name);
                            }
                        }
                    }
                    Err(err) => {
                        println!("upload avatar failed: {:?}", err);
                    }
                }
            } else if remote_avatar_map.contains_key(key) {
                if remote_avatar_map.get(key).unwrap().is_empty() {
                    continue;
                }
                let file_name = match key {
                    "avatar" => utils::get_avatar_file_name(wx_id),
                    "hd_avatar" => utils::get_hd_avatar_file_name(wx_id),
                    _ => String::new(),
                };
                let upload_url_file_request = UploadURLFileRequest {
                    remote_url: remote_avatar_map.get(key).unwrap().to_string(),
                    wechat_id: self.account_info.latest_user.wechat_id.clone(),
                    file_type_name: "avatar".to_string(),
                    file_name,
                    content_type: "image/jpeg".to_string(),
                };
                
                match serde_json::to_value(upload_url_file_request) {
                    Ok(body) => {
                        match self.http_client.post::<Response<UploadFileResponse>>(&upload_remote_file_endpoint, body).await {
                            Ok(upload_file_response) => {
                                if upload_file_response.code == 200 {
                                    if let Some(resp_data) = upload_file_response.data {
                                        let object_name = resp_data.object_name.clone();
                                        result_map.insert(key, object_name);
                                    }
                                }
                            }
                            Err(err) => {
                                println!("upload avatar failed: {:?}", err);
                            }
                        }
                    }
                    Err(err) => {
                        println!("Failed to serialize upload request: {:?}", err);
                    }
                }
            }
        }
        result_map
    }

    pub async fn sync_user_info(&self) {
        match self.account_info.get_user_info().await {
            Ok(mut user_info) => {
                let endpoint = format!("{}/api/v1/users", self.endpoint);
                let avatar_map = self.upload_avatar(&user_info.wechat_id).await;
                
                if avatar_map.contains_key("avatar") {
                    user_info.avatar = avatar_map.get("avatar").unwrap().to_string();
                }
                if avatar_map.contains_key("hd_avatar") {
                    user_info.hd_avatar = avatar_map.get("hd_avatar").unwrap().to_string();
                }

                match serde_json::to_value(user_info) {
                    Ok(body) => {
                        if let Err(err) = self.http_client.post::<Response<User>>(&endpoint, body).await {
                            println!("Failed to sync user info: {:?}", err);
                        }
                    }
                    Err(err) => {
                        println!("Failed to serialize user info: {:?}", err);
                    }
                }
            }
            Err(err) => {
                println!("Failed to get user info: {:?}", err);
            }
        }
    }

    pub async fn sync_contact(&self) {
        let endpoint = format!("{}/api/v1/contacts", self.endpoint);
        let mut offset = 0;
        let limit = 100;

        loop {
            match self.account_info.select_contacts(offset, limit).await {
                Ok(contacts) => {
                    if contacts.is_empty() {
                        break;
                    }
                    
                    for mut contact in contacts {
                        let exist_endpoint = format!("{}/api/v1/contacts/exist?wechat_id={}&user_name={}",
                                                     self.endpoint,
                                                     self.account_info.latest_user.wechat_id,
                                                     contact.user_name);
                                                     
                        match self.http_client.get::<Response<ContactExisted>>(&exist_endpoint).await {
                            Ok(exist) => {
                                if exist.code == 200 {
                                    if let Some(data) = exist.data {
                                        if data.exist {
                                            continue;
                                        }
                                    }
                                }
                            }
                            Err(err) => {
                                println!("Failed to check contact existence: {:?}", err);
                            }
                        }

                        let avatar_map = self.upload_avatar(&contact.user_name).await;
                        if avatar_map.contains_key("avatar") {
                            contact.avatar = avatar_map.get("avatar").unwrap().to_string();
                        }
                        if avatar_map.contains_key("hd_avatar") {
                            contact.hd_avatar = avatar_map.get("hd_avatar").unwrap().to_string();
                        }
                        
                        contact.wechat_id = String::from(&self.account_info.latest_user.wechat_id);
                        match serde_json::to_value(contact) {
                            Ok(body) => {
                                if let Err(err) = self.http_client.post::<Response<Contact>>(&endpoint, body).await {
                                    println!("Failed to sync contact: {:?}", err);
                                }
                            }
                            Err(err) => {
                                println!("Failed to serialize contact: {:?}", err);
                            }
                        }
                    }
                    offset += limit;
                }
                Err(err) => {
                    println!("select contacts failed: {:?}", err);
                    break;
                }
            }
        }
    }

    pub async fn sync_message(&self) {
        let message_count = match self.account_info.select_message_count().await {
            Ok(count) => count,
            Err(err) => {
                println!("select message count failed: {:?}", err);
                return;
            }
        };
        println!("message count: {}", message_count);
        let endpoint = format!("{}/api/v1/messages", self.endpoint);
        let mut offset = 0;
        let limit = 100;

        loop {
            match self.account_info.select_message_with_limit(offset, limit).await {
                Ok(messages) => {
                    if messages.is_empty() {
                        println!("no more messages,offset:{}", offset);
                        break;
                    }

                    for mut msg in messages {
                        let old_msg_id = msg.msg_id.unwrap();
                        msg.wechat_id = String::from(&self.account_info.latest_user.wechat_id);
                        
                        match serde_json::to_value(msg) {
                            Ok(body) => {
                                match self.http_client.post::<Response<SaveMessage>>(&endpoint, body).await {
                                    Ok(resp) => {
                                        if resp.code == 200 {
                                            if let Some(resp_data) = resp.data {
                                                if resp_data.id == Some(0) {
                                                    println!("message already exists: {:?}", resp_data.msg_svr_id);
                                                    continue;
                                                }
                                                if !is_file(resp_data.r#type) {
                                                    continue;
                                                }
                                                let new_msg_id = resp_data.id.unwrap();
                                                self.sync_file(old_msg_id, new_msg_id).await;
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        println!("sync message failed: {:?}", err);
                                    }
                                }
                            }
                            Err(err) => {
                                println!("Failed to serialize message: {:?}", err);
                            }
                        }
                    }
                    offset += limit;
                }
                Err(err) => {
                    println!("select message failed: {:?}", err);
                    break;
                }
            }
        }
    }

    async fn sync_file(&self, old_msg_id: u64, new_msg_id: u64) {
        match self.find_file_index(old_msg_id).await {
            Ok(file_index_vec) => {
                for mut file_index in file_index_vec {
                    file_index.msg_id = new_msg_id;
                    file_index.wechat_id = self.account_info.latest_user.wechat_id.clone();
                    if let Err(err) = self.upload_file(file_index).await {
                        println!("Failed to upload file: {:?}", err);
                    }
                }
            }
            Err(err) => {
                println!("Failed to find file index: {:?}", err);
            }
        }
    }

    async fn upload_file(&self, mut file_index: FileIndex) -> Result<()> {
        let mut local_file_path = Path::new(&file_index.path);
        if !local_file_path.exists() {
            println!("file path not exists: {:?}", local_file_path);
            return Err(anyhow!("file not exists"));
        }

        let mp3_file_path;
        if let Some(MsgType::Voice) = MsgType::from(file_index.msg_type) {
            match self.voice_decode(&local_file_path.to_str().unwrap()) {
                Ok(path) => {
                    mp3_file_path = path;
                    local_file_path = mp3_file_path.as_path();
                }
                Err(err) => {
                    println!("Failed to decode voice file: {:?}", err);
                    return Err(err);
                }
            }
        }

        let endpoint = format!("{}/api/v1/upload/wx_file", self.endpoint);
        let file_type_name = match MsgType::from_to_string(file_index.msg_type) {
            Some(name) => name,
            None => {
                println!("Invalid message type");
                return Err(anyhow!("Invalid message type"));
            }
        };

        match self.http_client
            .upload_wx_file::<Response<UploadFileResponse>>(&endpoint, local_file_path, &self.account_info.latest_user.wechat_id, &file_type_name)
            .await {
            Ok(upload_file_response) => {
                println!("{:?}", upload_file_response);
                if upload_file_response.code == 200 {
                    if let Some(resp_data) = upload_file_response.data {
                        file_index.path = resp_data.object_name;
                    }
                }
            }
            Err(err) => {
                println!("Failed to upload file: {:?}", err);
                return Err(err);
            }
        }

        let endpoint = format!("{}/api/v1/files", self.endpoint);
        match serde_json::to_value(file_index) {
            Ok(body) => {
                println!("{:?}", body);
                match self.http_client.post::<Response<FileIndex>>(&endpoint, body).await {
                    Ok(resp) => {
                        println!("{:?}", resp);
                        Ok(())
                    }
                    Err(err) => {
                        println!("Failed to save file index: {:?}", err);
                        Err(err.into())
                    }
                }
            }
            Err(err) => {
                println!("Failed to serialize file index: {:?}", err);
                Err(err.into())
            }
        }
    }

    async fn find_file_index(&self, msg_id: u64) -> Result<Vec<FileIndex>> {
        match self.account_info.select_wx_file_index_by_msg_id(msg_id).await {
            Ok(wx_file_index) => {
                let file_index_vec = wx_file_index.iter().map(|wx_file| {
                    println!("{:?}", wx_file);
                    let mut file_index = FileIndex::default();
                    match self.parse_file_path(&wx_file.path) {
                        Ok(path) => {
                            file_index.username = wx_file.username.clone();
                            file_index.msg_type = wx_file.msg_type;
                            file_index.msg_sub_type = wx_file.msg_sub_type;
                            file_index.path = path;
                            file_index.size = wx_file.size;
                            file_index.msg_time = wx_file.msg_time;
                        }
                        Err(err) => {
                            println!("Failed to parse file path: {:?}", err);
                        }
                    }
                    file_index
                }).collect();
                Ok(file_index_vec)
            }
            Err(err) => {
                println!("Failed to select wx file index: {:?}", err);
                Ok(Vec::new())
            }
        }
    }

    fn parse_file_path(&self, file_path: &str) -> Result<String> {
        match utils::get_first_value_after_double_slash(file_path) {
            None => {
                println!("Invalid file path format");
                Err(anyhow!("file path is invalid"))
            }
            Some(dir_name) => {
                let base_path = match dir_name {
                    "Download" => &self.account_info.file_path.download_path,
                    "image2" => &self.account_info.file_path.image_path,
                    "voice2" => &self.account_info.file_path.voice_path,
                    "video" => &self.account_info.file_path.video_path,
                    "openapi" => &self.account_info.file_path.openapi_path,
                    "attachment" => &self.account_info.file_path.attachment_path,
                    _ => {
                        println!("Unknown directory type: {}", dir_name);
                        return Err(anyhow!("file path is invalid"));
                    }
                };
                match utils::get_last_full_path(file_path) {
                    Some(last_path) => {
                        let file_full_path = base_path.join(last_path).to_str().unwrap().to_string();
                        Ok(file_full_path)
                    }
                    None => {
                        println!("Failed to get last full path");
                        Err(anyhow!("file path is invalid"))
                    }
                }
            }
        }
    }

    fn voice_decode(&self, file_path: &str) -> Result<PathBuf> {
        let arm_file_path = self.account_info.file_path.voice_path.join(file_path);
        match wechat_voice_decode(&arm_file_path) {
            Ok(mp3_file_path) => Ok(mp3_file_path),
            Err(err) => {
                println!("Failed to decode voice file: {:?}", err);
                Err(err.into())
            }
        }
    }
}