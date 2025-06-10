use rusqlite::Result;
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::common::model::{RContact, WxFileIndex3, Message, UserInfo};

#[derive(Debug, Clone)]
pub struct WeChatSaverDB {
    conn: Arc<Mutex<Connection>>,
}

impl WeChatSaverDB {
    /**
        @param base_path: userspace path
        @param wx_id: 微信id
    */
    pub fn new(base_path: &Path) -> Result<Self> {
        let conn = init_save_db(base_path)?;
        Ok(WeChatSaverDB { conn: Arc::new(Mutex::new(conn)) })
    }

    #[allow(dead_code)]
    pub async fn get_last_insert_row_id(&self) -> i64 {
        // lock
        let conn = self.conn.lock().await;
        conn.last_insert_rowid()
    }

    pub async fn get_latest_msg_id(&self) -> Result<i64> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("SELECT max(msgId) FROM message")?;
        let max_msg_id: i64 = stmt.query_row([], |row| row.get(0))?;
        Ok(max_msg_id)
    }

    pub async fn save_message(&self, message: &Message) -> Result<usize> {
        let conn = self.conn.lock().await;
        conn.execute(
                "INSERT INTO message (msgId,
                msgSvrId, type, status, isSend, isShowTimer, createTime, talker, content, imgPath, reserved, lvbuffer, transContent, transBrandWording, talkerId, bizClientMsgId, bizChatId, bizChatUserId, msgSeq, flag, solitaireFoldInfo, historyId
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21,?22)",
                rusqlite::params![
                    message.msg_id,
                    message.msg_svr_id,
                    message.msg_type,
                    message.status,
                    message.is_send,
                    message.is_show_timer,
                    message.create_time,
                    message.talker,
                    message.content,
                    message.img_path,
                    message.reserved,
                    message.lvbuffer,
                    message.trans_content,
                    message.trans_brand_wording,
                    message.talker_id,
                    message.biz_client_msg_id,
                    message.biz_chat_id,
                    message.biz_chat_user_id,
                    message.msg_seq,
                    message.flag,
                    message.solitaire_fold_info,
                    message.history_id,
            ],
            )
    }

    pub async fn save_r_contact(&self, contact: &RContact) -> Result<usize>{
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT INTO rcontact (
                username, alias, conRemark, domainList, nickname, pyInitial, quanPin, showHead, type, uiType, weiboFlag, weiboNickname, conRemarkPYFull, conRemarkPYShort, lvbuff, verifyFlag, encryptUsername, chatroomFlag, deleteFlag, contactLabelIds, descWordingId, openImAppid, sourceExtInfo, ticket, usernameFlag, contactExtra, createTime
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27)",
            rusqlite::params![
                contact.username,
                contact.alias,
                contact.con_remark,
                contact.domain_list,
                contact.nickname,
                contact.py_initial,
                contact.quan_pin,
                contact.show_head,
                contact.r#type,
                contact.ui_type,
                contact.weibo_flag,
                contact.weibo_nickname,
                contact.con_remark_py_full,
                contact.con_remark_py_short,
                contact.lvbuff,
                contact.verify_flag,
                contact.encrypt_username,
                contact.chatroom_flag,
                contact.delete_flag,
                contact.contact_label_ids,
                contact.desc_wording_id,
                contact.open_im_appid,
                contact.source_ext_info,
                contact.ticket,
                contact.username_flag,
                contact.contact_extra,
                contact.create_time,
            ],
        )
    }

    pub async fn save_user_info(&self, user_info: &UserInfo) -> Result<usize>{
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT INTO userinfo (
                id, type, value
            ) VALUES (?1, ?2, ?3)",
            rusqlite::params![
                user_info.id,
                user_info.w_type,
                user_info.w_value,
            ],
        )
    }

    pub async fn save_wx_file_index(&self, wx_file_index: &WxFileIndex3) -> Result<usize>{
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT INTO WxFileIndex3 (
                msgId, username, msgType, msgSubType, path, size, msgtime, hash, diskSpace, linkUUID
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![
                wx_file_index.msg_id,
                wx_file_index.username,
                wx_file_index.msg_type,
                wx_file_index.msg_sub_type,
                wx_file_index.path,
                wx_file_index.size,
                wx_file_index.msg_time,
                wx_file_index.hash,
                wx_file_index.disk_space,
                wx_file_index.link_uuid,
            ],
        )
    }

    /**
    判断消息是否可以备份
    @param msg_svr_id: Option<i64> 为None时表示msgSvrId为空
    @param talker: &str
    @param create_time: i64 消息创建时间
    @return: 返回是否可以备份 true: 可以 false: 不可以
    */
    pub async fn addition_message_flag(
        &self,
        msg_svr_id: Option<i64>,
        talker: &str,
        create_time: i64,
    ) -> Result<bool> {
        let conn = self.conn.lock().await;
        match msg_svr_id {
            None => {
                let mut stmt = conn.prepare("SELECT count(*) FROM message WHERE msgSvrId IS NULL AND talker = ? AND createTime = ?")?;
                let count: i64 = stmt.query_row(params![talker, create_time], |row| row.get(0))?;
                Ok(count == 0)
            }
            Some(id) => {
                let mut stmt = conn.prepare("SELECT count(*) FROM message WHERE msgSvrId = ? AND talker = ? AND createTime = ?")?;
                let count: i64 =
                    stmt.query_row(params![id, talker, create_time], |row| row.get(0))?;
                Ok(count == 0)
            }
        }
    }
}

