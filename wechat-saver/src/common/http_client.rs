use reqwest::multipart;
use std::path::Path;
use serde::de::DeserializeOwned;
use serde_json::Value;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use anyhow::Result;

#[derive(Clone)]
pub struct HttpClient {
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get<T>(&self, url: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self.client.get(url).send().await?;
        let json_data = response.json::<T>().await?;
        Ok(json_data)
    }

    pub async fn post<T>(&self, url: &str, json: Value) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self.client.post(url)
            .json(&json)
            .send()
            .await?;
        let json_data = response.json::<T>().await?;
        Ok(json_data)
    }

    pub async fn upload_wx_file<T>(&self, url: &str, file_path: &Path,
                                   wechat_id: &str, file_type_name: &str)
        -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut file = File::open(file_path).await?;
        let file_name = file_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("file")
            .to_string();

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;

        let part = multipart::Part::bytes(buffer)
            .file_name(file_name)
            .mime_str("application/octet-stream")?;

        let form = multipart::Form::new()
            .part("file", part);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("wechat_id", wechat_id.parse().unwrap());
        headers.insert("file_type_name", file_type_name.parse().unwrap());

        let response = self.client
            .post(url)
            .headers(headers)
            .multipart(form)
            .send()
            .await?;

        let json_data = response.json::<T>().await?;
        Ok(json_data)
    }

    pub async fn upload_file<T>(&self, url: &str, file_path: &Path) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut file = File::open(file_path).await?;
        let file_name = file_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("file")
            .to_string();

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;

        let part = multipart::Part::bytes(buffer)
            .file_name(file_name)
            .mime_str("application/octet-stream")?;

        let form = multipart::Form::new()
            .part("file", part);

        let response = self.client
            .post(url)
            .multipart(form)
            .send()
            .await?;

        let json_data = response.json::<T>().await?;
        Ok(json_data)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::model::{User, UserList, Response, UploadFileResponse};

    #[tokio::test]
    async fn test_http_client_get() {
        let http_client = HttpClient::new();
        let res: Response<UserList> = http_client.get("http://localhost:8090/api/v1/users?page=1&page_size=10")
            .await
            .expect("Failed to get users");
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_http_client_post() {
        let json_data = serde_json::json!({
            "wechat_id": "xxx2",
            "account_no": "test11ss1123",
            "phone": "138331000",
            "address": "Test Ad22dress",
            "real_name": "Test123 User",
            "nick_name": "Test123 Nick",
            "avatar": "http://example.com/avatar.jpg",
            "data_state": "active"
        });

        let http_client = HttpClient::new();
        let res: Response<User> = http_client.post("http://localhost:8090/api/v1/users", json_data)
            .await
            .expect("Failed to create user");
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_upload_file() {
        let http_client = HttpClient::new();
        let file_path = Path::new("/Users/zheng/coding/wechatsaverv2/README.md");
        let res: Response<UploadFileResponse> = http_client.upload_file(
            "http://localhost:8090/api/v1/upload/file",
            file_path
        )
        .await
        .expect("Failed to upload file");
        println!("{:?}", res);
    }
}