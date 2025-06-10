use std::io::Error;
use std::path::{Path, PathBuf};
use tokio::join;
use tokio::sync::mpsc::Sender;

pub mod account;
mod android_backup;
mod databases;
mod file_arch;
// mod utils;
mod wx_file_index;
mod message_parse;
use crate::miui_backup::android_backup::file::{unpack_android_backup, unpack_zip_file};
use account::AccountInfo;

use crate::common::utils::parse_all_uin;

/**
@param work_space: 这个项目的工作空间
@param android_backup_file: 微信备份文件的路径，一般以 .bak 或者 .db 为后缀的文件
@param android_sdcard_file: 媒体数据的备份压缩包，一般是一个 zip 文件
@return: 返回一个临时文件夹的路径（临时工作空间）
*/
pub async  fn process_backup_file(
    work_space: &Path,
    android_backup_file: &Path,
    android_sdcard_file: &Path,
) -> std::io::Result<PathBuf> {
    // 判断 android_backup_file 和 android_sdcard 是否存在
    if !android_backup_file.exists() {
        panic!("android_backup_file not exists");
    }
    if !android_sdcard_file.exists() {
        panic!("android_sdcard not exists");
    }
    // 判断work_space是否存在，不存在则创建
    if !work_space.exists() {
        std::fs::create_dir_all(work_space)?;
    }

    // 判断work_space 是否存在lock文件，存在则退出
    let lock_file = work_space.join("lock");
    if lock_file.exists() {
        let temp_dir_name = std::fs::read_to_string(&lock_file)?;
        let temp_dir = work_space.join(&temp_dir_name);
        return Ok(temp_dir);
    }

    let temp_dir_name = uuid::Uuid::new_v4().to_string();
    let temp_dir = work_space.join(&temp_dir_name);
    if !temp_dir.exists() {
        std::fs::create_dir_all(&temp_dir)?;
    }

    // 创建lock文件
    std::fs::File::create(&lock_file)?;
    std::fs::write(&lock_file, &temp_dir_name)?;

    let temp_dir_1 = temp_dir.clone();
    let temp_dir_2 = temp_dir.clone();

    let android_backup_file = android_backup_file.to_path_buf();
    let android_sdcard_file = android_sdcard_file.to_path_buf();

    let unpack_android_backup_task = tokio::spawn(async move {
        let res = unpack_android_backup(&android_backup_file, &temp_dir_1);
        match res {
            Ok(_) => {
                println!("unpack_android_backup success");
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    });

    let unpack_zip_file_task = tokio::spawn(async move {
        let res = unpack_zip_file(&android_sdcard_file, &temp_dir_2);
        match res {
            Ok(_) => {
                println!("unpack_zip_file success");
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    });

    match join!(unpack_android_backup_task, unpack_zip_file_task){
        (Ok(_), Ok(_)) => {
            println!("unpack_android_backup and unpack_zip_file success");
            Ok(temp_dir)
        }
        (Err(e1), Err(e2)) => {
            Err(Error::new(std::io::ErrorKind::Other, format!("unpack_android_backup error: {}, unpack_zip_file error: {}", e1, e2)))
        }
        (Err(e), _) => {
            Err(Error::new(std::io::ErrorKind::Other, format!("unpack_android_backup error: {}", e)))
        }
        (_, Err(e)) => {
            Err(Error::new(std::io::ErrorKind::Other, format!("unpack_android_backup error: {}", e)))
        }
    }

}



// TODO 从外部传入channel
pub async fn run(work_space: &Path,
                 android_backup_file: &Path,
                 android_sdcard_file: &Path,
                 tx: Sender<String>) -> std::io::Result<()> {
    // TODO 定义个全局任务 ID

    let temp_dir = process_backup_file(
        work_space,
        android_backup_file,
        android_sdcard_file).await?;

    if let Ok(account_vec) = get_all_account(&temp_dir).await{
        for account in account_vec {
            if let Ok(mut file_arch) = file_arch::FileArch::new(work_space, account){
                if let Err(e) = file_arch.arch_all().await{
                    println!("arch_all error: {:?}",e);
                }
            }
        }
        // TODO 完成，更新全局任务ID状态
        // TODO 删除临时文件夹
        // TODO 删除 lock 文件
        tx.send("run success".to_string()).await.unwrap();
    }else{
        // TODO 失败，更新全局任务ID状态
        tx.send("run error".to_string()).await.unwrap();
    }
    Ok(())
}


pub async fn get_all_account(base_path: &Path) -> std::io::Result<Vec<AccountInfo>> {
    let uin_vec = get_all_uin(base_path);
    let mut account_vec = Vec::new();
    for uin in uin_vec {
        let account_info = AccountInfo::new(base_path, &uin).await?;
        account_vec.push(account_info);
    }
    Ok(account_vec)
}

fn get_all_uin(base_path: &Path) -> Vec<String> {
    let uin_file_path = base_path.join("apps/com.tencent.mm/sp/app_brand_global_sp.xml");
    parse_all_uin(&uin_file_path)
}

#[cfg(test)]
mod test {
    use std::env;
    use std::path::Path;

    use super::*;

    #[test]
    fn test_get_all_uin() {
        dotenv::dotenv().ok();
        let temp_path = env::var("TEMP_PATH").unwrap();
        let base_path = Path::new(temp_path.as_str());
        let res = get_all_uin(base_path);
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_get_all_account() {
        dotenv::dotenv().ok();
        let temp_path = env::var("TEMP_PATH").unwrap();
        let base_path = Path::new(temp_path.as_str());
        let res = get_all_account(base_path).await;
        println!("{:?}", res);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 6)]
    async fn test_arch_all() {
        dotenv::dotenv().ok();
        let temp_path = env::var("TEMP_PATH").unwrap();
        let work_space = env::var("WORK_SPACE_PATH").unwrap();

        let work_space = Path::new(&work_space);

        let res = get_all_account(Path::new(&temp_path)).await;
        for account in res.unwrap() {
            let file_arch = file_arch::FileArch::new(work_space, account);
            file_arch.unwrap().arch_all().await.unwrap();
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 6)]
    async  fn test_run() {
        dotenv::dotenv().ok();
        // let temp_path = env::var("TEMP_PATH").unwrap();
        let work_space = env::var("WORK_SPACE_PATH").unwrap();
        let android_backup_file = env::var("ANDROID_BACKUP_FILE").unwrap();
        let android_sdcard_file = env::var("ANDROID_SDCARD_ZIP_FILE").unwrap();


        let (tx, mut rx) = tokio::sync::mpsc::channel(100);

        let run_task = tokio::spawn(async move {
            run(Path::new(&work_space), Path::new(&android_backup_file), Path::new(&android_sdcard_file), tx).await.unwrap();
        });

        let process_messages_task = tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                println!("Received message: {}", message);
            }
        });

        let _ = join!(run_task, process_messages_task);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_process_backup_file() {
        dotenv::dotenv().ok();
        let work_space = env::var("WORK_SPACE_PATH").unwrap();
        let android_backup_file = env::var("ANDROID_BACKUP_FILE").unwrap();
        let android_sdcard_file = env::var("ANDROID_SDCARD_ZIP_FILE").unwrap();

        let res = process_backup_file(Path::new(&work_space), Path::new(&android_backup_file), Path::new(&android_sdcard_file)).await;
        println!("{:?}", res.unwrap());
    }
}
