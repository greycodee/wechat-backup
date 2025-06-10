use std::path::Path;
use anyhow::Result;
use crate::common::account_base::{AccountInfoBase, AccountInitializer};
use crate::common::model::LatestUser;
use crate::neo_backup::preferences::parse_mm_preferences;

#[derive(Default)]
pub struct NeoAccountInitializer;

impl AccountInitializer for NeoAccountInitializer {
    fn parse_mm_preferences(base_path: &Path) -> Result<LatestUser> {
        parse_mm_preferences(base_path)
    }
}

pub type AccountInfo = AccountInfoBase;

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_parse_mm_preferences() {
        let base_path = Path::new("/Users/zheng/Downloads/wechat-backup/workspace2/c1493bf8-af3a-41f8-80b3-6913a1431536");
        let res = NeoAccountInitializer::parse_mm_preferences(base_path);
        println!("{:?}", res);
    }
    
    #[test]
    fn test_account_info() {
        let temp_path = Path::new("/Users/zheng/Downloads/wechat-backup/workspace2/c1493bf8-af3a-41f8-80b3-6913a1431536");
        let account_info = AccountInfoBase::new::<NeoAccountInitializer>(temp_path, &crate::neo_backup::file_path::NeoFilePathInitializer).unwrap();
        println!("{:?}", account_info);
    }
} 