fn init_save_db(dest_path: &Path) -> Result<Connection> {
    let db_path = dest_path.join("wechat.db");
    let conn = Connection::open(db_path)?;
    // create database
    conn.execute_batch(
        "
create table IF NOT EXISTS message
(
    msgId             INTEGER
        primary key autoincrement ,
    msgSvrId          INTEGER,
    type              INT,
    status            INT,
    isSend            INT,
    isShowTimer       INTEGER,
    createTime        INTEGER,
    talker            TEXT,
    content           TEXT,
    imgPath           TEXT,
    reserved          TEXT,
    lvbuffer          BLOB,
    transContent      TEXT,
    transBrandWording TEXT,
    talkerId          INTEGER,
    bizClientMsgId    TEXT,
    bizChatId         INTEGER default -1,
    bizChatUserId     TEXT,
    msgSeq            INTEGER,
    flag              INT,
    solitaireFoldInfo BLOB,
    historyId         TEXT
);

create index IF NOT EXISTS messageCreateTaklerTimeIndex
    on message (talker, createTime);

create index IF NOT EXISTS messageCreateTaklerTypeTimeIndex
    on message (talker, type, createTime);

create index IF NOT EXISTS messageCreateTimeIndex
    on message (createTime);

create index IF NOT EXISTS messageIdIndex
    on message (msgId);

create index IF NOT EXISTS messageSendCreateTimeIndex
    on message (status, isSend, createTime);

create index IF NOT EXISTS messageSvrIdIndex
    on message (msgSvrId);

create index IF NOT EXISTS messageTalkerCreateTimeIsSendIndex
    on message (talker, isSend, createTime);

create index IF NOT EXISTS messageTalkerIdTypeIndex
    on message (talkerId, type);

create index IF NOT EXISTS messageTalkerStatusIndex
    on message (talker, status);

create index IF NOT EXISTS messageTalkerSvrIdIndex
    on message (talker, msgSvrId);

create index IF NOT EXISTS messageTalkerTypeIndex
    on message (talker, type);

create index IF NOT EXISTS messagemessageTalkerFlagMsgSeqIndex
    on message (talker, flag, msgSeq);

create index IF NOT EXISTS messagemessageTalkerMsgSeqIndex
    on message (talker, msgSeq);

",
    )?;

    conn.execute_batch(
        "
create table IF NOT EXISTS userinfo
(
    id    INTEGER
        primary key,
    type  INT,
    value TEXT
);
",
    )?;

    conn.execute_batch(
        "
create table IF NOT EXISTS rcontact
(
    username         TEXT    default ''
        primary key,
    alias            TEXT    default '',
    conRemark        TEXT    default '',
    domainList       TEXT    default '',
    nickname         TEXT    default '',
    pyInitial        TEXT    default '',
    quanPin          TEXT    default '',
    showHead         INTEGER default '0',
    type             INTEGER default '0',
    uiType           LONG    default '0',
    weiboFlag        INTEGER default '0',
    weiboNickname    TEXT    default '',
    conRemarkPYFull  TEXT    default '',
    conRemarkPYShort TEXT    default '',
    lvbuff           BLOB,
    verifyFlag       INTEGER default '0',
    encryptUsername  TEXT    default '',
    chatroomFlag     INTEGER,
    deleteFlag       INTEGER default '0',
    contactLabelIds  TEXT    default '',
    descWordingId    TEXT    default '',
    openImAppid      TEXT,
    sourceExtInfo    TEXT,
    ticket           TEXT    default '',
    usernameFlag     LONG    default '0',
    contactExtra     BLOB,
    createTime       LONG    default '0'
);

create index IF NOT EXISTS contact_alias_index
    on rcontact (alias);

create unique index IF NOT EXISTS contact_username_unique_index
    on rcontact (username);

create index IF NOT EXISTS contact_usernameflag_index
    on rcontact (usernameFlag);

create index IF NOT EXISTS en_username_unique_index
    on rcontact (encryptUsername);

create index IF NOT EXISTS type_verifyFlag_index
    on rcontact (type, verifyFlag);
",
    )?;

    conn.execute_batch(
        "
create table IF NOT EXISTS WxFileIndex3
(
    msgId      LONG,
    username   TEXT,
    msgType    INTEGER,
    msgSubType INTEGER,
    path       TEXT,
    size       LONG,
    msgtime    LONG,
    hash       BLOB,
    diskSpace  LONG,
    linkUUID   BLOB
);

create index IF NOT EXISTS WxFileIndex_uuid
    on WxFileIndex3 (linkUUID);

create index IF NOT EXISTS msgid_username_index
    on WxFileIndex3 (msgId, username, msgSubType);

create index IF NOT EXISTS username_type_index
    on WxFileIndex3 (username, msgtime, msgSubType);

",
    )?;
    Ok(conn)
}


#[cfg(test)]
mod test{
    use super::*;

    #[tokio::test]
    async fn test_wechat_saver_db(){
        let base_path = Path::new("/tmp/com.tencent.mm/wxid_jafjkmbud9l912");
        let wechat_saver_db = WeChatSaverDB::new(base_path).unwrap();

        wechat_saver_db.save_user_info(&UserInfo{
            id: 99929,
            w_type: 1,
            w_value: "test".to_string(),
        }).await.unwrap();

        let last_row_id = wechat_saver_db.get_last_insert_row_id().await;
        println!("last_row_id: {}", last_row_id);
    }
}