use std::path::{Path, PathBuf};
use anyhow::Result;
use crate::common::model::LatestUser;
use crate::common::preferences_base::{PreferencesParser, parse_mm_preferences as parse_preferences};

pub struct NeoPreferencesParser;

impl PreferencesParser for NeoPreferencesParser {
    fn get_preferences_path(base_path: &Path) -> PathBuf {
        base_path.join("shared_prefs/com.tencent.mm_preferences.xml")
    }
}

pub fn parse_mm_preferences(base_path: &Path) -> Result<LatestUser> {
    parse_preferences::<NeoPreferencesParser>(base_path)
}