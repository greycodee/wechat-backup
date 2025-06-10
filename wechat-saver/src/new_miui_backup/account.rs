use std::path::Path;
use anyhow::Result;
use crate::common::account_base::{AccountInfoBase, AccountInitializer};
use crate::common::model::LatestUser;
use crate::new_miui_backup::preferences::parse_mm_preferences;

#[derive(Default)]
pub struct MiuiAccountInitializer;

impl AccountInitializer for MiuiAccountInitializer {
    fn parse_mm_preferences(base_path: &Path) -> Result<LatestUser> {
        parse_mm_preferences(base_path)
    }
}

pub type AccountInfo = AccountInfoBase;

#[cfg(test)]
mod test {
    use super::*;
    
    // ... 测试代码 ...
} 