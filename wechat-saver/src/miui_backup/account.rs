use super::databases::wechat_db::WechatDB;
use std::io::Error;
use std::path::{Path, PathBuf};
use crate::common::utils;

#[allow(dead_code)]
#[derive(Debug)]
pub struct WXUserInfo {
    pub wx_id: String,
    pub wx_account_no: String,
    pub account_name: String,
    pub account_phone: String,
    pub account_avatar_path: Option<PathBuf>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct AccountInfo {
    pub wx_user_info: WXUserInfo,
    pub account_uin: String,
    pub video_path: PathBuf,
    pub voice_path: PathBuf,
    pub sd_card_video_path: PathBuf,
    pub sd_card_voice_path: PathBuf,
    pub image_path: PathBuf,
    pub avatar_path: PathBuf,
    pub download_path: PathBuf,
    pub attachment_path: PathBuf,
    pub openapi_path: PathBuf,
    pub en_micro_msg_db_path: PathBuf,
    pub wx_file_index_db_path: PathBuf,
    pub db_private_key: String,
    pub db_conn: WechatDB,
}

impl AccountInfo {
    /**
        @param base_path: temp dir path
    */
    pub async fn new(base_path: &Path, uin: &str) -> std::io::Result<Self> {
        let account_dir_name = utils::get_system_file_name(uin);
        let account_file_path = base_path
            .join("apps/com.tencent.mm/r/MicroMsg")
            .join(account_dir_name);

        let attachment_path = account_file_path.join("attachment");
        let image_path = account_file_path.join("image2");
        let avatar_path = account_file_path.join("avatar");
        let video_path = account_file_path.join("video");
        let voice_path = account_file_path.join("voice2");
        let en_micro_msg_db_path = account_file_path.join("EnMicroMsg.db");
        let wx_file_index_db_path = account_file_path.join("WxFileIndex.db");

        let sd_card_dir_path = base_path
            .join("Android/data/com.tencent.mm/MicroMsg");
        // TODO 最新版微信都迁移到了系统attachment目录下
        let download_path = sd_card_dir_path.join("Download");

        let account_sd_card_dir_name = utils::get_miui_backup_sd_card_dir_name(base_path, uin)?;
        let account_sd_card_dir_path = sd_card_dir_path.join(account_sd_card_dir_name);

        // TODO 最新版微信都迁移到了系统app目录下
        let sd_card_video_path = account_sd_card_dir_path.join("video");
        let sd_card_voice_path = account_sd_card_dir_path.join("voice2");
        let openapi_path = account_sd_card_dir_path.join("openapi");


        let db_private_key = utils::gen_db_private_key(uin);
        let db_conn;
        match WechatDB::new(
            &en_micro_msg_db_path,
            &wx_file_index_db_path,
            &db_private_key,
        ) {
            Ok(w) => {
                db_conn = w;
            }
            Err(err) => {
                return Err(Error::new(std::io::ErrorKind::Other, err));
            }
        }

        match db_conn.get_wx_user_info().await {
            Ok(wx_user_info) => Ok(AccountInfo {
                wx_user_info,
                account_uin: uin.to_string(),
                video_path,
                voice_path,
                sd_card_video_path,
                sd_card_voice_path,
                image_path,
                avatar_path,
                download_path,
                attachment_path,
                openapi_path,
                en_micro_msg_db_path,
                wx_file_index_db_path,
                db_private_key,
                db_conn,
            }),
            Err(err) => Err(Error::new(std::io::ErrorKind::Other, err)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_account_info() {
        let uin = "1727242265";
        let base_path = Path::new("/Users/zheng/Downloads/20241024_091952");
        let account_info = AccountInfo::new(&base_path, uin).await;
        println!("{:?}", account_info);
    }
}
