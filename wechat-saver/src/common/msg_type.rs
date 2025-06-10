
// [
// {
// "msgType": 3
// },
// {
// "msgType": 34
// },
// {
// "msgType": 43
// },
// {
// "msgType": 49
// },
// {
// "msgType": 1048625
// },
// {
// "msgType": 436207665
// },
// {
// "msgType": 1040187441
// },
// {
// "msgType": 1090519089
// }
// ]

pub enum MsgType {
    File,
    Voice,
    Video,
    Image,
}

impl MsgType {
    pub fn from(msg_type: i64) -> Option<Self> {
        match msg_type {
            3 => Some(MsgType::Image),
            34 => Some(MsgType::Voice),
            43 => Some(MsgType::Video),
            1048625 => Some(MsgType::Image),
            1090519089 => Some(MsgType::File),
            _ => None,
        }
    }

    pub fn from_to_string(msg_type: i64) -> Option<String> {
        match msg_type {
            3 => Some("Image".to_string()),
            34 => Some("Voice".to_string()),
            43 => Some("Video".to_string()),
            1048625 => Some("Image".to_string()),
            1090519089 => Some("File".to_string()),
            _ => None,
        }
    }

}

pub fn is_file(msg_type: i64) -> bool {
    match msg_type {
        3 | 34 | 43 | 1048625 | 1090519089 => true,
        _ => false,
    }
}