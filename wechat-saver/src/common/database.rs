use std::path::Path;
use rusqlite::Connection;

pub fn open_wechat_db(db_path: &Path, pri_key: &str) -> rusqlite::Result<Connection> {
    let conn = Connection::open(db_path)?;
    conn.execute_batch(&format!("PRAGMA key = '{}';", pri_key))?;
    conn.execute_batch(&format!("PRAGMA cipher_use_hmac = {};", "off"))?;
    conn.execute_batch(&format!("PRAGMA kdf_iter = {};", 4000))?;
    conn.execute_batch(&format!("PRAGMA cipher_page_size = {};", 1024))?;
    conn.execute_batch(&format!("PRAGMA cipher_hmac_algorithm = {};", "HMAC_SHA1"))?;
    conn.execute_batch(&format!(
        "PRAGMA cipher_kdf_algorithm = {};",
        "PBKDF2_HMAC_SHA1"
    ))?;
    Ok(conn)
}

#[allow(dead_code)]
pub fn save_wechat_db_to_plan(db_path: &str, pri_key: &str) -> rusqlite::Result<String> {
    let conn = Connection::open(db_path)?;
    conn.execute_batch(&format!("PRAGMA key = '{}';", pri_key))?;
    conn.execute_batch(&format!("PRAGMA cipher_use_hmac = {};", "off"))?;
    conn.execute_batch(&format!("PRAGMA kdf_iter = {};", 4000))?;
    conn.execute_batch(&format!("PRAGMA cipher_page_size = {};", 1024))?;
    conn.execute_batch(&format!("PRAGMA cipher_hmac_algorithm = {};", "HMAC_SHA1"))?;
    conn.execute_batch(&format!(
        "PRAGMA cipher_kdf_algorithm = {};",
        "PBKDF2_HMAC_SHA1"
    ))?;
    let dest_db_path = format!("{}.plan.db", db_path);
    conn.execute_batch(&format!("ATTACH DATABASE '{}' AS plan_db KEY '';",dest_db_path))?;
    conn.execute_batch("SELECT sqlcipher_export('plan_db');")?;
    conn.execute_batch("DETACH DATABASE plan_db;")?;

    Ok(dest_db_path)
}