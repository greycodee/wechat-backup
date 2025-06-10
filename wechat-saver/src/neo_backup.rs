mod unpack_file;
mod account;
mod file_path;
mod preferences;
mod sync_cloud;
// 1. 解压备份文件 data.tar.gz 和 external_sdcard.tar.gz. 获取解压到的临时文件夹路径 [unpack_file.rs]
// 2. 获取 uid 列表 [unpack_file.rs -> get_all_uin]
// 3. 通过 uid 获取账号的文件存放路径 [account.rs]
// 4. 获取数据库密钥，解密数据库，获取账号信息
// 5. 备份操作 [sync_cloud.rs]

use anyhow::Result;
use crate::common::account_base::AccountInfoBase;
use crate::neo_backup::account::NeoAccountInitializer;
use crate::neo_backup::file_path::NeoFilePathInitializer;

pub async fn quick_backup(workspace: &str, data_tar_gz_file: &str, external_tar_gz_file: &str, endpoint: &str) -> Result<()> {
    let workspace = std::path::Path::new(workspace);
    let data_tar_gz_file = std::path::Path::new(data_tar_gz_file);
    let external_tar_gz_file = std::path::Path::new(external_tar_gz_file);
    let temp_path = unpack_file::unpack_backup_file(workspace, data_tar_gz_file, external_tar_gz_file)?;
    println!("temp_path: {:?}", temp_path);
    let account_info = AccountInfoBase::new::<NeoAccountInitializer>(&temp_path, &NeoFilePathInitializer)?;
    let sync_cloud = sync_cloud::SyncCloud::new(account_info, endpoint);
    sync_cloud.sync_all().await;
    Ok(())
}

#[cfg(test)]
mod test{
    use std::path::Path;
    use crate::common::account_base::AccountInfoBase;
    use crate::neo_backup::{sync_cloud, unpack_file};
    use crate::neo_backup::account::NeoAccountInitializer;
    use crate::neo_backup::file_path::NeoFilePathInitializer;

    #[tokio::test]
    async fn test_sync_cloud() {
        let temp_path = Path::new("/Volumes/hkdisk/neo-backup/workspace/b68b3737-ae18-494a-b5f5-9903f8628d90");
        let binding = unpack_file::get_all_uin(temp_path);
        let uin = binding.get(0).unwrap();
        let account_info = AccountInfoBase::new::<NeoAccountInitializer>(temp_path, &NeoFilePathInitializer).unwrap();
        let endpoint = "http://192.168.6.115:9012";
        let sync_cloud = sync_cloud::SyncCloud::new(account_info,endpoint);
        let res = sync_cloud.sync_user_info().await;
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_sync_message() {
        let temp_path = Path::new("/Volumes/hkdisk/neo-backup-dagongren/workspace/1c247e87-95e9-4f4f-9e41-bf5a2f40ad05");
        let account_info = AccountInfoBase::new::<NeoAccountInitializer>(temp_path, &NeoFilePathInitializer).unwrap();
        let endpoint = "http://192.168.6.115:9012";
        let sync_cloud = sync_cloud::SyncCloud::new(account_info,endpoint);
        let res = sync_cloud.sync_message().await;
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_sync_contact() {
        let temp_path = Path::new("/Volumes/hkdisk/neo-backup-dagongren/workspace/1c247e87-95e9-4f4f-9e41-bf5a2f40ad05");
        let account_info = AccountInfoBase::new::<NeoAccountInitializer>(temp_path, &NeoFilePathInitializer).unwrap();
        let endpoint = "http://192.168.6.115:9012";
        let sync_cloud = sync_cloud::SyncCloud::new(account_info,endpoint);
        let res = sync_cloud.sync_contact().await;
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_quick_backup() {
        let workspace = "/Users/zheng/Downloads/wechat-backup/workspace2";
        let data_tar_gz_file = "/Users/zheng/Downloads/wechat-backup/data.tar.gz";
        let external_tar_gz_file = "/Users/zheng/Downloads/wechat-backup/external_files.tar.gz";
        let endpoint = "http://192.168.6.115:9012";
        let res = super::quick_backup(workspace, data_tar_gz_file, external_tar_gz_file, endpoint).await;
    }
}
