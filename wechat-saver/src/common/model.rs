use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Message {
    pub msg_id: i64,
    pub msg_svr_id: Option<i64>,
    pub msg_type: Option<i64>,
    pub status: Option<i64>,
    pub is_send: Option<i64>,
    pub is_show_timer: Option<i64>,
    pub create_time: i64,
    pub talker: String,
    pub content: Option<String>,
    pub img_path: Option<String>,
    pub reserved: Option<String>,
    pub lvbuffer: Option<Vec<u8>>, // BLOB as Vec<u8> in Rust
    pub trans_content: Option<String>,
    pub trans_brand_wording: Option<String>,
    pub talker_id: Option<i64>,
    pub biz_client_msg_id: Option<String>,
    pub biz_chat_id: Option<i64>, // Default value can be set at struct instantiation
    pub biz_chat_user_id: Option<String>,
    pub msg_seq: Option<i64>,
    pub flag: Option<i64>,
    pub solitaire_fold_info: Option<Vec<u8>>, // BLOB as Vec<u8> in Rust
    pub history_id: Option<String>,
}



#[derive(Debug)]
pub struct RContact {
    pub username: String,                // TEXT default ''
    pub alias: String,                   // TEXT default ''
    pub con_remark: String,              // TEXT default ''
    pub domain_list: String,             // TEXT default ''
    pub nickname: String,                // TEXT default ''
    pub py_initial: String,              // TEXT default ''
    pub quan_pin: String,                // TEXT default ''
    pub show_head: i64,                  // INTEGER default '0'
    pub r#type: i64,                     // INTEGER default '0' (type is a keyword, use r#type)
    pub ui_type: i64,                    // LONG default '0'
    pub weibo_flag: i64,                 // INTEGER default '0'
    pub weibo_nickname: String,          // TEXT default ''
    pub con_remark_py_full: String,      // TEXT default ''
    pub con_remark_py_short: String,     // TEXT default ''
    pub lvbuff: Option<Vec<u8>>,         // BLOB
    pub verify_flag: i64,                // INTEGER default '0'
    pub encrypt_username: String,        // TEXT default ''
    pub chatroom_flag: Option<i64>,      // INTEGER
    pub delete_flag: i64,                // INTEGER default '0'
    pub contact_label_ids: String,       // TEXT default ''
    pub desc_wording_id: String,         // TEXT default ''
    pub open_im_appid: Option<String>,   // TEXT
    pub source_ext_info: Option<String>, // TEXT
    pub ticket: String,                  // TEXT default ''
    pub username_flag: i64,              // LONG default '0'
    pub contact_extra: Option<Vec<u8>>,  // BLOB
    pub create_time: i64,                // LONG default '0'
}

#[derive(Debug,Clone)]
pub struct WxFileIndex3 {
    pub msg_id: i64,                // LONG
    pub username: String,           // TEXT
    pub msg_type: i64,              // INTEGER
    pub msg_sub_type: i64,          // INTEGER
    pub path: String,               // TEXT
    pub size: i64,                  // LONG
    pub msg_time: i64,              // LONG
    pub hash: Option<Vec<u8>>,      // BLOB
    pub disk_space: i64,            // LONG
    pub link_uuid: Option<Vec<u8>>, // BLOB
}


// sync model

#[derive(Debug)]
pub struct UserInfo {
    pub id: i64,       // INTEGER, primary key
    pub w_type: i64,   // INT (type is a keyword, use w_type )
    pub w_value: String, // TEXT
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub wechat_id: String,
    pub account_no: String,
    pub phone: String,
    pub address: String,
    pub real_name: String,
    pub nick_name: String,
    pub avatar: String,
    pub hd_avatar: String,
}

impl User {

    pub fn default() -> Self {
        Self {
            wechat_id: "".to_string(),
            account_no: "".to_string(),
            phone: "".to_string(),
            address: "".to_string(),
            real_name: "".to_string(),
            nick_name: "".to_string(),
            avatar: "".to_string(),
            hd_avatar: "".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserList {
    total: i32,
    page: i32,
    page_size: i32,
    data: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

// contact model

#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    pub wechat_id: String,
    pub user_name: String,
    pub alias: String,
    pub con_remark: String,
    pub nick_name: String,
    pub py_initial: String,
    pub quan_pin: String,
    #[serde(rename = "type")]
    pub r#type: i64,
    pub con_remark_py_full: String,
    pub con_remark_py_short: String,
    pub avatar: String,
    pub hd_avatar: String,
}

impl Contact {
    pub fn default() -> Self {
        Self {
            wechat_id: "".to_string(),
            user_name: "".to_string(),
            alias: "".to_string(),
            con_remark: "".to_string(),
            nick_name: "".to_string(),
            py_initial: "".to_string(),
            quan_pin: "".to_string(),
            r#type: 0,
            con_remark_py_full: "".to_string(),
            con_remark_py_short: "".to_string(),
            avatar: "".to_string(),
            hd_avatar: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ContactExisted {
    pub exist: bool,
}

// upload file response model
#[derive(Serialize, Deserialize, Debug)]
pub struct UploadFileResponse {
    pub file_url: String,
    pub object_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct UploadURLFileRequest {
    pub remote_url: String,
    pub wechat_id: String,
    pub file_type_name: String,
    pub file_name: String,
    pub content_type: String,
}
// message

#[derive(Serialize, Deserialize,Debug)]
pub struct SaveMessage {
    pub id: Option<u64>,
    pub wechat_id: String,
    pub msg_id: Option<u64>,
    pub msg_svr_id: i64,
    #[serde(rename = "type")]
    pub r#type: i64,
    pub status: i64,
    pub is_send: i64,
    pub msg_create_time: i64,
    pub talker: String,
    pub content: Option<String>,
}

// file index request model

#[derive(Serialize, Deserialize, Debug)]
pub struct FileIndex {
    pub wechat_id: String,
    pub msg_id: u64,
    pub username: String,
    pub msg_type: i64,
    pub msg_sub_type: i64,
    pub path: String,
    pub size: i64,
    pub msg_time: i64,
}
impl FileIndex {

    pub fn default() -> Self {
        Self {
            wechat_id: "".to_string(),
            msg_id: 0,
            username: "".to_string(),
            msg_type: 0,
            msg_sub_type: 0,
            path: "".to_string(),
            size: 0,
            msg_time: 0,
        }
    }
}

// latest user
#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct LatestUser {
    pub wechat_id: String,
    pub account_no: String,
    pub bind_phone: String,
    pub gps: String,
    pub bind_qq: String,
    pub avatar: String,
    pub uin: String,
    pub nick_name: String,
}

impl LatestUser {
    pub fn default() -> Self {
        Self {
            wechat_id: "".to_string(),
            account_no: "".to_string(),
            bind_phone: "".to_string(),
            gps: "".to_string(),
            bind_qq: "".to_string(),
            avatar: "".to_string(),
            uin: "".to_string(),
            nick_name: "".to_string(),
        }
    }
}