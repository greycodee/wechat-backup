use std::path::{Path, PathBuf};
use quick_xml::de::from_str;
use serde::Deserialize;
use anyhow::Result;
use crate::common::model::LatestUser;

#[derive(Debug, Deserialize)]
struct StringEntry {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@value", default)]
    value: Option<String>,
    #[serde(rename = "$text", default)]
    text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BooleanEntry {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@value", default)]
    value: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct LongEntry {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@value", default)]
    value: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct IntEntry {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@value", default)]
    value: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct FloatEntry {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@value", default)]
    value: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "map")]
struct Map {
    #[serde(rename = "$value", default)]
    entries: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Entry {
    Boolean(BooleanEntry),
    String(StringEntry),
    Long(LongEntry),
    Int(IntEntry),
    Float(FloatEntry),
}

pub trait PreferencesParser {
    fn get_preferences_path(base_path: &Path) -> PathBuf;
}

pub fn parse_mm_preferences<T: PreferencesParser>(base_path: &Path) -> Result<LatestUser> {
    let xml_file_path = T::get_preferences_path(base_path);
    let content = std::fs::read_to_string(xml_file_path)?;
    parse_latest_user(&content)
}

fn parse_latest_user(content: &str) -> Result<LatestUser> {
    let map: Map = from_str(content)?;

    let mut user = LatestUser {
        wechat_id: String::new(),
        account_no: String::new(),
        bind_phone: String::new(),
        gps: String::new(),
        bind_qq: String::new(),
        avatar: String::new(),
        uin: String::new(),
        nick_name: String::new(),
    };

    for entry in &map.entries {
        if let Entry::String(string_entry) = entry {
            let value = string_entry.value.as_ref()
                .map(|s| s.to_string())
                .or_else(|| string_entry.text.as_ref().map(|s| s.to_string()))
                .unwrap_or_default();
                
            if value.is_empty() {
                continue;
            }

            match string_entry.name.as_str() {
                "login_weixin_username" => user.wechat_id = value,
                "last_login_alias" => user.account_no = value,
                "last_login_bind_mobile" => user.bind_phone = value,
                "__T_Last_Gps_Cell__" => user.gps = value,
                "last_login_bind_qq" => user.bind_qq = value,
                "last_avatar_path" => user.avatar = value,
                "last_login_uin" => user.uin = value,
                "last_login_nick_name" => user.nick_name = value,
                _ => {}
            }
        }
    }

    Ok(user)
} 