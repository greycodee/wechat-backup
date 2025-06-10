use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FilePath {
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
}

pub trait FilePathInitializer {
    fn get_sys_dir_path(&self, base_path: &Path, uin: &str) -> PathBuf;
    fn get_sd_card_dir_path(&self, base_path: &Path, uin: &str) -> PathBuf;
}

impl FilePath {

    pub fn new() -> Self {
        FilePath {
            video_path: PathBuf::new(),
            voice_path: PathBuf::new(),
            sd_card_video_path: PathBuf::new(),
            sd_card_voice_path: PathBuf::new(),
            image_path: PathBuf::new(),
            avatar_path: PathBuf::new(),
            download_path: PathBuf::new(),
            attachment_path: PathBuf::new(),
            openapi_path: PathBuf::new(),
            en_micro_msg_db_path: PathBuf::new(),
            wx_file_index_db_path: PathBuf::new(),
        }
    }

    pub fn init_with_initializer<T: FilePathInitializer>(
        &mut self,
        initializer: &T,
        base_path: &Path,
        uin: &str,
    ) {
        let sys_dir_path = initializer.get_sys_dir_path(base_path, uin);
        let sd_card_dir_path = initializer.get_sd_card_dir_path(base_path, uin);

        self.get_video_dir_path(&sys_dir_path);
        self.get_voice_dir_path(&sys_dir_path);
        self.get_sd_card_video_dir_path(&sd_card_dir_path);
        self.get_sd_card_voice_dir_path(&sd_card_dir_path);
        self.get_image_dir_path(&sys_dir_path);
        self.get_avatar_dir_path(&sys_dir_path);
        self.get_download_dir_path(&sd_card_dir_path);
        self.get_attachment_dir_path(&sys_dir_path);
        self.get_openapi_dir_path(&sd_card_dir_path);
        self.get_en_micro_msg_db_path(&sys_dir_path);
        self.get_wx_file_index_db_path(&sys_dir_path);
    }

    fn get_video_dir_path(&mut self,parent_path: &Path) {
        self.video_path = parent_path.join("video")
    }

    fn get_voice_dir_path(&mut self,parent_path: &Path) {
        self.voice_path = parent_path.join("voice2")
    }

    fn get_sd_card_video_dir_path(&mut self,parent_path: &Path) {
        self.sd_card_video_path = parent_path.join("video")
    }

    fn get_sd_card_voice_dir_path(&mut self,parent_path: &Path) {
        self.sd_card_voice_path = parent_path.join("voice2")
    }

    fn get_image_dir_path(&mut self,parent_path: &Path) {
        self.image_path = parent_path.join("image2")
    }

    fn get_avatar_dir_path(&mut self,parent_path: &Path) {
        self.avatar_path = parent_path.join("avatar")
    }

    fn get_download_dir_path(&mut self,parent_path: &Path) {
        self.download_path = parent_path.join("Download")
    }

    fn get_attachment_dir_path(&mut self,parent_path: &Path) {
        self.attachment_path = parent_path.join("attachment")
    }

    fn get_openapi_dir_path(&mut self,parent_path: &Path) {
        self.openapi_path = parent_path.join("openapi")
    }

    fn get_en_micro_msg_db_path(&mut self,parent_path: &Path) {
        self.en_micro_msg_db_path = parent_path.join("EnMicroMsg.db")
    }

    fn get_wx_file_index_db_path(&mut self,parent_path: &Path) {
        self.wx_file_index_db_path = parent_path.join("WxFileIndex.db")
    }
}