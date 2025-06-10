use std::path::{Path, PathBuf};
use crate::common::utils::parse_all_uin;

pub fn unpack_backup_file(workspace: &Path, data_tar_gz_file: &Path, external_tar_gz_file: &Path) -> std::io::Result<PathBuf> {
    // 查看是否已经解压过： 查看workspace下是否有lock文件
    let lock_file = workspace.join("lock");
    if lock_file.exists() {
        // 查看lock内容，返回里面的临时目录名称
        let temp_dir_name = std::fs::read_to_string(&lock_file)?;
        let temp_dir = workspace.join(&temp_dir_name);
        return Ok(temp_dir);
    }
    let temp_dir_name = uuid::Uuid::new_v4().to_string();
    let temp_dir = workspace.join(&temp_dir_name);
    if !temp_dir.exists() {
        std::fs::create_dir_all(&temp_dir)?;
    }
    unpack_data_tar_gz(data_tar_gz_file, &temp_dir)?;
    unpack_external_tar_gz(external_tar_gz_file, &temp_dir)?;

    // 创建lock文件
    std::fs::write(&lock_file, temp_dir_name)?;

    Ok(temp_dir)
}

fn unpack_data_tar_gz(tar_gz_file: &Path, output_dir: &Path) -> std::io::Result<()> {
    unpack_tar_gz(tar_gz_file, output_dir)
}

fn unpack_external_tar_gz(tar_gz_file: &Path, output_dir: &Path) -> std::io::Result<()> {
    unpack_tar_gz(tar_gz_file, output_dir)
}

fn unpack_tar_gz(tar_gz_file: &Path, output_dir: &Path) -> std::io::Result<()> {
    let tar_gz_file = std::fs::File::open(tar_gz_file)?;
    let tar = flate2::read::GzDecoder::new(tar_gz_file);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(output_dir)?;
    Ok(())
}

pub fn get_all_uin(base_path: &Path) -> Vec<String> {
    let uin_file_path = base_path.join("shared_prefs/app_brand_global_sp.xml");
    parse_all_uin(&uin_file_path)
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_unpack_backup_file() {
        let workspace = Path::new("/Users/zheng/Downloads/wechat-backup/workspace2");
        let data_tar_gz_file = Path::new("/Users/zheng/Downloads/wechat-backup/data.tar.gz");
        let external_tar_gz_file = Path::new("/Users/zheng/Downloads/wechat-backup/external_files.tar.gz");
        let res = unpack_backup_file(workspace, data_tar_gz_file, external_tar_gz_file);
        println!("{:?}", res);
        // assert!(res.unwrap());
    }

    #[test]
    fn test_get_all_uin() {
        let all_uin = get_all_uin(Path::new("/Volumes/hkdisk/neo-backup-dagongren/workspace/1c247e87-95e9-4f4f-9e41-bf5a2f40ad05"));
        println!("{:?}", all_uin);
    }
}