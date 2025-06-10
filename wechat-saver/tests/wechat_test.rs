mod common;


#[test]
fn test_dotenv(){
    common::setup();
    let version = std::env::var("WECHAT_VERSION").unwrap();
    assert_eq!(version, "1.0.0");
}