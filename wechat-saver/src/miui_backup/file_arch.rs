use crate::miui_backup::account::AccountInfo;
use crate::miui_backup::databases::wechat_saver_db::WeChatSaverDB;
use std::fs;
use std::io::{Error, Result};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use rusqlite::Error::InvalidColumnType;
use tokio::sync::Mutex;
use crate::miui_backup::databases::wechat_db::WechatDB;
use crate::common::utils::change_file_extension;
use crate::common::voice_decode::{convert_all_voice_to_mp3, wechat_voice_decode};
use crate::miui_backup::wx_file_index::{get_after_double_slash, get_file_dir_name, get_file_name, FileDirName};

#[derive(Debug)]
pub struct FileArch {
    account_info: AccountInfo,
    dest_path: PathBuf,
    wechat_saver_db: WeChatSaverDB,
}

impl FileArch {
    /**
        @param base_path: workspace
    */
    pub fn new(base_path: &Path, account_info:  AccountInfo) -> Result<Self> {
        let user_space_path = base_path.join(&account_info.wx_user_info.wx_id);
        if !user_space_path.exists() {
            fs::create_dir_all(&user_space_path)?;
        }
        if let Ok(wechat_saver_db) = WeChatSaverDB::new(&user_space_path) {
            Ok(FileArch {
                account_info,
                dest_path: user_space_path,
                wechat_saver_db,
            })
        } else {
            Err(Error::new(
                std::io::ErrorKind::Other,
                "create wechat saver db error",
            ))
        }
    }
    pub async fn arch_all(&mut self) -> Result<()> {
        self.arch_db().await?;
        self.arch_voice()?;
        self.arch_image()?;
        self.arch_avatar()?;
        self.arch_video()?;
        self.arch_openapi()?;
        self.arch_attachment()?;
        Ok(())
    }

    async fn arch_db(&mut self) -> Result<()> {
        // TODO 考虑增量备份的情况
        let db_path = &self.dest_path.join("db");
        if !db_path.exists() {
            fs::create_dir_all(db_path)?;
        }
        let en_micro_msg_db_path = Path::new(&self.account_info.en_micro_msg_db_path);
        let en_micro_msg_db_dst_path = db_path.join(en_micro_msg_db_path.file_name().unwrap());
        fs::copy(en_micro_msg_db_path, en_micro_msg_db_dst_path)?;
        let wx_file_index_db_path = Path::new(&self.account_info.wx_file_index_db_path);
        let wx_file_index_db_dst_path = db_path.join(wx_file_index_db_path.file_name().unwrap());
        fs::copy(wx_file_index_db_path, wx_file_index_db_dst_path)?;

        self.arch_db_message_table().await?;
        self.arch_db_r_contact_table().await?;
        self.arch_db_user_info_table().await?;

        Ok(())
    }

