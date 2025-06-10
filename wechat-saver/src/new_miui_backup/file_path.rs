use crate::common::{utils, file_path_base::{FilePath, FilePathInitializer}};
use std::path::{Path, PathBuf};

pub struct MiuiFilePathInitializer;

impl FilePathInitializer for MiuiFilePathInitializer {
    fn get_sys_dir_path(&self, base_path: &Path, uin: &str) -> PathBuf {
        let sys_dir_name = utils::get_system_file_name(uin);
        base_path
            .join("apps/com.tencent.mm/r/MicroMsg")
            .join(sys_dir_name)
    }

    fn get_sd_card_dir_path(&self, base_path: &Path, uin: &str) -> PathBuf {
        let sd_card_dir_name = utils::get_miui_backup_sd_card_dir_name(base_path, uin).unwrap();
        base_path
            .join("Android/data/com.tencent.mm/MicroMsg")
            .join(sd_card_dir_name)
    }
}

pub fn new_file_path(base_path: &Path, uin: &str) -> FilePath {
    let mut file_path = FilePath::new();
    file_path.init_with_initializer(&MiuiFilePathInitializer, base_path, uin);
    file_path
}