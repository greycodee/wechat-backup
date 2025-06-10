use crate::miui_backup::account::WXUserInfo;
use crate::common::utils;
use crate::common::model::{Message, RContact, WxFileIndex3};
use crate::common::model::{Contact,User,UserInfo};
use rusqlite::{params, Connection, OptionalExtension, Result};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::common::database::open_wechat_db;

#[derive(Debug,Clone)]
pub struct WechatDB {
    en_micro_msg_conn: Arc<Mutex<Connection>>,
    wx_file_index_conn: Arc<Mutex<Connection>>,
}

impl WechatDB {
    pub fn new(
        en_micro_msg_db_path: &Path,
        wx_file_index_db_path: &Path,
        db_private_key: &str,
    ) -> Result<Self> {
        let en_micro_msg_conn = open_wechat_db(en_micro_msg_db_path, db_private_key)?;
        let wx_file_index_conn = open_wechat_db(wx_file_index_db_path, db_private_key)?;
        Ok(WechatDB {
            en_micro_msg_conn: Arc::new(Mutex::new(en_micro_msg_conn)),
            wx_file_index_conn: Arc::new(Mutex::new(wx_file_index_conn)),
        })
    }

    pub async fn select_message_with_limit(&self, start: u32, end: u32) -> Result<Vec<Message>> {
        let en_micro_msg_conn = self.en_micro_msg_conn.lock().await;
        let mut stmt = en_micro_msg_conn
            .prepare("SELECT * FROM message limit ?,?")?;
        let messages = stmt
            .query_map((start, end), |row| {
                Ok(Message {
                    msg_id: row.get(0)?,
                    msg_svr_id: row.get(1)?,
                    msg_type: row.get(2)?,
                    status: row.get(3)?,
                    is_send: row.get(4)?,
                    is_show_timer: row.get(5)?,
                    create_time: row.get(6)?,
                    talker: row.get(7)?,
                    content: row.get(8)?,
                    img_path: row.get(9)?,
                    reserved: row.get(10)?,
                    lvbuffer: row.get(11)?,
                    trans_content: row.get(12)?,
                    trans_brand_wording: row.get(13)?,
                    talker_id: row.get(14)?,
                    biz_client_msg_id: row.get(15)?,
                    biz_chat_id: row.get(16)?,
                    biz_chat_user_id: row.get(17)?,
                    msg_seq: row.get(18)?,
                    flag: row.get(19)?,
                    solitaire_fold_info: row.get(20)?,
                    history_id: row.get(21)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(messages)
    }

    pub async fn select_r_contact_with_limit(&self,start:u32,end:u32) -> Result<Vec<RContact>>{
        let en_micro_msg_conn = self.en_micro_msg_conn.lock().await;
        let mut stmt = en_micro_msg_conn.prepare("SELECT * FROM rcontact LIMIT ?,?")?;
        let contacts = stmt.query_map((start, end), |row| {
            Ok(RContact {
                username: row.get(0)?,
                alias: row.get(1)?,
                con_remark: row.get(2)?,
                domain_list: row.get(3)?,
                nickname: row.get(4)?,
                py_initial: row.get(5)?,
                quan_pin: row.get(6)?,
                show_head: row.get(7)?,
                r#type: row.get(8)?,
                ui_type: row.get(9)?,
                weibo_flag: row.get(10)?,
                weibo_nickname: row.get(11)?,
                con_remark_py_full: row.get(12)?,
                con_remark_py_short: row.get(13)?,
                lvbuff: row.get(14)?,
                verify_flag: row.get(15)?,
                encrypt_username: row.get(16)?,
                chatroom_flag: row.get(17)?,
                delete_flag: row.get(18)?,
                contact_label_ids: row.get(19)?,
                desc_wording_id: row.get(20)?,
                open_im_appid: row.get(21)?,
                source_ext_info: row.get(22)?,
                ticket: row.get(23)?,
                username_flag: row.get(24)?,
                contact_extra: row.get(25)?,
                create_time: row.get(26)?,
            })
        })?.collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(contacts)
    }

    pub async fn select_contacts(&self) -> Result<Vec<Contact>>{
        let en_micro_msg_conn = self.en_micro_msg_conn.lock().await;
        let sql = r"
            SELECT
                username, alias,conRemark, nickname, pyInitial, quanPin, type, conRemarkPYFull, conRemarkPYShort
            FROM rcontact
         ";
        let mut stmt = en_micro_msg_conn.prepare(sql)?;
        let contacts = stmt.query_map(params![], |row| {
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

    pub async fn select_user_info_with_limit(&self,start:u32,end:u32) -> Result<Vec<UserInfo>> {
        let en_micro_msg_conn = self.en_micro_msg_conn.lock().await;
        let mut stmt = en_micro_msg_conn.prepare("SELECT * FROM userinfo LIMIT ?,?")?;
        let persons = stmt.query_map((start, end), |row| {
            Ok(UserInfo {
                id: row.get(0)?,
                w_type: row.get(1)?,
                w_value: row.get(2)?,
            })
        })?.collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(persons)
    }

    pub async fn select_user_info(&self) -> Result<User> {
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

    pub async fn select_wx_file_index_by_msg_id(&self, msg_id: i64) -> Result<Option<WxFileIndex3>> {
        let wx_file_index_conn = self.wx_file_index_conn.lock().await;
        let mut stmt = wx_file_index_conn.prepare("SELECT * FROM WxFileIndex3 WHERE msgId = ?")?;
        let wx_file_index = stmt.query_row(params![msg_id], |row| {
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
        }).optional()?;
        Ok(wx_file_index)
    }


    pub async fn get_wx_user_info(&self) -> Result<WXUserInfo> {
        let en_micro_msg_conn = self.en_micro_msg_conn.lock().await;
        let mut stmt = en_micro_msg_conn
            .prepare("SELECT id,value FROM userinfo where id in (2,4,6,42)")?;
        let persons = stmt.query_map(params![], |row| Ok((row.get(0)?, row.get(1)?)))?;

        let mut account_info = WXUserInfo {
            account_name: "".to_string(),
            account_phone: "".to_string(),
            account_avatar_path: None,
            wx_id: "".to_string(),
            wx_account_no: "".to_string(),
        };
        for p in persons {
            let (id, value): (i32, String) = p?;
            match id {
                2 => account_info.wx_id = value,
                4 => account_info.account_name = value,
                6 => account_info.account_phone = value,
                42 => account_info.wx_account_no = value,
                _ => {}
            }
        }
        account_info.account_avatar_path = Some(utils::get_avatar_path(&account_info.wx_id));
        Ok(account_info)
    }
}


#[cfg(test)]
mod test {

    use crate::common::database::save_wechat_db_to_plan;

    #[test]
    fn test_save_wechat_db_to_plan() {
        let db_path = "/Volumes/hkdisk/wechat-backup/20241218z-111111-h/workspace/wxid_jafjkmbud9l912/db/WxFileIndex.db";
        let db_private_key = "626d0bc";
        let result = save_wechat_db_to_plan(db_path, db_private_key);
        println!("{:?}", result);
    }
}
