use std::path::{Path, PathBuf};
use md5::{Digest, Md5};
use quick_xml::events::Event;
use quick_xml::Reader;

/**
    生成数据库私钥

    @param uin: 微信账号的uin
*/
pub fn gen_db_private_key(uin: &str) -> String {
    let mut private_key = String::from("1234567890ABCDEF");
    private_key.push_str(uin);
    let md5_private_key = md5_encode(&private_key);
    md5_private_key[0..7].to_string()
}

/**
    md5加密

    @param input: 输入字符串
*/
pub fn md5_encode(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input);
    let result = hasher.finalize();
    let result = hex::encode(result);
    result
}

/**
    修改文件扩展名

    @param file_path: 文件路径
    @param extension: 新的扩展名
*/
pub fn change_file_extension(file_path: &Path,extension: &str) -> PathBuf {
    let mut new_path = file_path.to_path_buf();
    new_path.set_extension(extension);
    new_path
}

/**
    获取所有的uin

    @param base_path: app_brand_global_sp.xml 路径
*/
pub fn parse_all_uin(xml_path: &Path) -> Vec<String> {
    // 判断文件是否存在
    if !xml_path.exists() {
        return Vec::new();
    }
    let mut uin_vec = Vec::new();
    let mut reader = Reader::from_file(xml_path).unwrap();
    reader.config_mut().trim_text(true);
    loop {
        match reader.read_event_into(&mut Vec::new()) {
            Ok(Event::Text(e)) => match String::from_utf8(e.into_inner().into_owned()) {
                Ok(uin) => {
                    uin_vec.push(uin);
                }
                Err(e) => {
                    panic!("Error: {:?}", e);
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }
    uin_vec
}

/**
    获取系统文件名

    @param uin: 微信账号的uin
*/
pub fn get_system_file_name(uin: &str) -> String {
    let mut private_key = String::from("mm");
    private_key.push_str(uin);
    md5_encode(&private_key)
}

/**
    获取 [miui] sd卡目录名

    @param base_path: 微信根目录
    @param uin: 微信账号的uin
*/
pub fn get_miui_backup_sd_card_dir_name(base_path: &Path, uin: &str) -> std::io::Result<String> {
    let account_dir_name = get_system_file_name(uin);
    let account_mapping_file_path = base_path
        .join("apps/com.tencent.mm/r/MicroMsg")
        .join(account_dir_name)
        .join("account.mapping");

    let account_mapping_file = std::fs::read_to_string(account_mapping_file_path)?;

    Ok(account_mapping_file)
}

/**
    获取sd卡目录名

    @param base_path: 微信根目录
    @param uin: 微信账号的uin
*/
pub fn get_noe_backup_sd_card_dir_name(base_path: &Path, uin: &str) -> std::io::Result<String> {
    let account_dir_name = get_system_file_name(uin);
    let account_mapping_file_path = base_path
        .join("MicroMsg")
        .join(account_dir_name)
        .join("account.mapping");

    let account_mapping_file = std::fs::read_to_string(account_mapping_file_path)?;

    Ok(account_mapping_file)
}

/**
    获取头像路径

    @param wx_id: 微信id
*/
pub fn get_avatar_path(wx_id: &str) -> PathBuf {
    let md5_wx_id = md5_encode(wx_id);
    let avatar_file_name = format!("user_{}.png", md5_wx_id);
    let avatar_pre_dir_path = format!("{}/{}", &md5_wx_id[0..2], &md5_wx_id[2..4]);
    let avatar_path = PathBuf::from(avatar_pre_dir_path).join(avatar_file_name);
    avatar_path
}

pub fn get_avatar_file_name(wx_id: &str) -> String {
    let md5_wx_id = md5_encode(wx_id);
    let avatar_file_name = format!("user_{}.png", md5_wx_id);
    avatar_file_name
}


/**
    获取高清头像路径

    @param wx_id: 微信id
*/
pub fn get_hd_avatar_path(wx_id: &str) -> PathBuf {
    let md5_wx_id = md5_encode(wx_id);
    let avatar_file_name = format!("user_hd_{}.png", md5_wx_id);
    let avatar_pre_dir_path = format!("{}/{}", &md5_wx_id[0..2], &md5_wx_id[2..4]);
    let avatar_path = PathBuf::from(avatar_pre_dir_path).join(avatar_file_name);
    avatar_path
}

pub fn get_hd_avatar_file_name(wx_id: &str) -> String {
    let md5_wx_id = md5_encode(wx_id);
    let avatar_file_name = format!("user_hd_{}.png", md5_wx_id);
    avatar_file_name
}

#[allow(dead_code)]
pub fn get_first_value_after_double_slash(input: &str) -> Option<&str> {
    if let Some(start) = input.find("//") {
        let rest = &input[start + 2..];
        if let Some(end) = rest.find('/') {
            return Some(&rest[..end]);
        }
    }
    None
}

pub fn get_after_double_slash(input: &str) -> Option<&str> {
    if let Some(start) = input.find("//") {
        let rest = &input[start + 2..];
        return Some(rest);
    }
    None
}

// 提供: wcf://image2/d7/0c/th_d70cd0752c8e5042c86de60349dd6b2b
// 返回: /d7/0c/th_d70cd0752c8e5042c86de60349dd6b2b
pub fn get_last_full_path(input: &str) -> Option<&str> {
    if let Some(start) = input.find("//") {
        let rest = &input[start + 2..];
        if let Some(end) = rest.find('/') {
            return Some(&rest[end + 1..]);
        }
    }
    None
}

pub fn get_file_name(input: &str) -> Option<&str> {
    if let Some(start) = input.rfind('/') {
        return Some(&input[start + 1..]);
    }
    None
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gen_db_private_key() {
        let uin = "1727242265";
        let key = gen_db_private_key(uin);
        println!("key: {}", key);
    }

    #[test]
    fn test_md5_encode() {
        let input = "123123";
        let res = md5_encode(input);
        println!("{:?}", res);
    }

    #[test]
    fn test_change_file_extension() {
        let file_path = std::path::Path::new("/tmp/test.txt");
        let res = change_file_extension(file_path, "mp3");
        println!("{:?}", res);
    }

    fn test_get_first_value_after_double_slash() {
        let input = "wcf://attachment/clash_for_android.apk";
        let res = get_first_value_after_double_slash(input);
        assert_eq!(res, Some("attachment"));
    }

    #[test]
    fn test_get_first_value_after_double_slash_1() {
        let input = "wcf://Download/test.docx";
        let res = get_first_value_after_double_slash(input);
        assert_eq!(res, Some("Download"));
    }

    #[test]
    fn test_get_after_double_slash(){
        let input = "wcf://Download/test.docx";
        let res = get_after_double_slash(input);
        assert_eq!(res, Some("Download/test.docx"));
    }

    #[test]
    fn test_get_after_double_slash_2(){
        let input = "wcf://image2/d7/0c/th_d70cd0752c8e5042c86de60349dd6b2b";
        let res = get_after_double_slash(input);
        assert_eq!(res, Some("image2/d7/0c/th_d70cd0752c8e5042c86de60349dd6b2b"));
    }

    #[test]
    fn test_get_file_name(){
        let input = "wcf://Download/test.docx";
        let res = get_file_name(input);
        assert_eq!(res, Some("test.docx"));
    }

    #[test]
    fn test_get_last_full_path(){
        let input = "wcf://image2/d7/0c/th_d70cd0752c8e5042c86de60349dd6b2b";
        let res = get_last_full_path(input);
        assert_eq!(res, Some("/d7/0c/th_d70cd0752c8e5042c86de60349dd6b2b"));
    }

    #[test]
    fn test_get_last_full_path2(){
        let input = "wcf://Download/test.docx";
        let res = get_last_full_path(input);
        assert_eq!(res, Some("/test.docx"));
    }
}