    async fn arch_db_message_table(&self) -> Result<()> {
        // 获取当前最新的 msg_id
        let latest_msg_id = match self.wechat_saver_db.get_latest_msg_id().await {
            Ok(mut id) => {
                id += 1;
                Arc::new(Mutex::new(id))
            },
            Err(e) => {
                match e {
                    InvalidColumnType(_, _, ..) => {
                        // init insert
                        Arc::new(Mutex::new(1))
                    },
                    _ => {
                        return Err(Error::new(std::io::ErrorKind::Other, format!("get latest msg id error: {:?}", e)));
                    }
                }
            },
        };

        let mut offset = 0;
        let limit = 1000;
        let mut task_vec = Vec::new();
        loop {
            let message_list = self
                .account_info
                .db_conn
                .select_message_with_limit(offset, limit);
            match message_list.await {
                Ok(list) => {
                    if list.is_empty() {
                        break;
                    }
                    let db_conn = self.account_info.db_conn.clone();
                    let wechat_saver_db = self.wechat_saver_db.clone();
                    let latest_msg_id = Arc::clone(&latest_msg_id); // Clone the Arc here
                    let download_path = self.account_info.download_path.clone();
                    let dest_path = self.dest_path.clone();
                    let task = tokio::spawn(async move {
                        for mut message in list {
                            if let Ok(true) = wechat_saver_db.addition_message_flag(
                                message.msg_svr_id,
                                &message.talker,
                                message.create_time,
                            ).await {
                                let mut msg_id = latest_msg_id.lock().await;
                                let old_msg_id = message.msg_id;
                                message.msg_id = *msg_id;

                                match wechat_saver_db.save_message(&message).await {
                                    Ok(_) => {
                                        *msg_id += 1;
                                        // 重建 WXFileIndex3 的索引
                                        match arch_db_wx_file_index_by_msg_id(
                                            old_msg_id,
                                            message.msg_id,
                                            &download_path,
                                            &dest_path,
                                            &db_conn,
                                            &wechat_saver_db
                                        ).await{
                                            Ok(_) => {},
                                            Err(e) => {
                                                println!("arch db wx file index by msg id error: {:?}", e);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        println!("save message error: {:?}", e);
                                    }
                                }
                            }
                        }
                    });
                    task_vec.push(task);
                }
                Err(_e) => {
                    break;
                }
            }
            offset += limit;
        }
        for task in task_vec {
            task.await?;
        }
        Ok(())
    }

    async fn arch_db_r_contact_table(&self) -> Result<()> {
        // TODO 考虑返回冲突的联系人，选择是否更新
        // TODO 标记删除的联系人
        let mut offset = 0;
        let limit = 500;
        loop {
            let contact_list = self.account_info.db_conn.select_r_contact_with_limit(offset, limit);
            match contact_list.await {
                Ok(list) => {
                    if list.is_empty() {
                        break;
                    }
                    for contact in list {
                        if let Err(e) = self.wechat_saver_db.save_r_contact(&contact).await {
                            println!("save r contact error: {:?}", e);
                        }
                    }
                }
                Err(_e) => {
                    break;
                }
            }
            offset += limit;
        }
        Ok(())
    }

    async fn arch_db_user_info_table(&self) -> Result<()> {
        let mut offset = 0;
        let limit = 500;
        loop {
            let user_info_list = self.account_info.db_conn.select_user_info_with_limit(offset, limit);
            match user_info_list.await {
                Ok(list) => {
                    if list.is_empty() {
                        break;
                    }
                    for user_info in list {
                        if let Err(e) = self.wechat_saver_db.save_user_info(&user_info).await {
                            println!("save user info error: {:?}", e);
                        }
                    }
                }
                Err(_e) => {
                    break;
                }
            }
            offset += limit;
        }
        Ok(())
    }

    #[allow(dead_code)]
    fn arch_single_voice(&self,voice_file_path:&str) -> Result<PathBuf> {
        let amr_file_path = &self.account_info.voice_path.parent().unwrap().join(voice_file_path);
        if !amr_file_path.exists(){
            return Err(Error::new(std::io::ErrorKind::NotFound,format!("amr file not found: {:?}",amr_file_path)));
        }
        let mp3_file_path = wechat_voice_decode(amr_file_path)?;
        let dst_mp3_relative_path = change_file_extension(voice_file_path.as_ref(), "mp3");
        let dst_path = &self.dest_path.join(&dst_mp3_relative_path);
        if !dst_path.parent().unwrap().exists(){
            fs::create_dir_all(dst_path.parent().unwrap())?;
        }
        if mp3_file_path.exists() {
            fs::copy(mp3_file_path, dst_path)?;
        }
        Ok(dst_mp3_relative_path)
    }

    fn arch_voice(&self) -> Result<()> {
        let src_path = Path::new(&self.account_info.voice_path);
        let dst_path = &self.dest_path.join("voice2");
        self.copy_dir_all(src_path, dst_path)?;

        let sd_card_voice_path = Path::new(&self.account_info.sd_card_voice_path);
        self.copy_dir_all(sd_card_voice_path, dst_path)?;

        convert_all_voice_to_mp3(dst_path)?;
        Ok(())
    }

    fn arch_image(&self) -> Result<()> {
        let src_path = Path::new(&self.account_info.image_path);
        let dst_path = &self.dest_path.join("image2");
        self.copy_dir_all(src_path, dst_path)?;
        Ok(())
    }

    fn arch_avatar(&self) -> Result<()> {
        let src_path = Path::new(&self.account_info.avatar_path);
        let dst_path = &self.dest_path.join("avatar");
        self.copy_dir_all(src_path, dst_path)?;
        Ok(())
    }

    fn arch_video(&self) -> Result<()> {
        let src_path = Path::new(&self.account_info.video_path);
        let dst_path = &self.dest_path.join("video");
        self.copy_dir_all(src_path, dst_path)?;

        let sd_card_video_path = Path::new(&self.account_info.sd_card_video_path);
        self.copy_dir_all(sd_card_video_path, dst_path)?;
        Ok(())
    }

    fn arch_openapi(&self) -> Result<()> {
        let src_path = Path::new(&self.account_info.openapi_path);
        let dst_path = &self.dest_path.join("openapi");
        self.copy_dir_all(src_path, dst_path)?;
        Ok(())
    }

    fn arch_attachment(&self) -> Result<()> {
        let src_path = Path::new(&self.account_info.attachment_path);
        let dst_path = &self.dest_path.join("attachment");
        self.copy_dir_all(src_path, dst_path)?;
        Ok(())
    }

    fn copy_dir_all(&self, src: &Path, dst: &Path) -> Result<()> {
        if !dst.exists() {
            fs::create_dir_all(dst)?;
        }
        if !src.exists() {
            return Ok(());
        }
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if file_type.is_dir() {
                self.copy_dir_all(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }
        Ok(())
    }
}

async fn arch_db_wx_file_index_by_msg_id(
    old_msg_id:i64,
    new_msg_id:i64,
    download_path:&Path,
    dest_path:&Path,
    db_conn: &WechatDB,
    wechat_saver_db: &WeChatSaverDB
) -> Result<()> {
    if let Ok(old_wx_file_index_opt) = db_conn.select_wx_file_index_by_msg_id(old_msg_id).await{
        if let Some(old_wx_file_index) = old_wx_file_index_opt {
            let mut new_wx_file_index = old_wx_file_index.clone();
            new_wx_file_index.msg_id = new_msg_id;

            match get_file_dir_name(&old_wx_file_index.path) {
                None => {
                    new_wx_file_index.path = get_after_double_slash(&old_wx_file_index.path).unwrap().to_string();
                }
                Some(name) => {
                    match name {
                        FileDirName::Download => {
                            arch_download(get_file_name(&old_wx_file_index.path).unwrap(),download_path,dest_path)?;
                            new_wx_file_index.path = get_after_double_slash(&old_wx_file_index.path).unwrap().to_string();
                        },
                        FileDirName::Voice2 => {
                            let temp_path = Path::new(get_after_double_slash(&old_wx_file_index.path).unwrap());
                            new_wx_file_index.path = change_file_extension(temp_path, "mp3").to_str().unwrap().to_string();
                        }
                    }
                }
            }
            if let Err(e) = wechat_saver_db.save_wx_file_index(&new_wx_file_index).await{
                println!("save wx file index error: {:?}", e);
            }
        }
    }
    Ok(())
}

fn arch_download(file_name: &str,download_path:&Path, dest_path:&Path) -> Result<()> {
    // let file_path = &self.account_info.download_path.join(file_name);
    // let dst_path = &self.dest_path.join("Download");
    let file_path = download_path.join(file_name);
    let dst_path = dest_path.join("Download");
    if !dst_path.exists(){
        fs::create_dir_all(&dst_path)?;
    }
    let dst_path = dst_path.join(file_name);
    if file_path.exists() {
        fs::copy(file_path, dst_path)?;
    }
    Ok(())
}