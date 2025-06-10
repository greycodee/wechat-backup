use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use anyhow::Result;
use rusqlite::{params, Connection};
use tokio::sync::Mutex;

use crate::common::database::{open_wechat_db, save_wechat_db_to_plan};
use crate::common::model::{Contact, LatestUser, SaveMessage, User, UserInfo, WxFileIndex3};
use crate::common::file_path_base::{FilePath, FilePathInitializer};
use crate::common::utils;

#[derive(Debug, Clone)]
pub struct AccountInfoBase {
    pub latest_user: LatestUser,
    pub file_path: FilePath,
    db_secret_key: String,
    en_micro_msg_conn: Arc<Mutex<Connection>>,
    wx_file_index_conn: Arc<Mutex<Connection>>,
}

pub trait AccountInitializer: Default {
    fn parse_mm_preferences(base_path: &Path) -> Result<LatestUser>;
}

impl AccountInfoBase {
    pub fn new<T: AccountInitializer>(base_path: &Path, file_path_initializer: &impl FilePathInitializer) -> Result<Self> {
        let last_user = T::parse_mm_preferences(base_path)?;
        let mut file_path = FilePath::new();
        file_path.init_with_initializer(file_path_initializer, base_path, &last_user.uin);
        let db_secret_key = utils::gen_db_private_key(&last_user.uin);
        let en_micro_msg_conn = open_wechat_db(&file_path.en_micro_msg_db_path, &db_secret_key)?;
        let wx_file_index_conn = open_wechat_db(&file_path.wx_file_index_db_path, &db_secret_key)?;
        
        Ok(AccountInfoBase {
            latest_user: last_user,
            file_path,
            db_secret_key,
            en_micro_msg_conn: Arc::new(Mutex::new(en_micro_msg_conn)),
            wx_file_index_conn: Arc::new(Mutex::new(wx_file_index_conn)),
        })
    }

    pub async fn get_user_info(&self) -> Result<User> {
        let en_micro_msg_conn = self.en_micro_msg_conn.lock().await;
        let sql = "SELECT * FROM userinfo where id in (2,4,6,42,12292,12293,294913)";
        let mut stmt = en_micro_msg_conn.prepare(sql)?;
        let persons = stmt.query_map(params![], |row| {
            Ok(UserInfo {
                id: row.get(0)?,
                w_type: row.get(1)?,
                w_value: row.get(2)?,
            })
        })?.collect::<std::result::Result<Vec<_>, _>>()?;

        let mut user = User::default();
        let mut address = "".to_string();
        for p in persons {
            match p.id {
                2 => user.wechat_id = p.w_value,
                4 => user.nick_name = p.w_value,
                6 => user.phone = p.w_value,
                42 => user.account_no = p.w_value,
                12292 | 12293 => address = format!("{}-{}", address, p.w_value),
                294913 => user.real_name = p.w_value,
                _ => {}
            }
        }
        user.address = address;
        Ok(user)
    }

    pub async fn select_message_with_limit(&self, start: u32, end: u32) -> Result<Vec<SaveMessage>> {
        let en_micro_msg_conn = self.en_micro_msg_conn.lock().await;
        let sql = r"
            SELECT
                msgId, msgSvrId, type, status, isSend, createTime, talker, content
            from message
            where msgSvrId is not null
            and status is not null
            limit ?,?
        ";
        let mut stmt = en_micro_msg_conn
            .prepare(sql)?;
        let messages = stmt
            .query_map(params![start, end], |row| {
                Ok(SaveMessage {
                    id: None,
                    wechat_id: "".to_string(),
                    msg_id: row.get(0)?,
                    msg_svr_id: row.get(1)?,
                    r#type: row.get(2)?,
                    status: row.get(3)?,
                    is_send: row.get(4)?,
                    msg_create_time: row.get(5)?,
                    talker: row.get(6)?,
                    content: row.get(7)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>, _>>()?;
        Ok(messages)
    }
    
    pub async fn select_message_count(&self) -> Result<u32> {
        let en_micro_msg_conn = self.en_micro_msg_conn.lock().await;
        let sql = r"
            SELECT count(*) from message
            where msgSvrId is not null
            and status is not null
        ";
        let mut stmt = en_micro_msg_conn.prepare(sql)?;
        let count = stmt.query_row(params![], |row| {
            row.get(0)
        })?;
        Ok(count)
    }

    pub async fn select_contacts(&self, start: u32, end: u32) -> Result<Vec<Contact>> {
        let en_micro_msg_conn = self.en_micro_msg_conn.lock().await;
        let sql = r"
            SELECT
                username, alias,conRemark, nickname, pyInitial, quanPin, type, conRemarkPYFull, conRemarkPYShort
            FROM rcontact limit ?,?
         ";
        let mut stmt = en_micro_msg_conn.prepare(sql)?;
        let contacts = stmt.query_map(params![start,end], |row| {
            Ok(Contact {
                wechat_id: "".to_string(),
                user_name: row.get(0)?,
                alias: row.get(1)?,
                con_remark: row.get(2)?,
                nick_name: row.get(3)?,
                py_initial: row.get(4)?,
                quan_pin: row.get(5)?,
                r#type: row.get(6)?,
                con_remark_py_full: row.get(7)?,
                con_remark_py_short: row.get(8)?,
                avatar: "".to_string(),
                hd_avatar: "".to_string(),
            })
        })?.collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(contacts)
    }

    pub async fn select_wx_file_index_by_msg_id(&self, msg_id: u64) -> rusqlite::Result<Vec<WxFileIndex3>> {
        let wx_file_index_conn = self.wx_file_index_conn.lock().await;
        let mut stmt = wx_file_index_conn.prepare("SELECT * FROM WxFileIndex3 WHERE msgId = ?")?;
        let wx_file_index = stmt.query_map(params![msg_id], |row| {
            Ok(WxFileIndex3 {
                msg_id: row.get(0)?,
                username: row.get(1)?,
                msg_type: row.get(2)?,
                msg_sub_type: row.get(3)?,
                path: row.get(4)?,
                size: row.get(5)?,
                msg_time: row.get(6)?,
                hash: row.get(7).ok(),
                disk_space: row.get(8)?,
                link_uuid: row.get(9).ok(),
            })
        })?.collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(wx_file_index)
    }

    pub async fn select_contact_avatar(&self, user_name: &str) -> Result<HashMap<String, String>> {
        let en_micro_msg_conn = self.en_micro_msg_conn.lock().await;
        let sql = r"
            SELECT reserved1, reserved2
            FROM img_flag where username = ?
        ";
        let mut stmt = en_micro_msg_conn.prepare(sql)?;
        let contact_avatar_map = stmt.query_row(params![user_name], |row| {
            let avatar = row.get(1)?;
            let hd_avatar = row.get(0)?;
            let mut map = HashMap::new();
            map.insert("avatar".to_string(), avatar);
            map.insert("hd_avatar".to_string(), hd_avatar);
            Ok(map)
        })?;
        Ok(contact_avatar_map)
    }


    pub fn save_en_micro_msg_db(&self) -> Result<String> {
        let plan_path = save_wechat_db_to_plan(self.file_path.en_micro_msg_db_path.to_str().unwrap(), &self.db_secret_key)?;
        Ok(plan_path)
    }

    pub fn save_wx_file_index_db(&self) -> Result<String> {
        let plan_path = save_wechat_db_to_plan(self.file_path.wx_file_index_db_path.to_str().unwrap(), &self.db_secret_key)?;
        Ok(plan_path)
    }
} 