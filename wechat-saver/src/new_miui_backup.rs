mod account;
mod file_path;
mod preferences;
mod sync_cloud;
mod unpack_file;

use crate::common::account_base::AccountInfoBase;
use crate::new_miui_backup::account::MiuiAccountInitializer;
use crate::new_miui_backup::file_path::MiuiFilePathInitializer;

pub async fn quick_backup(workspace: &str, android_backup_file: &str, android_sdcard_file: &str, endpoint: &str) -> anyhow::Result<()> {
    let workspace = std::path::Path::new(workspace);
    let data_tar_gz_file = std::path::Path::new(android_backup_file);
    let external_tar_gz_file = std::path::Path::new(android_sdcard_file);

    let temp_path = unpack_file::unpack_backup_file(workspace, data_tar_gz_file, external_tar_gz_file).await?;
    println!("temp_path: {:?}", temp_path);
    let account_info = AccountInfoBase::new::<MiuiAccountInitializer>(&temp_path, &MiuiFilePathInitializer)?;
    println!("{:?}", account_info);
    let sync_cloud = sync_cloud::SyncCloud::new(account_info, endpoint);
    sync_cloud.sync_all().await;
    Ok(())
}

#[cfg(test)]
mod test{
    use crate::common::account_base::AccountInfoBase;
    use crate::new_miui_backup::account::MiuiAccountInitializer;
    use crate::new_miui_backup::file_path::MiuiFilePathInitializer;

    #[tokio::test]
    async fn test_account_info() {
        let temp_path = std::path::Path::new("/Volumes/hkdisk/wechat-backup/20241218z-111111-h/workspace/468ae5f2-9fb3-4485-bd40-6940fa9c6208");
        let account_info = AccountInfoBase::new::<MiuiAccountInitializer>(&temp_path, &MiuiFilePathInitializer).unwrap();
        println!("{:?}", account_info);
    }

    #[tokio::test]
    async fn test_quick_backup() {
        // let workspace = "/Volumes/hkdisk/wechat-backup/20241218z-111111-h/workspace";
        // let data_tar_gz_file = "/Volumes/hkdisk/wechat-backup/20241218z-111111-h/wechat.bak";
        // let external_tar_gz_file = "/Volumes/hkdisk/wechat-backup/20241218z-111111-h/backup_wechat.zip";

        let workspace = "/Volumes/hkdisk/wechat-backup/20241218z-111111-h/workspace";
        let data_tar_gz_file = "/Volumes/hkdisk/wechat-backup/20241218z-111111-h/wechat.bak";
        let external_tar_gz_file = "/Volumes/hkdisk/wechat-backup/20241218z-111111-h/backup_wechat.zip";

        let endpoint = "http://192.168.6.115:9012";
        let res = super::quick_backup(workspace, data_tar_gz_file, external_tar_gz_file, endpoint).await;
        println!("{:?}", res);
    }
}